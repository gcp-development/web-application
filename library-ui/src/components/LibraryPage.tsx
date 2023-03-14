import { Suspense } from 'react';
import { useLoaderData, Await, useNavigate } from 'react-router-dom';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { assertIsBooks } from '../api/getBooks';
import { Book } from '../types/Book';
import { BooksList } from './BooksList';
import { addBook } from '../api/addBook';
import { NewBookForm } from './NewBookForm';

type Data = {
  posts: Book[];
};

export function assertIsData(data: unknown): asserts data is Data {
  if (typeof data !== 'object') {
    throw new Error("Data isn't an object");
  }
  if (data === null) {
    throw new Error('Data is null');
  }
  if (!('posts' in data)) {
    throw new Error("data doesn't contain posts");
  }
}

export function LibraryPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { mutate } = useMutation(addBook, {
    onSuccess: (addBook) => {
      queryClient.setQueryData<Book[]>(['libraryData'], (oldData) => {
        if (oldData === undefined) {
          return [addBook];
        } else {
          return [addBook, ...oldData];
        }
      });
      navigate('/');
    },
  });

  const data = useLoaderData();
  assertIsData(data);

  return (
    <div className="w-96 mx-auto mt-6">
      <h2 className="text-xl text-slate-900 font-bold">Library</h2>
      <NewBookForm onSave={mutate} />
      <Suspense fallback={<div>Fetching...</div>}>
        <Await resolve={data.posts}>
          {(books) => {
            assertIsBooks(books);
            return <BooksList books={books} />;
          }}
        </Await>
      </Suspense>
    </div>
  );
}
