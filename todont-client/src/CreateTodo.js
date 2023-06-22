import { Link } from 'react-router-dom';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro';

export default function CreateTodo() {
    return (
        <div className="w-full h-full p-4 bg-gray-50 border shadow-md">
            <Link to={'/todos'}>
                <button className="p-2 text-gray-500">
                    <FontAwesomeIcon icon={solid('arrow-left')} size="md" className="mr-2"/>
                    Back
                </button>
            </Link>
        </div>
    )
}
