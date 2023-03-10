import { FieldError, useForm } from 'react-hook-form';
import { ValidationError } from './ValidationError';
import { Book } from '../types/Book';
import { useEffect, useState } from 'react';

type Props = {
  onSave: (newBook: Book) => void;
};

export function NewBookForm({ onSave }: Props) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting, isSubmitSuccessful },
  } = useForm<Book>();
  const fieldStyle = 'flex flex-col mb-2';
  function getEditorStyle(fieldError: FieldError | undefined) {
    return fieldError ? 'border-red-500' : '';
  }

  const [bookId, setInput] = useState(0);

  useEffect(() => {
    console.log({ id: bookId }, typeof bookId);
  }, [bookId]);

  function handleChange(event: { target: { name: any; type: any; value: any } }) {
    const { value } = event.target;
    setInput((bookId) => {
      return Number(value);
    });
  }

  return (
    <form noValidate className="border-b py-4" onSubmit={handleSubmit(onSave)}>
      <div className={fieldStyle}>
        <label htmlFor="id">Id</label>
        <input
          type="number"
          id="id"
          {...register('id', {
            required: 'You must enter a id',
          })}
          name="id"
          value={bookId}
          onChange={handleChange}
          className={getEditorStyle(errors.id)}
        />
        <ValidationError fieldError={errors.id} />
      </div>

      <div className={fieldStyle}>
        <label htmlFor="title">Title</label>
        <input
          type="text"
          id="title"
          {...register('title', {
            required: 'You must enter a title',
          })}
          className={getEditorStyle(errors.title)}
        />
        <ValidationError fieldError={errors.title} />
      </div>
      <div className={fieldStyle}>
        <label htmlFor="author">Author</label>
        <textarea
          id="author"
          {...register('author', {
            required: 'You must enter the description',
          })}
          className={getEditorStyle(errors.author)}
        />
        <ValidationError fieldError={errors.author} />
      </div>
      <div className={fieldStyle}>
        <button
          type="submit"
          disabled={isSubmitting}
          className="mt-2 h-10 px-6 font-semibold bg-black text-white"
        >
          Save
        </button>
        {isSubmitSuccessful && (
          <div role="alert" className="text-green-500 text-xs mt-1">
            The post was successfully saved
          </div>
        )}
      </div>
    </form>
  );
}
