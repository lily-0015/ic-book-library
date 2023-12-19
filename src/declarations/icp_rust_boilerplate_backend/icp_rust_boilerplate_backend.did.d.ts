import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Book {
  'id' : bigint,
  'title' : string,
  'borrowed_at' : [] | [bigint],
  'borrowed_by' : [] | [string],
  'borrowed' : boolean,
  'author' : string,
  'published_year' : number,
}
export interface BookPayload {
  'title' : string,
  'author' : string,
  'published_year' : number,
}
export type Error = { 'BookNotBorrowed' : { 'msg' : string } } |
  { 'BookAlreadyBorrowed' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } } |
  { 'UnauthorizedAccess' : { 'msg' : string } } |
  { 'Validation' : { 'msg' : string } };
export type Result = { 'Ok' : [] | [Book] } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Book } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_book' : ActorMethod<[BookPayload], Result>,
  'borrow_book' : ActorMethod<[bigint, string], Result_1>,
  'get_book' : ActorMethod<[bigint], Result_1>,
  'get_return_history' : ActorMethod<[bigint], BigUint64Array | bigint[]>,
  'is_book_available' : ActorMethod<[bigint], boolean>,
  'is_book_borrowed' : ActorMethod<[bigint], boolean>,
  'list_books' : ActorMethod<[], Array<Book>>,
  'record_return_history' : ActorMethod<[bigint], undefined>,
  'return_book' : ActorMethod<[bigint], Result_1>,
  'update_book' : ActorMethod<[bigint, BookPayload], Result_1>,
  'view_book' : ActorMethod<[bigint], Result_1>,
}
