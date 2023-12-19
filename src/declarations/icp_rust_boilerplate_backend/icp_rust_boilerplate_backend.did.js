export const idlFactory = ({ IDL }) => {
  const BookPayload = IDL.Record({
    'title' : IDL.Text,
    'author' : IDL.Text,
    'published_year' : IDL.Nat32,
  });
  const Book = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'borrowed_at' : IDL.Opt(IDL.Nat64),
    'borrowed_by' : IDL.Opt(IDL.Text),
    'borrowed' : IDL.Bool,
    'author' : IDL.Text,
    'published_year' : IDL.Nat32,
  });
  const Error = IDL.Variant({
    'BookNotBorrowed' : IDL.Record({ 'msg' : IDL.Text }),
    'BookAlreadyBorrowed' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'UnauthorizedAccess' : IDL.Record({ 'msg' : IDL.Text }),
    'Validation' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Opt(Book), 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : Book, 'Err' : Error });
  return IDL.Service({
    'add_book' : IDL.Func([BookPayload], [Result], []),
    'borrow_book' : IDL.Func([IDL.Nat64, IDL.Text], [Result_1], []),
    'get_book' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_return_history' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(IDL.Nat64)],
        ['query'],
      ),
    'is_book_available' : IDL.Func([IDL.Nat64], [IDL.Bool], ['query']),
    'is_book_borrowed' : IDL.Func([IDL.Nat64], [IDL.Bool], ['query']),
    'list_books' : IDL.Func([], [IDL.Vec(Book)], ['query']),
    'record_return_history' : IDL.Func([IDL.Nat64], [], []),
    'return_book' : IDL.Func([IDL.Nat64], [Result_1], []),
    'update_book' : IDL.Func([IDL.Nat64, BookPayload], [Result_1], []),
    'view_book' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
