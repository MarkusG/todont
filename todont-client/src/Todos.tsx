import { useQuery, useQueryClient, useMutation } from '@tanstack/react-query';

import TodoDisplay from './TodoDisplay.tsx';
import CreateTodoButton from './CreateTodoButton.tsx';

import Todo from './Todo.ts';

export default function Todos() {
    const queryClient = useQueryClient();

    const { isLoading, error, data } = useQuery<Todo[], Error>({
        queryKey: ['todos'],
        queryFn: () =>
            fetch('http://localhost:3001/todos')
            .then((res) => res.json()),
        initialData: []
    });

    const doneMutation = useMutation({
        mutationFn: async (todo: Todo) => {
            todo.completed_at = new Date();
            await fetch(`http://localhost:3001/todos/${todo.id}`, {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({...todo, completed_at: new Date() })
            }).then((res) => res.json())
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['todos'] });
        },
    });

    const deleteMutation = useMutation({
        mutationFn: async (id: string) => { 
            await fetch(`http://localhost:3001/todos/${id}`, {
                method: "DELETE",
            })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['todos'] });
        },
    });

    if (isLoading) return 'Loading...';

    if (error) return 'An error has occurred: ' + error.message;

    const doneTodos = data.filter(t => { return t.completed_at !== null});

    return (
        <>
        <div className="mb-4">
            <CreateTodoButton/>
        </div>
        <div className={`flex flex-col gap-4${doneTodos.length > 0 ? 'mb-4' : ''}`}>
          {data.filter(t => { return t.completed_at === null}).map((t) => (
              <div key={t.id}>
              <TodoDisplay
                todo={t}
                onDone={(todo) => { doneMutation.mutate(todo) }}
                onDelete={(id) => { deleteMutation.mutate(id) }}/>
              </div>
          ))}
        </div>
        {doneTodos.length > 0 &&
            <h3 className="text-3xl my-4 text-center">Completed Todos</h3>
        }
            {doneTodos.map((t) => (
                <div key={t.id}>
                <TodoDisplay
                  todo={t}
                  onDelete={(id) => { deleteMutation.mutate(id) }}/>
                </div>
            ))}
        </>
    );
}
