import { Link, useNavigate, useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

export default function CreateTodo() {
    const { id } = useParams();
    const [title, setTitle] = useState("");
    const [content, setContent] = useState("");
    const navigate = useNavigate();

    useEffect(() => {
      fetch(`http://localhost:3001/todos/${id}`)
        .then((response) => {
            if (response.ok)
                return response.json();
        })
        .then((todo) => { setTitle(todo.title); setContent(todo.content); })
        .catch((e) => {
            console.log(e.message);
        });
    }, [id]);

    function submit() {
        // TODO validation
        fetch(`http://localhost:3001/todos/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ title, content })
        }).then((response) => {
            if (response.ok)
                navigate('/todos');
        }).catch((e) => {
            console.log(e.message);
        });
    }

    return (
        <div className="w-full h-full p-4 bg-gray-50 border shadow-md">
            <div className="flex justify-between mb-2">
                <Link to={'/todos'} draggable="false">
                    <button className="block p-2 text-gray-500">
                        <FontAwesomeIcon icon={solid('arrow-left')} size="md" className="mr-2"/>
                        Back
                    </button>
                </Link>
                <button className="block p-2 bg-primary-500 text-white rounded" onClick={submit}>
                    Apply
                </button>
            </div>
            <div className="m-auto min-w-fit max-w-[500px]">
                <div className="mb-2">
                    <label for="title" className="text-lg">Title:</label>
                    <input
                        id="title"
                        name="title"
                        className="block w-full p-1 bg-white border rounded"
                        value={title}
                        onChange={e => setTitle(e.target.value)}/>
                </div>
                <div className="mb-2">
                    <label for="content" className="text-lg">Content:</label>
                    <textarea
                        id="content"
                        name="content"
                        className="block w-full h-[8em] p-1 bg-white border rounded"
                        value={content}
                        onChange={e => setContent(e.target.value)}/>
                </div>
            </div>
        </div>
    )
}
