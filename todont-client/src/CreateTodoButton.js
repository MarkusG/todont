import { Link } from 'react-router-dom';

export default function CreateTodoButton() {
    return (
        <Link to={'/todos/create'} draggable="false">
            <button className="text-xl p-2 rounded bg-primary-500 text-white">
                <span className="mr-2 plus"></span>
                Create Todo
            </button>
        </Link>
    )
}
