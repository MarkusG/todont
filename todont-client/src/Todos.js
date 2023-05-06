import { useEffect, useState } from 'react';

import Todo from './Todo.js';

export default function Todos() {
    const [todos, setTodos] = useState([]);
    useEffect(() => {
      fetch('http://localhost:3001/todos')
        .then((response) => response.json())
        .then((data) => { setTodos(data); })
        .catch((e) => { console.log(e.message); });
    }, []);
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
