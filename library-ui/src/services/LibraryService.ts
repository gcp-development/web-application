import http from "../http-common";
import IBook from "../types/Book";

const addBook = (data: IBook) => {
    return http.post<IBook>("library/", data);
  };

//const bulkInsert = () => {
//   return http.post<IBook>("/tutorials", data);
//};

const getbooks = () => {
    return http.get<Array<IBook>>("/library");
  };

const getBookById = (title: string) => {
    return http.get<Array<IBook>>(`/tutorials?title=${title}`);
  };

const updateBookById = (id: any, data: IBook) => {
  return http.put<any>(`/tutorials/${id}`, data);
};

const deleteBookById  = (id: any) => {
  return http.delete<any>(`/tutorials/${id}`);
};


const LibraryService = {
    addBook,
    //bulkInsert,
    getbooks,
    updateBookById,
    getBookById,
    deleteBookById,
};

export default LibraryService;