import './Calendar.css';
import { For, createEffect, createMemo, Show, createSignal } from 'solid-js';
import { useSearchParams } from '@solidjs/router';
import { type Ridate, monthName, eq, weekdayName, getCalendarFrame, currentDate, leadingZero } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { DAYS_PER_WEEK } from '../util/constants';
import { useCalCache } from '../util/calCache';
import type { Task, TaskData, Calendar } from '../util/types';
import { Button } from '../components/Button';
import { useGlobalKey } from '../util/hooks';

type DateData = {
  date: Ridate;
  tasks: Task[];
};

function TaskPopover(props: { task: Task }) {
  // TODO: display popover to the left if it's on a saturday, and top if it's bottom week
  // TODO: interaction: marking complete, etc.
  // TODO: close with esc
  const [_, setState] = useCalendarState();

  useGlobalKey(() => {
    setState('selectedTask', undefined);
  }, 'Escape');

  return (
    <div class={`task-popover ${props.task.complete ? 'complete' : ''}`}>
      <h3>{props.task.title}</h3>
      <textarea class="description" placeholder="Description">{props.task.description}</textarea>
    </div>
  );
}

function NewTaskPopover(props: { date: Ridate }) {
  // TODO: display popover to the left if it's on a saturday, and top if it's bottom week
  // TODO: interaction: marking complete, etc.
  // TODO: refactor to reuse code from normal task popover
  let newEntryRef!: HTMLInputElement;

  const [_, setState] = useCalendarState();
  const calCache = useCalCache();

  createEffect(() => {
    newEntryRef.focus();
  });

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
    });
  }

  useGlobalKey(() => {
    setState('selectedTask', undefined);
  }, 'Escape');

  // TODO: allow editing of fields

  return (
    <div class={`task-popover`}>
      <input onChange={(e) => setCurrTask({...currTask(), title: e.target.value})} ref={newEntryRef} placeholder="New Entry" autofocus>{currTask().title}</input>
      <textarea onChange={(e) => setCurrTask({...currTask(), description: e.target.value})} class="description" placeholder="Description">{currTask().description}</textarea>
      <Button disabled={loading()} onClick={createTask}>Create</Button>
    </div>
  );
}

function NewTaskButton(props: { date: Ridate }) {
  const [state, setState] = useCalendarState();

  const creatingNewTaskDate = () => (state.selectedTask && state.selectedTask.newTask) ? state.selectedTask.date : undefined;
  const startNewTask = () => {
    setState('selectedTask', creatingNewTaskDate() ? undefined : { newTask: true, date: props.date });
  }

  return (
    <>
      <button class="new-task" onClick={startNewTask}>
        +
      </button>
      <Show when={creatingNewTaskDate() && eq(creatingNewTaskDate()!, props.date)}>
        <NewTaskPopover date={props.date} />
      </Show>
    </>
  );
}

function TaskTile(props: { task: Task }) {
  const [state, setState] = useCalendarState();

  const selected = () => state.selectedTask && !state.selectedTask.newTask && state.selectedTask?.id === props.task.task_id;
  const toggleSelected = () => {
    setState('selectedTask', selected() ? undefined : { newTask: false, id: props.task.task_id });
  }

  return (
    <div class="task-tile">
      <button
        class={`task ${props.task.complete ? 'complete' : ''} ${selected() ? 'selected' : ''}`}
        onClick={toggleSelected}
      >
        {props.task.title || <>&nbsp;</>}
      </button>
      <Show when={selected()}>
        <TaskPopover task={props.task} />
      </Show>
    </div>
  )
}

function MonthDay(props: { day: DateData }) {
  const [state, _] = useCalendarState();
  const isToday = () => eq(currentDate(), props.day.date);

  const dayOfMonthDisp = () => {
    const dayOfMonth = props.day.date.dayOfMonth;
    if (state.selectedMonth === props.day.date.month) {
      return dayOfMonth.toString();
    } else {
      return `${monthName(props.day.date)} ${dayOfMonth}`
    }
  }

  return (
    <div class={`month-day ${isToday() ? 'today' : ''}`}>
      <div class="month-day-top">
        <div class="day-of-month">{dayOfMonthDisp()}</div>
      </div>
      <div class="tasks">
        <For each={props.day.tasks}>
          {(task) => <TaskTile task={task} />}
        </For>
        <NewTaskButton date={props.day.date} />
      </div>
    </div>
  );
}

function MonthView() {
  const [state, _] = useCalendarState();
  const [monthFromApi, setMonthFromApi] = createSignal<Calendar | undefined>(undefined);

  const calCache = useCalCache();

  createEffect(() => {
    console.log(`[DBG] Rerendering month view`);
    setMonthFromApi(undefined);

    calCache.getMonth(state.selectedYear, state.selectedMonth).then((c) => {
      setMonthFromApi(c);
    });
  });

  const days = createMemo(() => {
    console.log('rerendering days');
    const frame = getCalendarFrame(state.selectedYear, state.selectedMonth);
    const frameAugmented: DateData[][] = frame.map(week => week.map(date => ({
      date,
      tasks: (monthFromApi() && date.month === state.selectedMonth) ? monthFromApi()!.days[date.dayOfMonth - 1] : [],
    })));
    const res = frameAugmented.flat();
    return res;
  });

  return (
    <div class="month-view">
      <div class="month-view-weekdays">
        <For each={days().slice(0, DAYS_PER_WEEK)}>
          {(day) => (
            <div>{weekdayName(day.date)}</div>
          )}
        </For>
      </div>
      <div class="month-view-grid">
        <For each={days()}>
          {(day) => (
            <MonthDay day={day} />
          )}
        </For>
      </div>
    </div>
  );
}

function TopBar() {
  const [state] = useCalendarState();
  const [_, setParams] = useSearchParams();

  const prevMonth = () => {
    setParams({
      y: state.selectedMonth === 1 ? state.selectedYear - 1 : state.selectedYear,
      m: state.selectedMonth === 1 ? 12 : state.selectedMonth - 1,
    });
  }

  const nextMonth = () => {
    setParams({
      y: state.selectedMonth === 12 ? state.selectedYear + 1 : state.selectedYear,
      m: state.selectedMonth === 12 ? 1 : state.selectedMonth + 1,
    });
  }

  return (
    <div class="top-bar">
      <img src="/RicalIcon.svg" alt="Rical Home" />
      <div class="month-select">
        <button onClick={prevMonth}>Prev</button>
        <h2>{state.selectedYear}/{leadingZero(state.selectedMonth)}</h2>
        <button onClick={nextMonth}>Next</button>
      </div>
    </div>
  );
}

function Page() {
  const [params] = useSearchParams();
  const [_, setState] = useCalendarState();

  // TODO: kick out un-authed users

  createEffect(() => {
    const year = () => Number(params.y || currentDate().year);
    const month = () => Number(params.m || currentDate().month);
    
    setState('selectedYear', year());
    setState('selectedMonth', month());
  });

  return (
    <div class="cal-root">
      <TopBar />
      <div class="main-cal">
        <MonthView />
      </div>
    </div>
  );
}

export default Page;
