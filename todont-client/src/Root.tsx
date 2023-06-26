import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import { useNavigate } from 'react-router-dom';

import './Styles.css';
import Sidebar from './Sidebar.tsx';

function Root() {
    const navigate = useNavigate();
    useEffect(() => {
        document.title = 'Todont';
        navigate('/todos');
    }, [navigate]);

    return (
        <div className="flex flex-col md:flex-row h-screen w-screen bg-gray-100">
          <Sidebar/>
          <div className="w-full p-4">
            <Outlet/>
          </div>
        </div>
    );
}

export default Root;
