import { Book } from '../types/Book';

export async function getBooks() {
  const response = await fetch(process.env.REACT_APP_API_URL!.concat('library'), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const body = (await response.json()) as unknown;
  assertIsBooks(body);
  return body;
}

export function assertIsBooks(items: unknown): asserts items is Book[] {
  if (!Array.isArray(items)) {
    throw new Error("Response isn't an array");
  }
  if (items.length === 0) {
    return;
  }
  items.forEach((item) => {
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
  });
}
