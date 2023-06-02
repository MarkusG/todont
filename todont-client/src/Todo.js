export default function Todo({ todo, onDone }) {
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
                    onDone(todo.id);
            })
            .catch((e) => {
                console.log(e.message);
            });
    }

    return (
      <div className="p-4 flex justify-between border shadow-md">
        <div>
          <h2 className="text-xl">{todo.title}</h2>
          <p>{todo.description}</p>
        </div>
        <div className="ml-2 my-auto flex gap-2">
          <span className="cursor-pointer check" onClick={done}></span>
          <span className="cursor-pointer x"></span>
        </div>
      </div>
    )
}
