import { fetchCalMonth, fetchCreateTask, fetchTask, fetchPutTask, fetchDeleteTask } from './apiInterface';
import type { TaskData, Task } from './types';
import { useCalendarState } from './StateProvider';
import { unwrap } from 'solid-js/store';
import { type Ridate } from './ridate';

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
    for (const day of res.days) {
      for (const task of day) {
        newCache.tasks[task.task_id] = task;
      }
    }
    setState('calCache', newCache);
    return res;
  }

  const getTasksAtDate = (date: Ridate) => {
    const cache = state.calCache;
    const key = toKey(date.year, date.month);
    return cache.months[key].days[date.dayOfMonth - 1];
  }

  const createTask = async (taskData: TaskData) => {
    const cache = state.calCache;

    fetchCreateTask(taskData).then((res) => {
      const key = toKey(taskData.year, taskData.month);
      const newCache = structuredClone(unwrap(cache));
      const createdTask = { ...taskData, task_id: res.task_id };
      newCache.months[key].days[taskData.day - 1].push(createdTask);
      newCache.tasks[res.task_id] = createdTask;
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
    setState('calCache', newCache);
    return task;
  }

  const putTask = async (task: Task) => {
    const cache = state.calCache;

    fetchPutTask(task).then(() => {
      const newCache = structuredClone(unwrap(cache));
      newCache.tasks[task.task_id] = task;
      // TODO: moving tasks
      newCache.months[toKey(task.year, task.month)].days[task.day - 1].forEach(t => {
        if (t.task_id === task.task_id) {
          Object.assign(t, task);
        }
      });
      setState('calCache', newCache);
      return task;
    });
  }

  const deleteTask = async (id: number) => {
    const cache = state.calCache;

    await fetchDeleteTask(id);

    const newCache = structuredClone(unwrap(cache));
    const task = newCache.tasks[id];
    const month = newCache.months[toKey(task.year, task.month)];
    month.days[task.day - 1] = month.days[task.day - 1].filter((d) => d.task_id != id);
    delete newCache.tasks[id];
    setState('calCache', newCache);
  }

  return {
    getMonth,
    createTask,
    getTask,
    putTask,
    deleteTask,
    getTasksAtDate,
  };
}
