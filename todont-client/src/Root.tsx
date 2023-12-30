import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import { Navigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';

import { getToken } from './auth';

import './Styles.css';
import Sidebar from './Sidebar.tsx';

function Root() {
    const location = useLocation();

    useEffect(() => {
        document.title = 'Todont';
    });

    if (getToken() === null) {
        return (<Navigate to="/login"/>);
    }

    if (location.pathname === '/') {
        return (<Navigate to="/todos"/>);
    }

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
