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

    function done(id) {
        setTodos(todos.filter(t => { return t.id !== id }));
    };

    if (error != null)
        return (<p>{error}</p>);

    return (
        <>
        <div className="flex flex-col gap-4">
          {todos.filter(t => { return t.completed_at === null}).map((t) => (
              <div key={t.id}>
              <Todo todo={t}
                onDone={done}/>
              </div>
          ))}
        </div>
        <h3 className="text-3xl my-4 text-center">Completed Todos</h3>
          {todos.filter(t => { return t.completed_at !== null}).map((t) => (
              <div key={t.id}>
              <Todo todo={t}
                onDone={done}/>
              </div>
          ))}
        </>
    );
}
