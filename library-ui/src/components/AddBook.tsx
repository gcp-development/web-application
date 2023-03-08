import React, { useState, ChangeEvent } from "react";
import LibraryService from "../services/LibraryService";
import IBook from '../types/Book';

const AddBook: React.FC = () => {
    const initialBookState = {
      id: 0,
      title: "",
      author: "",
      posted_time: ""
    };

const [book, setBook] = useState<IBook>(initialBookState);
const [submitted, setSubmitted] = useState<boolean>(false);

const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setBook({ ...book, [name]: value });
};

const saveBook = () => {
    var data = {
        id: book.id,
        title: book.title,
        author: book.author,
        posted_time: book.posted_time
    };

    LibraryService.addBook(data)
      .then((response: any) => {
        setBook({
          id: response.data.id,
          title: response.data.title,
          author: response.data.author,
          posted_time: response.data.posted_time
        });
        setSubmitted(true);
        console.log(response.data);
      })
      .catch((e: Error) => {
        console.log(e);
      });
  };

  const newBook = () => {
    setBook(initialBookState);
    setSubmitted(false);
  };


  return (
    <div className="submit-form">
      {submitted ? (
        <div>
          <h4>You submitted successfully!</h4>
          <button className="btn btn-success" onClick={newBook}>
            Add
          </button>
        </div>
      ) : (
        <div>
          <div className="form-group">
            <label htmlFor="id">Id</label>
            <input
              type="number"
              className="form-control"
              id="id"
              required
              value={book.id}
              onChange={handleInputChange}
              name="id"
            />
          </div>

          <div className="form-group">
            <label htmlFor="title">Title</label>
            <input
              type="text"
              className="form-control"
              id="title"
              required
              value={book.title}
              onChange={handleInputChange}
              name="title"
            />
          </div>

          <div className="form-group">
            <label htmlFor="description">Author</label>
            <input
              type="text"
              className="form-control"
              id="description"
              required
              value={book.author}
              onChange={handleInputChange}
              name="author"
            />
          </div>

          <div className="form-group">
            <label htmlFor="posted_time">Posted_time</label>
            <input
              type="text"
              className="form-control"
              id="posted_time"
              required
              value={book.posted_time}
              onChange={handleInputChange}
              name="posted_time"
            />
          </div>

          <button onClick={saveBook} className="btn btn-success">
            Submit
          </button>
        </div>
      )}
    </div>
  );
};

export default AddBook;