import type { Calendar, TaskData, Task } from './types';

function apiRoot() {
  const res: string | undefined = import.meta.env.VITE_API_URL;
  if (!res) {
    console.error("Root API URL is undefined");
  }
  return res;
}

type HeaderOptions = ('auth' | 'json')[];

export function genHeaders(needed: HeaderOptions): HeadersInit {
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

async function fetchTemplate(
  method: string,
  path: string,
  headersNeeded: HeaderOptions,
  expectedReturn: 'json' | 'none',
  body?: string,
) {
  const res = await fetch(apiRoot() + path, {
    method,
    headers: genHeaders(headersNeeded),
    body,
  });
  if (!res.ok) {
    throw new Error("Could not fetch from API: " + res.status);
  }
  if (expectedReturn === 'json') {
    return await res.json();
  } else {
    return true;
  }
}

export async function fetchSignup(username: string, password: string): Promise<true> {
  return await fetchTemplate('POST', "/account/signup", ['json'], 'none', JSON.stringify({
    username, password
  }));
}

export async function fetchLogin(username: string, password: string): Promise<{ token: string }> {
  return await fetchTemplate('POST', "/account/login", ['json'], 'json', JSON.stringify({
    username, password
  }));
}

export async function fetchCalMonth(year: number, month: number): Promise<Calendar> {
  return await fetchTemplate('GET', `/calendar/${year}/${month}`, ['auth', 'json'], 'json');
}

/** Returns task id */
export async function fetchCreateTask(taskData: TaskData): Promise<{ task_id: number }> {
  return await fetchTemplate('POST', "/task", ['auth', 'json'], 'json', JSON.stringify(taskData));
}

export async function fetchTask(id: number): Promise<Task> {
  return await fetchTemplate('GET', `/task/${id}`, ['auth', 'json'], 'json');
}

/** Returns the old task */
export async function fetchPutTask(task: Task): Promise<Task> {
  return await fetchTemplate('PUT', `/task/${task.task_id}`, ['auth', 'json'], 'json', JSON.stringify(task));
}
