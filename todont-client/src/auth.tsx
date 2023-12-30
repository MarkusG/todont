export function setToken(token: string) {
    sessionStorage.setItem('token', token);
}

export function getToken() {
    return sessionStorage.getItem('token');
}
