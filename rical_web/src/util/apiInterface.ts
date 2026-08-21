import type { Task } from './types';

export type Calendar = {
  days: Task[],
};

export function apiRoot() {
  const res: string | undefined = import.meta.env.VITE_API_URL;
  if (!res) {
    console.error("Root API URL is undefined");
  }
  return res;
}

export async function fetchCalMonth(year: number, month: number): Promise<Calendar> {
  const res = await fetch(`${apiRoot()}/calendar/${year}/${month}`, {
    method: 'GET',
    headers: {
      // TODO: switch to cookies
      'Authorization': 'Bearer ' + localStorage.tok,
    }
  });
  if (!res.ok) {
    throw new Error("Could not fetch calendar from API");
  }
  return await res.json();
}

export async function fetchSignup(username: string, password: string): Promise<true> {
  const res = await fetch(`${apiRoot()}/account/signup`, {
    method: 'POST',
    body: JSON.stringify({ username, password }),
    headers: {
      'Content-Type': 'application/json'
    }
  });
  if (!res.ok) {
    throw new Error("Could not sign up with API");
  }
  return true;
}

export async function fetchLogin(username: string, password: string): Promise<{ token: string }> {
  const res = await fetch(`${apiRoot()}/account/login`, {
    method: 'POST',
    body: JSON.stringify({ username, password }),
    headers: {
      'Content-Type': 'application/json'
    }
  });
  if (!res.ok) {
    throw new Error("Could not log in with API");
  }
  return await res.json();
}
