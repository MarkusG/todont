import { useEffect, useState } from 'react';
import './App.css';
import NavLink from './NavLink.js';
import Todo from './Todo.js';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro'

function App() {
  useEffect(() => {
    document.title = 'Todont';
  }, []);
  const [todos, setTodos] = useState([]);
  useEffect(() => {
    fetch('http://localhost:3001/todos')
      .then((response) => response.json())
      .then((data) => { setTodos(data); })
      .catch((e) => { console.log(e.message); });
  }, []);
  return (
      <>
      <div className="flex flex-col md:flex-row h-screen w-screen">
      { /* sidebar */ }
        <div className="w-full md:w-[80px] md:h-full h-[60px] py-4 bg-slate-700 text-white text-center">
          <NavLink name="Todos" icon={solid('table-list')}/>
        </div>
        { /* main page container */ }
        <div className="w-full p-4">
          <div className="flex flex-col gap-4">
            {todos.map((t) => (
                <div key={t.id}>
                    <Todo title={t.title} description={t.description}/>
                </div>
            ))}
          </div>
        </div>
      </div>
      </>
  );
}

export default App;
