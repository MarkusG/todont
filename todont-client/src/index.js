import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';

import Root from './Root.js';
import Todos from './Todos.js';
import CreateTodo from './CreateTodo.js';

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
            }
        ]
    }
]);

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>
);
