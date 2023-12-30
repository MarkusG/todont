import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { setToken } from './auth';

interface LoginPayload {
    username: string,
    password: string
}

export default function Login() {
    useEffect(() => { document.title = 'Login' });

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState(false);

    const navigate = useNavigate();

    function login(payload: LoginPayload) {
        fetch(`http://localhost:3001/authenticate`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(payload)
        }).then((response) => {
            if (!response.ok) {
                setError(true);
                return Promise.reject(response);
            }
            return response.text();
        }).then(data => {
            setToken(data);
            navigate('/todos');
        }).catch((e) => {
            console.log(e.message);
        });
    }

    function submit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const loginPayload: LoginPayload = {
            username: e.target.username.value,
            password: e.target.password.value };
        login(loginPayload);
    }

    function errorDisplay() {
        if (!error) {
            return;
        }

        return (
            <p className="text-danger-500">Invalid username/password. Please try again.</p>
        );
    }

    return (
        <div className="md:w-screen md:h-screen md:bg-gray-100 md:py-4">
        <form onSubmit={submit}>
            <div className="flex flex-col gap-2 max-w-[400px] md:shadow-md bg-white mx-auto p-8">
                <div>
                    <label htmlFor="username" className="text-sm">Username:</label>
                    <input
                    id="username"
                    name="username"
                    className="block w-full p-1 border rounded"
                    value={username}
                    onChange={e => setUsername(e.target.value)}/>
                </div>
                <div>
                    <label htmlFor="password" className="text-sm">Password:</label>
                    <input
                    id="password"
                    name="password"
                    type="password"
                    className="block w-full p-1 border rounded"
                    value={password}
                    onChange={e => setPassword(e.target.value)}/>
                </div>
                <button
                    type="submit"
                    className="block p-2 mt-2 bg-primary-500 text-white rounded">
                    Login
                </button>
                {errorDisplay()}
            </div>
        </form>
        </div>
    );
}
