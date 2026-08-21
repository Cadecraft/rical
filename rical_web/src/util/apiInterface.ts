import type { Task } from './types';

export type Calendar = {
  days: Task[],
};

export async function fetchCalMonth(year: number, month: number): Promise<Calendar> {
  const res = await fetch(`${import.meta.env.VITE_API_URL}/calendar/${year}/${month}`, {
    method: 'GET',
    headers: {
      // TODO: switch to cookies
      'Authorization': 'Bearer ' + localStorage.tok,
    }
  });
  if (!res.ok) {
    throw new Error("Could not fetch calendar from API");
  }
  const j = await res.json();
  return j;
}
