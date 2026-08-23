import './TaskPopover.css';
import { createEffect, Show, createSignal } from 'solid-js';
import { type Ridate } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { useCalCache } from '../util/calCache';
import type { Task, TaskData } from '../util/types';
import { Button } from './Button';
import { useGlobalKey } from '../util/hooks';
import HotkeyPrompt from './HotkeyPrompt';

function TaskPopoverForm(props: {
  taskData: TaskData,
  loading: boolean,
  unsaved: boolean,
  setTaskData: (t: TaskData) => void,
  submit: () => void, id?: number
}) {
  let outerRef!: HTMLDivElement;
  let newEntryRef!: HTMLInputElement;

  const focusForm = () => newEntryRef.focus();

  const isNewTask = () => !props.id;

  createEffect(() => {
    if (isNewTask()) {
      focusForm();
    }
  }, [props.taskData.day, props.taskData.year, props.taskData.month, props.id]);

  const toggleComplete = () => {
    props.setTaskData({...props.taskData, complete: !props.taskData.complete});
    props.submit();
  }

  useGlobalKey(() => {
    if (props.unsaved && !props.loading) {
      props.submit();
    }
  }, 'Enter');

  // TODO: deletion, capture and autoformat times

  return (
    <div ref={outerRef} class={`task-popover ${props.taskData.complete ? 'complete' : ''}`}>
      <div class="form-body">
        <div class="top-options">
          <div>
            Delete
          </div>
          <div class="title-form">
            <HotkeyPrompt hotkey={'e'} onActivated={focusForm} actionDescr="Focus edit box" />
            <input
              onInput={(e) => props.setTaskData({...props.taskData, title: e.target.value})}
              ref={newEntryRef}
              placeholder="New Entry"
              autofocus value={props.taskData.title}
            />
          </div>
        </div>
        <div class="time-form">
          <input placeholder="00:00" />
          to
          <input placeholder="00:00" />
        </div>
        <textarea onChange={(e) => props.setTaskData({...props.taskData, description: e.target.value})} class="description" placeholder="Description">{props.taskData.description}</textarea>
        <div class="bottom-options">
          <Button disabled={!props.unsaved || props.loading} onClick={props.submit}>{props.id ? "Save Changes" : "Create"}</Button>
          <Show when={!isNewTask() && !props.unsaved}>
            <Button
              hotkey="d"
              disabled={props.loading}
              onClick={toggleComplete}
            >
              {props.taskData.complete ? "Mark Not Done" : "Mark Done"}
            </Button>
          </Show>
        </div>
      </div>
    </div>
  );
}

export function ExistingTaskPopover(props: { taskId: number }) {
  const calCache = useCalCache();
  const [task, setTask] = createSignal<Task | undefined>(undefined);
  const [originalTask, setOriginalTask] = createSignal<Task | undefined>(undefined);
  const [loading, setLoading] = createSignal(true);

  createEffect(() => {
    setTask(undefined);
    setOriginalTask(undefined)
    calCache.getTask(props.taskId).then(res => {
      setTask(res);
      setOriginalTask(res);
    });
    setLoading(false);
  }, [props.taskId]);

  const [_, setState] = useCalendarState();

  useGlobalKey(() => {
    setState('selectedTask', undefined);
  }, 'Escape');

  const unsaved = () => (!!task() || !!originalTask()) && JSON.stringify(task()) !== JSON.stringify(originalTask());

  const updateTask = () => {
    if (!task()) {
      return;
    }
    setLoading(true);
    calCache.putTask(task()!).then(() => {
    }).catch((err) => {
      // TODO: catch
      setLoading(false);
    });
  };

  return (
    <Show when={task()}>{(task) => (
      <TaskPopoverForm unsaved={unsaved()} taskData={task()} setTaskData={setTask} loading={loading()} submit={updateTask} id={props.taskId} />
    )}</Show>
  );
}

export function NewTaskPopover(props: { date: Ridate }) {
  const [_, setState] = useCalendarState();
  const calCache = useCalCache();
  const [currTask, setCurrTask] = createSignal<TaskData>({
    title: '',
    description: '',
    complete: false,
    year: props.date.year,
    month: props.date.month,
    day: props.date.dayOfMonth,
    start_min: undefined,
    end_min: undefined,
  });
  const [loading, setLoading] = createSignal(false);

  const createTask = () => {
    setLoading(true);

    calCache.createTask(currTask()).then(() => {
      setState('selectedTask', undefined);
    }).catch((err) => {
      // TODO: catch
      setLoading(false);
    });
  }

  useGlobalKey(() => {
    setState('selectedTask', undefined);
  }, 'Escape');

  // TODO: allow editing of fields

  return (
    <TaskPopoverForm unsaved={true} taskData={currTask()} setTaskData={setCurrTask} loading={loading()} submit={createTask} />
  );
}
