export default function Todo({ todo, onDone, onDelete }) {
    function done() {
        todo.completed_at = new Date();
        fetch(`http://localhost:3001/todos/${todo.id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(todo)
        })
            .then((response) => {
                if (response.ok)
                    onDone(todo);
            })
            .catch((e) => {
                console.log(e.message);
            });
    };

    function delete_todo() {
        fetch(`http://localhost:3001/todos/${todo.id}`, {
            method: "DELETE",
        })
            .then((response) => {
                if (response.ok)
                    onDelete(todo.id);
            })
            .catch((e) => {
                console.log(e.message);
            });
    };

    return (
      <div className="p-4 flex justify-between border shadow-md">
        <div>
          <h2 className="text-xl">{todo.title}</h2>
          <p>{todo.description}</p>
        </div>
        <div className="ml-2 my-auto flex gap-2">
        {todo.completed_at === null &&
          <span className="cursor-pointer check" onClick={done}></span>
        }
          <span className="cursor-pointer x" onClick={delete_todo}></span>
        </div>
      </div>
    )
}
