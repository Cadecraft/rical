import './Calendar.css';
import { For, createEffect, createMemo, Show, createSignal } from 'solid-js';
import { useSearchParams } from '@solidjs/router';
import { type Ridate, monthName, eq, weekdayName, getCalendarFrame, currentDate, leadingZero } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { DAYS_PER_WEEK } from '../util/constants';
import { useCalCache } from '../util/calCache';
import type { Task, Calendar } from '../util/types';
import { NewTaskPopover, ExistingTaskPopover } from '../components/TaskPopover';
import useMotions from '../util/motions';
import { useDateNav } from '../util/hooks';

import { IoArrowBackSharp, IoArrowForwardSharp } from 'solid-icons/io';

type DateData = {
  date: Ridate;
  tasks: Task[];
};

function NewTaskButton(props: { date: Ridate }) {
  const [state, setState] = useCalendarState();

  const creatingNewTaskDate = () => (state.selectedTask && state.selectedTask.newTask) ? state.selectedTask.date : undefined;
  const startNewTask = () => {
    setState('selectedTask', (creatingNewTaskDate() && eq(creatingNewTaskDate()!, props.date)) ? undefined : { newTask: true, date: props.date });
  }

  return (
    <>
      <button class="new-task" onClick={startNewTask}>
        +
      </button>
    </>
  );
}

function TaskTile(props: { task: Task }) {
  const [state, setState] = useCalendarState();
  const { toYearMonth } = useDateNav();

  const selected = () => state.selectedTask && !state.selectedTask.newTask && state.selectedTask?.id === props.task.task_id;
  const toggleSelected = () => {
    if (selected()) {
      setState('selectedTask', undefined);
    } else {
      setState('selectedTask', { newTask: false, id: props.task.task_id });
      toYearMonth(props.task.year, props.task.month);
      setState('selectedDay', props.task.day);
    }
  }

  return (
    <div class="task-tile">
      <button
        class={`task ${props.task.complete ? 'complete' : ''} ${selected() ? 'selected' : ''}`}
        onClick={toggleSelected}
      >
        {props.task.title || <>&nbsp;</>}
      </button>
    </div>
  )
}

function MonthDay(props: { day: DateData }) {
  const [state, _] = useCalendarState();
  const isToday = () => eq(currentDate(), props.day.date);
  const isSelected = () => state.selectedDay &&
    state.selectedDay === props.day.date.dayOfMonth &&
    state.selectedMonth === props.day.date.month;

  const dayOfMonthDisp = () => {
    const dayOfMonth = props.day.date.dayOfMonth;
    if (state.selectedMonth === props.day.date.month) {
      return dayOfMonth.toString();
    } else {
      return `${monthName(props.day.date)} ${dayOfMonth}`
    }
  }

  // TODO: document 'o' for new task hotkey, only when this day is selected?

  return (
    <div class={`month-day ${isToday() ? 'today' : ''} ${isSelected() ? 'selected' : ''} ${state.selectedTask ? 'task-focused' : ''}`}>
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
  const { prevMonth, nextMonth } = useDateNav();

  return (
    <div class="top-bar">
      <img src="/RicalIcon.svg" alt="Rical Home" />
      <div class="month-select">
        <button onClick={prevMonth}><IoArrowBackSharp /></button>
        <h2>{state.selectedYear}/{leadingZero(state.selectedMonth)}</h2>
        <button onClick={nextMonth}><IoArrowForwardSharp /></button>
      </div>
    </div>
  );
}

function Page() {
  const [params] = useSearchParams();
  const [state, setState] = useCalendarState();

  // TODO: kick out un-authed users

  createEffect(() => {
    const year = () => Number(params.y || currentDate().year);
    const month = () => Number(params.m || currentDate().month);
    
    setState('selectedYear', year());
    setState('selectedMonth', month());
  });

  useMotions();

  return (
    <div class="cal-root">
      <TopBar />
      <div class="main-cal">
        <MonthView />
      </div>
      <Show when={state.selectedTask && state.selectedTask.newTask === false && state.selectedTask.id}>{(id) => (
        <ExistingTaskPopover taskId={id()} />
      )}</Show>
      <Show when={state.selectedTask && state.selectedTask.newTask === true && state.selectedTask.date}>{(date) => (
        <NewTaskPopover date={date()} />
      )}</Show>
    </div>
  );
}

export default Page;
