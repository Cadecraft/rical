import "./TaskPopover.css";
import { createEffect, Show, createSignal, createMemo } from "solid-js";
import { type Ridate } from "../util/ridate";
import { useCalendarState } from "../util/StateProvider";
import { useCalCache } from "../util/calCache";
import type { Task, TaskData } from "../util/types";
import { Button } from "./Button";
import { useGlobalKey } from "../util/hooks";
import HotkeyPrompt from "./HotkeyPrompt";
import { isAnythingFocusedInclButton, formatMin, parseMinString } from "../util/helpers";

import { IoTrashSharp } from "solid-icons/io";

function TimeInput(props: { min: number | undefined; setMin: (time: number | undefined) => void }) {
  const [curr, setCurr] = createSignal(formatMin(props.min));

  const parsedVal = createMemo(() => parseMinString(curr()));

  const updateInput = (newInput: string) => {
    setCurr(newInput);
  };

  const invalid = () => parsedVal() === undefined && curr().length > 0;

  const change = () => {
    props.setMin(parsedVal());
    setCurr(formatMin(parsedVal()));
  };

  return (
    <input
      value={curr()}
      onInput={(e) => updateInput(e.target.value)}
      class={`time-range ${invalid() ? "invalid" : ""}`}
      title={invalid() ? "Not a valid time" : ""}
      placeholder="00:00"
      onChange={change}
    />
  );
}

function TaskPopoverForm(props: {
  taskData: TaskData;
  loading: boolean;
  unsaved: boolean;
  setTaskData: (t: TaskData) => void;
  submit: () => void;
  cancelEdit: () => void;
  id?: number;
  deleteTask?: () => void;
}) {
  let outerRef!: HTMLDivElement;
  let newEntryRef!: HTMLInputElement;

  const [_, setState] = useCalendarState();

  const [knowOfFocus, setKnowOfFocus] = createSignal(false);

  const focusForm = () => {
    newEntryRef.focus();
    setKnowOfFocus(true);
  };

  const isNewTask = () => !props.id;

  const focusedInside = () => {
    return isAnythingFocusedInclButton() && outerRef.contains(document.activeElement);
  };

  createEffect(() => {
    if (isNewTask()) {
      focusForm();
    }
  }, [props.taskData.day, props.taskData.year, props.taskData.month, props.id]);

  const toggleComplete = () => {
    props.setTaskData({ ...props.taskData, complete: !props.taskData.complete });
    props.submit();
  };

  useGlobalKey(() => {
    if (focusedInside()) {
      const userFocusingTextarea = document.activeElement?.tagName === "TEXTAREA";
      if (props.unsaved && !props.loading && !userFocusingTextarea) {
        props.submit();
      }
    }
  }, "Enter");

  useGlobalKey(() => {
    setKnowOfFocus(false);
    if (focusedInside() && document.activeElement instanceof HTMLElement && !isNewTask()) {
      // Escape out of editing
      props.cancelEdit();
      document.activeElement?.blur();
    } else {
      setState("selection", { type: "precise-day", day: props.taskData.day });
    }
  }, "Escape");

  // TODO: capture and autoformat times
  // TODO: better way to determine whether the selected day is near the bottom
  const dayIsNearBottom = () => props.taskData.day > 21;

  return (
    <div
      ref={outerRef}
      class={`task-popover ${props.taskData.complete ? "complete" : ""} ${dayIsNearBottom() ? "from-top" : ""}`}
    >
      <div class="form-body">
        <div class="top-options">
          <Show when={!isNewTask()}>
            <Button hotkey="#" onClick={() => props.deleteTask?.()} class="icon-button">
              <IoTrashSharp size={18} />
            </Button>
          </Show>
          <div class="title-form">
            <Show when={!knowOfFocus()}>
              <HotkeyPrompt hotkey={"Enter"} onActivated={focusForm} actionDescr="Focus edit box" />
            </Show>
            <input
              onInput={(e) => props.setTaskData({ ...props.taskData, title: e.target.value })}
              ref={newEntryRef}
              placeholder="New Entry"
              value={props.taskData.title}
              title={props.taskData.title}
            />
          </div>
        </div>
        <div class="time-form">
          <TimeInput
            min={props.taskData.start_min}
            setMin={(newMin) => props.setTaskData({ ...props.taskData, start_min: newMin })}
          />
          to
          <TimeInput
            min={props.taskData.end_min}
            setMin={(newMin) => props.setTaskData({ ...props.taskData, end_min: newMin })}
          />
        </div>
        <textarea
          onInput={(e) => props.setTaskData({ ...props.taskData, description: e.target.value })}
          class="description"
          placeholder="Description"
        >
          {props.taskData.description}
        </textarea>
        <div class="bottom-options">
          <Button disabled={!props.unsaved || props.loading} onClick={props.submit}>
            {props.id ? "Save Changes" : "Create"}
          </Button>
          <Show when={!isNewTask() && !props.unsaved}>
            <Button hotkey="d" disabled={props.loading} onClick={toggleComplete}>
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
    setOriginalTask(undefined);
    calCache.getTask(props.taskId).then((res) => {
      setTask(res);
      setOriginalTask(res);
    });
    setLoading(false);
  }, [props.taskId]);

  const [_, setState] = useCalendarState();

  const unsaved = () =>
    (!!task() || !!originalTask()) && JSON.stringify(task()) !== JSON.stringify(originalTask());

  const updateTask = () => {
    if (!task()) {
      return;
    }
    setLoading(true);
    calCache
      .putTask(task()!)
      .then(() => {})
      .catch((_err) => {
        // TODO: catch
        setLoading(false);
      });
  };

  const deleteTask = () => {
    setLoading(true);
    const returnToDay = calCache.getTaskUnsafe(props.taskId).day;
    calCache.deleteTask(props.taskId);
    setState("selection", { type: "precise-day", day: returnToDay });
  };

  const cancelEdit = () => {
    setTask(originalTask);
  };

  return (
    <Show when={task()}>
      {(task) => (
        <TaskPopoverForm
          unsaved={unsaved()}
          taskData={task()}
          setTaskData={setTask}
          loading={loading()}
          submit={updateTask}
          id={props.taskId}
          deleteTask={deleteTask}
          cancelEdit={cancelEdit}
        />
      )}
    </Show>
  );
}

export function NewTaskPopover(props: { date: Ridate }) {
  const [_, setState] = useCalendarState();
  const calCache = useCalCache();
  const [currTask, setCurrTask] = createSignal<TaskData>({
    title: "",
    description: "",
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

    calCache
      .createTask(currTask())
      .then(() => {
        setState("selection", { type: "precise-day", day: props.date.dayOfMonth });
      })
      .catch((_err) => {
        // TODO: catch
        setLoading(false);
      });
  };

  return (
    <TaskPopoverForm
      unsaved={true}
      taskData={currTask()}
      setTaskData={setCurrTask}
      loading={loading()}
      submit={createTask}
      cancelEdit={() => {}}
    />
  );
}
