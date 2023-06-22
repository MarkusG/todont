import { useEffect } from 'react';
import './App.css';
import Sidebar from './Sidebar.js';
import CreateTodoButton from './CreateTodoButton.js';
import Todos from './Todos.js';

function App() {
    useEffect(() => {
        document.title = 'Todont';
    }, []);
    return (
        <div className="flex flex-col md:flex-row h-screen w-screen bg-gray-100">
          <Sidebar/>
          { /* main page container */ }
          <div className="w-full p-4 flex flex-col gap-4">
            <div>
                <CreateTodoButton/>
            </div>
            <Todos/>
          </div>
        </div>
    );
}

export default App;
