import { useEffect, useState } from 'react';

import Todo from './Todo.js';

export default function Todos() {
    const [todos, setTodos] = useState([]);
    const [error, setError] = useState(null);

    useEffect(() => {
      fetch('http://localhost:3001/todos')
        .then((response) => {
            if (response.ok)
                return response.json();
            setError(`An error occured when fetching todos
                (${response.statusText})`);
        })
        .then((data) => { setTodos(data); })
        .catch((e) => {
            console.log(e.message);
            setError(`An error occurred when fetching todos (${e.message})`);
        });
    }, []);

    if (error != null)
        return (<p>{error}</p>);

    return (
        <div className="flex flex-col gap-4">
          {todos.map((t) => (
              <div key={t.id}>
              <Todo title={t.title} description={t.description}/>
              </div>
          ))}
        </div>
    );
}
