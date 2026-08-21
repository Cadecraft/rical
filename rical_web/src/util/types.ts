export type Task = {
  year: number,
  month: number,
  day: number,
  // TODO: these could be null too, from the api
  start_min?: number,
  end_min?: number,
  title: string;
  description?: string;
  complete: boolean;
  task_id: number;
}

export type TaskData = Omit<Task, 'task_id'>;

export type Calendar = {
  days: Task[][],
};
