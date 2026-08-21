export type Task = {
  title: string;
  description?: string;
  task_id: number;
  complete: boolean;
}

export type TaskData = Omit<Task, 'task_id'>;
