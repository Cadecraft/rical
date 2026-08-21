import type { Calendar, TaskData } from './types';

export function apiRoot() {
  const res: string | undefined = import.meta.env.VITE_API_URL;
  if (!res) {
    console.error("Root API URL is undefined");
  }
  return res;
}

export function genHeaders(...needed: ('auth' | 'json')[]): HeadersInit {
  const res: HeadersInit = {};
  if (needed.includes('auth')) {
    // TODO: switch to cookies
    res['Authorization'] = 'Bearer ' + localStorage.tok;
  }
  if (needed.includes('json')) {
    res['Content-Type'] = 'application/json';
  }
  return res;
}

export async function fetchCalMonth(year: number, month: number): Promise<Calendar> {
  const res = await fetch(`${apiRoot()}/calendar/${year}/${month}`, {
    method: 'GET',
    headers: genHeaders('auth', 'json'),
  });
  if (!res.ok) {
    throw new Error("Could not fetch calendar from API");
  }
  return await res.json();
}

export async function fetchSignup(username: string, password: string): Promise<true> {
  const res = await fetch(`${apiRoot()}/account/signup`, {
    method: 'POST',
    headers: genHeaders('json'),
    body: JSON.stringify({ username, password }),
  });
  if (!res.ok) {
    throw new Error("Could not sign up with API");
  }
  return true;
}

export async function fetchLogin(username: string, password: string): Promise<{ token: string }> {
  const res = await fetch(`${apiRoot()}/account/login`, {
    method: 'POST',
    headers: genHeaders('json'),
    body: JSON.stringify({ username, password }),
  });
  if (!res.ok) {
    throw new Error("Could not log in with API");
  }
  return await res.json();
}

/** Returns task id */
export async function fetchCreateTask(taskData: TaskData): Promise<number> {
  const res = await fetch(`${apiRoot()}/task`, {
    method: 'POST',
    headers: genHeaders('auth', 'json'),
    body: JSON.stringify(taskData),
  });
  if (!res.ok) {
    throw new Error("Could not fetch calendar from API");
  }
  return await res.json();
}
