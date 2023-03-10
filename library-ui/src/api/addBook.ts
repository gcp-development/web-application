import { Book } from '../types/Book';

export async function addBook(newBook: Book) {
  const response = await fetch(process.env.REACT_APP_API_URL!.concat('library/'), {
    method: 'POST',
    body: JSON.stringify(newBook),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const body = (await response.json()) as unknown;
  assertIfNewItemIsBook(body);
  return { ...newBook, ...body };
}

function assertIfNewItemIsBook(item: any): asserts item is Book {
  if (!('id' in item)) {
    throw new Error("Book doesn't contain id");
  }
  if (typeof item.id !== 'number') {
    throw new Error('id is not a number');
  }
  if (!('title' in item)) {
    throw new Error("Book doesn't contain title");
  }
  if (typeof item.title !== 'string') {
    throw new Error('Title is not a string');
  }
  if (!('author' in item)) {
    throw new Error("book doesn't contain author");
  }
  if (typeof item.author !== 'string') {
    throw new Error('author is not a string');
  }
}
