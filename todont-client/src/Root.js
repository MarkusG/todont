import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';

import './Styles.css';
import Sidebar from './Sidebar.js';

function App() {
    useEffect(() => {
        document.title = 'Todont';
    }, []);

    return (
        <div className="flex flex-col md:flex-row h-screen w-screen bg-gray-100">
          <Sidebar/>
          <div className="w-full p-4">
            <Outlet/>
          </div>
        </div>
    );
}

export default App;
