import { createSignal } from 'solid-js';
import type { Task } from './types';

type Calendar = {
  days: Task[],
};

export function useCalCache() {
  const [cache, setCache] = createSignal<Record<string, Calendar>>({});

  const toKey = (year: number, month: number) => {
    return `${year}/${month}`;
  }

  const getMonth = async (year: number, month: number) => {
    const key = toKey(year, month);
    if (key in cache()) {
      return Promise.resolve(cache()[key]);
    } else {
      fetch(`${import.meta.env.VITE_API_URL}/calendar/${year}/${month}`, {
        method: 'GET',
        headers: {
          // TODO: switch to cookies
          'Authorization': 'Bearer ' + localStorage.tok,
        }
      }).then(res => {
        if (res.ok) {
          res.json().then(j => {
            console.log(JSON.stringify(j));
            const newCache = structuredClone(cache());
            newCache[key] = j;
            setCache(newCache);
          });
        }
      });
    }
  }

  return {
    getMonth,
  };
}
