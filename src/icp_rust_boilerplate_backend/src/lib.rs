#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Book {
    id: u64,
    title: String,
    author: String,
    published_year: u32,
    borrowed: bool,
    borrowed_by: Option<String>,
    borrowed_at: Option<u64>, // New field for storing borrowing timestamp
}

impl Storable for Book {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct BookPayload {
    title: String,
    author: String,
    published_year: u32,
}

#[ic_cdk::query]
fn get_book(id: u64) -> Result<Book, Error> {
    match _get_book(&id) {
        Some(book) => Ok(book),
        None => Err(Error::NotFound {
            msg: format!("a book with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_book(book: BookPayload) -> Result<Option<Book>, Error> {
    // Implement access control here (e.g., check if the caller is authorized)
    if !is_authorized_caller() {
        return Err(Error::UnauthorizedAccess {
            msg: "Unauthorized access to add_book".to_string(),
        });
    }

    // Validation: Check if the book title is not empty
    if book.title.is_empty() {
        return Err(Error::Validation {
            msg: "Book title cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let new_book = Book {
        id,
        title: book.title,
        author: book.author,
        published_year: book.published_year,
        borrowed: false,
        borrowed_by: None,
        borrowed_at: None,
    };

    do_insert_book(&new_book);
    Ok(Some(new_book))
}

#[ic_cdk::update]
fn borrow_book(id: u64, borrower: String) -> Result<Book, Error> {
    // Implement access control here (e.g., check if the caller is authorized)
    if !is_authorized_caller() {
        return Err(Error::UnauthorizedAccess {
            msg: "Unauthorized access to borrow_book".to_string(),
        });
    }

    // Validation: Check if the borrower name is not empty
    if borrower.is_empty() {
        return Err(Error::Validation {
            msg: "Borrower name cannot be empty".to_string(),
        });
    }

    match STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        if let Some(book) = storage.get(&id) {
            if book.borrowed {
                return Err(Error::BookAlreadyBorrowed {
                    msg: format!("book with id={} is already borrowed", id),
                });
            }

            let mut borrowed_book = book.clone();
            borrowed_book.borrowed = true;
            borrowed_book.borrowed_by = Some(borrower);
            borrowed_book.borrowed_at = Some(time()); // Record the borrowing timestamp

            // Remove the old book and insert the modified one
            storage.remove(&id);
            storage.insert(id, borrowed_book.clone());

            Ok(borrowed_book)
        } else {
            Err(Error::NotFound {
                msg: format!("couldn't borrow a book with id={}. book not found", id),
            })
        }
    }) {
        Ok(book) => Ok(book),
        Err(err) => Err(err),
    }
}

fn do_insert_book(book: &Book) {
    STORAGE.with(|service| service.borrow_mut().insert(book.id, book.clone()));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    BookAlreadyBorrowed { msg: String },
    UnauthorizedAccess { msg: String },
    BookNotBorrowed { msg: String },
    Validation { msg: String },  // Add this variant for validation errors
}

// Function to check if the caller is authorized (replace this with your actual logic)
fn is_authorized_caller() -> bool {
    // For example, check the caller's principal or other authentication details
    true // Replace with actual authorization logic
}

// a helper method to get a book by id. used in get_book/borrow_book
fn _get_book(id: &u64) -> Option<Book> {
    STORAGE.with(|service| service.borrow().get(id))
}

#[ic_cdk::query]
fn view_book(id: u64) -> Result<Book, Error> {
    get_book(id)
}

#[ic_cdk::query]
fn list_books() -> Vec<Book> {
    STORAGE.with(|service| {
        service.borrow().iter().map(|(_, book)| book.clone()).collect()
    })
}

// A function to check if a book is borrowed
#[ic_cdk::query]
fn is_book_borrowed(id: u64) -> bool {
    if let Some(book) = get_book(id).ok() {
        book.borrowed
    } else {
        false
    }
}

#[ic_cdk::update]
fn return_book(id: u64) -> Result<Book, Error> {
    // Implement access control here (e.g., check if the caller is authorized)
    if !is_authorized_caller() {
        return Err(Error::UnauthorizedAccess {
            msg: "Unauthorized access to return_book".to_string(),
        });
    }

    match STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        if let Some(book) = storage.get(&id) {
            if !book.borrowed {
                return Err(Error::BookNotBorrowed {
                    msg: format!("book with id={} is not currently borrowed", id),
                });
            }

            let mut returned_book = book.clone();
            returned_book.borrowed = false;
            returned_book.borrowed_by = None;
            returned_book.borrowed_at = None; // Reset the borrowing timestamp

            // Remove the old book and insert the modified one
            storage.remove(&id);
            storage.insert(id, returned_book.clone());

            Ok(returned_book)
        } else {
            Err(Error::NotFound {
                msg: format!("couldn't return a book with id={}. book not found", id),
            })
        }
    }) {
        Ok(book) => Ok(book),
        Err(err) => Err(err),
    }
}

#[ic_cdk::update]
fn update_book(id: u64, updated_book: BookPayload) -> Result<Book, Error> {
    // Implement access control here (e.g., check if the caller is authorized)
    if !is_authorized_caller() {
        return Err(Error::UnauthorizedAccess {
            msg: "Unauthorized access to update_book".to_string(),
        });
    }

    // Validation: Check if the updated book title is not empty
    if updated_book.title.is_empty() {
        return Err(Error::Validation {
            msg: "Updated book title cannot be empty".to_string(),
        });
    }

    match STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        if let Some(book) = storage.get(&id) {
            let mut updated = book.clone();
            updated.title = updated_book.title;
            updated.author = updated_book.author;
            updated.published_year = updated_book.published_year;

            // Remove the old book and insert the modified one
            storage.remove(&id);
            storage.insert(id, updated.clone());

            Ok(updated)
        } else {
            Err(Error::NotFound {
                msg: format!("couldn't update a book with id={}. book not found", id),
            })
        }
    }) {
        Ok(book) => Ok(book),
        Err(err) => Err(err),
    }
}

#[ic_cdk::query]
fn is_book_available(id: u64) -> bool {
    if let Some(book) = get_book(id).ok() {
        !book.borrowed
    } else {
        false
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ReturnHistory {
    book_id: u64,
    returned_at: u64,
}

thread_local! {
    static RETURN_HISTORY: RefCell<Vec<ReturnHistory>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn get_return_history(book_id: u64) -> Vec<u64> {
    RETURN_HISTORY.with(|history| {
        history
            .borrow()
            .iter()
            .filter(|&entry| entry.book_id == book_id)
            .map(|entry| entry.returned_at)
            .collect()
    })
}

#[ic_cdk::update]
fn record_return_history(book_id: u64) {
    RETURN_HISTORY.with(|history| {
        history.borrow_mut().push(ReturnHistory {
            book_id,
            returned_at: time(),
        });
    });
}

// need this to generate candid
ic_cdk::export_candid!();
