import { useState } from 'react';

import Modal from './Modal.js';

export default function CreateTodoButton() {
    const [isOpen, setIsOpen] = useState(false);

    return (
        <>
        <button className="text-xl p-2 rounded bg-primary-500 text-white"
            onClick={() => { setIsOpen(true); }}>
            <span className="mr-2 plus"></span>
            Create Todo
        </button>
        <Modal
            isOpen={isOpen}
            onRequestClose={() => { setIsOpen(false) }}>
            <div>Hello world!</div>
        </Modal>
        </>
    )
}
