import { fetchCalMonth, fetchCreateTask, fetchTask } from './apiInterface';
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
    if (key in cache.months) {
      return Promise.resolve(cache.months[key]);
    }

    const res = await fetchCalMonth(year, month)
    console.log(`[DBG] fetched month: ${JSON.stringify(res)}`);
    const newCache = structuredClone(unwrap(cache));
    newCache.months[key] = res;
    setState('calCache', newCache);
    return res;
  }

  const createTask = async (taskData: TaskData) => {
    const cache = state.calCache;

    fetchCreateTask(taskData).then((task_id) => {
      const key = toKey(taskData.year, taskData.month);
      const newCache = structuredClone(unwrap(cache));
      const createdTask = { ...taskData, task_id };
      newCache.months[key].days[taskData.day - 1].push(createdTask);
      setState('calCache', newCache);
    });
  }

  const getTask = async (id: number) => {
    const cache = state.calCache;

    if (id in cache.tasks) {
      return cache.tasks[id];
    }

    const task = await fetchTask(id);
    const newCache = structuredClone(unwrap(cache));
    newCache.tasks[id] = task;
    return task;
  }

  return {
    getMonth,
    createTask,
  };
}
