import { createSignal } from 'solid-js';
import { type Calendar, fetchCalMonth } from './apiInterface';

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
      fetchCalMonth(year, month).then(res => {
        console.log(`[DBG] fetched month: ${JSON.stringify(res)}`);
        const newCache = structuredClone(cache());
        newCache[key] = res;
        setCache(newCache);
      });
    }
  }

  return {
    getMonth,
  };
}
