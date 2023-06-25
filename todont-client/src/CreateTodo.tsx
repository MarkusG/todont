import { Link, useNavigate } from 'react-router-dom';
import { useState } from 'react';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

import InputText from './Forms/InputText.tsx';
import InputTextArea from './Forms/InputTextArea.tsx';

export default function CreateTodo() {
    const [title, setTitle] = useState({ value: "", errors: [] });
    const [content, setContent] = useState({ value: "", errors: [] });
    const navigate = useNavigate();

    function submit() {
        fetch(`http://localhost:3001/todos`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ title: title.value, content: content.value })
        }).then((response) => {
            if (response.ok)
                navigate('/todos');
        }).catch((e) => {
            console.log(e.message);
        });
    }

    function updateTitle(title) {
        let newTitle = { value: title, errors: [] };
        if (title.length > 50)
            newTitle.errors.push('Title must be 50 characters or less');
        setTitle(newTitle)
    }

    function updateContent(content) {
        let newContent = { value: content, errors: [] };
        if (content.length > 1000)
            newContent.errors.push('Content must be 1000 characters or less');
        setContent(newContent)
    }

    return (
        <div className="w-full h-full p-4 bg-gray-50 border shadow-md">
            <div className="flex justify-between mb-2">
                <Link to={'/todos'} draggable="false">
                    <button className="block p-2 text-gray-500">
                        <FontAwesomeIcon icon={solid('arrow-left')} className="mr-2"/>
                        Back
                    </button>
                </Link>
                <button className="block p-2 bg-primary-500 disabled:bg-primary-300 text-white rounded"
                    disabled={title.errors.length !== 0 || content.errors.length !== 0}
                    onClick={submit}>
                    Create
                </button>
            </div>
            <div className="m-auto min-w-fit max-w-[500px]">
                <div className="mb-2">
                    <label htmlFor="title" className="text-lg">Title:</label>
                    <InputText
                        id="title"
                        name="title"
                        className="block w-full p-1 bg-white border rounded"
                        value={title.value}
                        errors={title.errors}
                        onChange={e => updateTitle(e)}/>
                </div>
                <div className="mb-2">
                    <label htmlFor="content" className="text-lg">Content:</label>
                    <InputTextArea
                        id="content"
                        name="content"
                        className="block w-full h-[8em] p-1 bg-white border rounded"
                        value={content.value}
                        errors={content.errors}
                        onChange={e => updateContent(e)}/>
                </div>
            </div>
        </div>
    )
}
