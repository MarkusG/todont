import { Link, useNavigate, useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';

import Todo from './Todo.ts';
import { getToken } from './auth.tsx';

export default function CreateTodo() {
    const { id } = useParams();
    const [todo, setTodo] = useState<Todo | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
      fetch(`http://localhost:3001/todos/${id}`)
        .then((response) => {
            if (response.ok)
                return response.json();
        })
        .then((todo) => { setTodo(todo); })
        .catch((e) => {
            console.log(e.message);
        });
    }, [id]);

    function submit() {
        // TODO validation
        fetch(`http://localhost:3001/todos/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${getToken()}`
            },
            body: JSON.stringify(todo)
        }).then((response) => {
            if (response.ok)
                navigate('/todos');
        }).catch((e) => {
            console.log(e.message);
        });
    }

    if (todo === null)
        return; // TODO loading icon

    return (
        <div className="w-full h-full p-4 bg-gray-50 border shadow-md">
            <div className="flex justify-between mb-2">
                <Link to={'/todos'} draggable="false">
                    <button className="block p-2 text-gray-500">
                        <i className="fa-solid fa-arrow-left mr-2"></i>
                        Back
                    </button>
                </Link>
                <button className="block p-2 bg-primary-500 text-white rounded" onClick={submit}>
                    Apply
                </button>
            </div>
            <div className="m-auto min-w-fit max-w-[500px]">
                <div className="mb-2">
                    <label htmlFor="title" className="text-lg">Title:</label>
                    <input
                        id="title"
                        name="title"
                        className="block w-full p-1 bg-white border rounded"
                        value={todo.title}
                        onChange={e => setTodo({ ...todo, title: e.target.value })}/>
                </div>
                <div className="mb-2">
                    <label htmlFor="content" className="text-lg">Content:</label>
                    <textarea
                        id="content"
                        name="content"
                        className="block w-full h-[8em] p-1 bg-white border rounded"
                        value={todo.content}
                        onChange={e => setTodo({ ...todo, content: e.target.value })}/>
                </div>
            </div>
        </div>
    )
}
