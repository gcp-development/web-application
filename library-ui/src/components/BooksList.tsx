import { Book } from '../types/Book';

type Props = {
  books: Book[];
};
export function BooksList({ books }: Props) {
  return (
    <ul className="list-none">
      {books.map((book) => (
        <li key={book.id} className="border-b py-4">
          <h3 className="text-slate-900 font-bold">{book.title}</h3>
          <p className=" text-slate-900 ">{book.author}</p>
        </li>
      ))}
    </ul>
  );
}
