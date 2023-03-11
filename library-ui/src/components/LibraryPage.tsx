import { useEffect, useState } from 'react';
import { getBooks } from '../api/getBooks';
import { Book } from '../types/Book';
import { BooksList } from './BooksList';

import { addBook } from '../api/addBook';
import { NewBookForm } from './NewBookForm';

export function LibraryPage() {
  const [isLoading, setIsLoading] = useState(true);
  const [books, setBooks] = useState<Book[]>([]);
  useEffect(() => {
    let cancel = false;
    getBooks().then((data) => {
      if (!cancel) {
        setBooks(data);
        setIsLoading(false);
      }
    });
    return () => {
      cancel = true;
    };
  }, []);

  async function handleSave(newBook: Book) {
    //This is due the input type that does return the type number.
    newBook.id = parseInt(newBook.id.toString());
    const newPost = await addBook(newBook);
    setBooks([newPost, ...books]);
  }

  if (isLoading) {
    return <div className="w-96 mx-auto mt-6">Loading ...</div>;
  }
  return (
    <div className="w-96 mx-auto mt-6">
      <h2 className="text-xl text-slate-900 font-bold">Library</h2>
      <NewBookForm onSave={handleSave} />
      <BooksList books={books} />
    </div>
  );
}
