import { useEffect } from 'react';
import './App.css';
import Sidebar from './Sidebar.js';
import Todos from './Todos.js';

function App() {
    useEffect(() => {
        document.title = 'Todont';
    }, []);
    return (
        <>
        <div className="flex flex-col md:flex-row h-screen w-screen">
          <Sidebar/>
          { /* main page container */ }
          <div className="w-full p-4">
            <Todos/>
          </div>
        </div>
        </>
    );
}

export default App;
