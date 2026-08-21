import { fetchCalMonth, fetchCreateTask } from './apiInterface';
import type { TaskData } from './types';
import { useCalendarState } from './StateProvider';
import { unwrap } from 'solid-js/store';

export function useCalCache() {
  const [state, setState] = useCalendarState();

  const toKey = (year: number, month: number) => {
    return `${year}/${month}`;
  }

  const getMonth = async (year: number, month: number) => {
    const cache = state.calCache;

    const key = toKey(year, month);
    if (key in cache) {
      return Promise.resolve(cache[key]);
    } else {
      fetchCalMonth(year, month).then(res => {
        console.log(`[DBG] fetched month: ${JSON.stringify(res)}`);
        setState('calCache', (c) => ({ ...c, [key]: res }));
      });
    }
  }

  const createTask = async (taskData: TaskData) => {
    const cache = state.calCache;

    fetchCreateTask(taskData).then((task_id) => {
      const key = toKey(taskData.year, taskData.month);
      const newCache = structuredClone(unwrap(cache));
      const createdTask = { ...taskData, task_id };
      newCache[key].days[taskData.day - 1].push(createdTask);
      setState('calCache', newCache);
    });
  }

  return {
    getMonth,
    createTask,
  };
}
