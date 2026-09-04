import {
  fetchCalMonth,
  fetchCreateTask,
  fetchTask,
  fetchPutTask,
  fetchDeleteTask,
} from "./apiInterface";
import type { TaskData, Task } from "./types";
import { useCalendarState } from "./StateProvider";
import { unwrap } from "solid-js/store";
import { type Ridate } from "./ridate";
import { getCalendarFrame } from '../util/ridate';

export type DateData = {
  date: Ridate;
  tasks: Task[];
};

export function useCalCache() {
  const [state, setState] = useCalendarState();

  const toKey = (year: number, month: number) => {
    return `${year}/${month}`;
  };

  const getMonth = async (year: number, month: number) => {
    const cache = state.calCache;

    const key = toKey(year, month);
    if (key in cache.months) {
      return Promise.resolve(cache.months[key]);
    }

    const res = await fetchCalMonth(year, month);
    console.log(`[DBG] fetched month: ${JSON.stringify(res)}`);
    const newCache = structuredClone(unwrap(cache));
    newCache.months[key] = res;
    for (const day of res.days) {
      for (const task of day) {
        newCache.tasks[task.task_id] = task;
      }
    }
    setState("calCache", newCache);
    return res;
  };

  /** Get the calendar frame for the current month, including the prev and next month's margin days */
  const getMonthFrame = async (year: number, month: number) => {
    const res = await Promise.all([
      getMonth(month === 1 ? year - 1 : year, month === 1 ? 12 : month - 1),
      getMonth(year, month),
      getMonth(month === 12 ? year + 1 : year, month === 12 ? 1 : month + 1),
    ]);

    const frame = getCalendarFrame(year, month);

    const frameAugmented: DateData[][] = frame.map((week) =>
      week.map((date) => ({
        date,
        tasks: (res[date.month - month + 1].days[date.dayOfMonth - 1])
      })),
    );

    return frameAugmented;
  }

  const getTasksAtDate = (date: Ridate) => {
    const cache = state.calCache;
    const key = toKey(date.year, date.month);
    return cache.months[key].days[date.dayOfMonth - 1];
  };

  const createTask = async (taskData: TaskData) => {
    const cache = state.calCache;

    fetchCreateTask(taskData).then((res) => {
      const key = toKey(taskData.year, taskData.month);
      const newCache = structuredClone(unwrap(cache));
      const createdTask = { ...taskData, task_id: res.task_id };
      newCache.months[key].days[taskData.day - 1].push(createdTask);
      newCache.tasks[res.task_id] = createdTask;
      setState("calCache", newCache);
    });
  };

  const getTask = async (id: number) => {
    const cache = state.calCache;

    if (id in cache.tasks) {
      return cache.tasks[id];
    }

    const task = await fetchTask(id);
    const newCache = structuredClone(unwrap(cache));
    newCache.tasks[id] = task;
    setState("calCache", newCache);
    return task;
  };

  const getTaskUnsafe = (id: number) => {
    if (!(id in state.calCache.tasks)) {
      console.error(`getTaskUnsafe failed for task id ${id}`);
    }
    return state.calCache.tasks[id];
  };

  const putTask = async (task: Task) => {
    const cache = state.calCache;

    fetchPutTask(task).then(() => {
      const newCache = structuredClone(unwrap(cache));
      newCache.tasks[task.task_id] = task;
      // TODO: moving tasks
      newCache.months[toKey(task.year, task.month)].days[task.day - 1].forEach((t) => {
        if (t.task_id === task.task_id) {
          Object.assign(t, task);
        }
      });
      setState("calCache", newCache);
      return task;
    });
  };

  const deleteTask = async (id: number) => {
    const cache = state.calCache;

    await fetchDeleteTask(id);

    const newCache = structuredClone(unwrap(cache));
    const task = newCache.tasks[id];
    const month = newCache.months[toKey(task.year, task.month)];
    month.days[task.day - 1] = month.days[task.day - 1].filter((d) => d.task_id != id);
    delete newCache.tasks[id];
    setState("calCache", newCache);
  };

  return {
    getMonthFrame,
    createTask,
    getTask,
    putTask,
    deleteTask,
    getTasksAtDate,
    getTaskUnsafe,
  };
}
