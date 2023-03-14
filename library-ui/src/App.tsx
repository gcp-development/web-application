import { createBrowserRouter, RouterProvider, defer } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { getBooks } from './api/getBooks';
import { LibraryPage } from './components/LibraryPage';

const queryClient = new QueryClient();

const router = createBrowserRouter([
  {
    path: '/',
    element: <LibraryPage />,
    loader: async () => {
      const existingData = queryClient.getQueryData(['libraryData']);
      if (existingData) {
        return defer({ posts: existingData });
      }
      return defer({ posts: queryClient.fetchQuery(['libraryData'], getBooks) });
    },
  },
]);

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}

export default App;
