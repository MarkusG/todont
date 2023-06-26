import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import Root from './Root.tsx'
import Todos from './Todos.tsx';
import CreateTodo from './CreateTodo.tsx';
import EditTodo from './EditTodo.tsx';

const queryClient = new QueryClient();

const router = createBrowserRouter([
    {
        path: '/',
        element: <Root/>,
        children: [
            {
                path: 'todos',
                element: <Todos/>
            },
            {
                path: 'todos/create',
                element: <CreateTodo/>
            },
            {
                path: 'todos/:id',
                element: <EditTodo/>
            }
        ]
    }
]);

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router}/>
    </QueryClientProvider>
  </React.StrictMode>
);
