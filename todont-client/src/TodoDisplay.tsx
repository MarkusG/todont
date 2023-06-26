import { useNavigate } from 'react-router-dom';

import Todo from './Todo.ts';

export interface TodoDisplayProps {
    todo: Todo;
    onDone?: (todo: Todo) => void;
    onDelete: (id: string) => void;
}

const TodoDisplay = (props: TodoDisplayProps) => {
    const navigate = useNavigate();
    const { todo, onDone, onDelete } = props;

    function done() {
        if (onDone)
            onDone(todo);
    }

    function edit() {
        navigate(`/todos/${todo.id}`);
    }

    function delete_todo() {
        if (onDelete)
            onDelete(todo.id);
    }

    return (
      <div className="p-4 flex justify-between border shadow-md bg-gray-50">
        <div>
          <h2 className="text-2xl">{todo.title}</h2>
          <p>{todo.content}</p>
        </div>
        <div className="ml-2 my-auto flex gap-4">
        {todo.completed_at === null &&
          <button className="text-success-500" onClick={done}>
            <i className="fa-solid fa-lg fa-check"></i>
          </button>
        }
          <button className="text-gray-500" onClick={edit}>
            <i className="fa-solid fa-lg fa-pencil"></i>
          </button>
          <button className="text-danger-500" onClick={delete_todo}>
            <i className="fa-solid fa-lg fa-x"></i>
          </button>
        </div>
      </div>
    )
}

export default TodoDisplay;
