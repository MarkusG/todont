export function setToken(token: string) : void {
    sessionStorage.setItem('token', token);
}

export function getToken() : string | null {
    return sessionStorage.getItem('token');
}
