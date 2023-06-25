import { useEffect, useState } from 'react';

import Todo from './Todo.tsx';
import CreateTodoButton from './CreateTodoButton.tsx';

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

    function done(todo) {
        setTodos(todos.map(t => { return t.id === todo.id ? todo : t}));
    };

    function deleted(id) {
        setTodos(todos.filter(t => { return t.id !== id }));
    };

    if (error != null)
        return (<p>{error}</p>);

    const doneTodos = todos.filter(t => { return t.completed_at !== null});

    return (
        <>
        <div className="mb-4">
            <CreateTodoButton/>
        </div>
        <div className={`flex flex-col gap-4${doneTodos.length > 0 ? 'mb-4' : ''}`}>
          {todos.filter(t => { return t.completed_at === null}).map((t) => (
              <div key={t.id}>
              <Todo todo={t}
                onDone={done}
                onDelete={deleted}/>
              </div>
          ))}
        </div>
        {doneTodos.length > 0 &&
            <h3 className="text-3xl mb-4 text-center">Completed Todos</h3>
        }
            {doneTodos.map((t) => (
                <div key={t.id}>
                <Todo todo={t}
                  onDelete={deleted}/>
                </div>
            ))}
        </>
    );
}