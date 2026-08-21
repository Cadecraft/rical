import './Calendar.css';
import { For, createEffect, createMemo, Show } from 'solid-js';
import { useSearchParams } from '@solidjs/router';
import { type Ridate, monthName, eq, weekdayName, getCalendarFrame, currentDate, leadingZero } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { DAYS_PER_WEEK } from '../util/constants';
import { useCalCache } from '../util/calCache';
import type { Task } from '../util/types';

type DateData = {
  date: Ridate;
  tasks: Task[];
};

function TaskPopover(props: { task: Task }) {
  // TODO: display popover to the left if it's on a saturday, and top if it's bottom week
  // TODO: interaction: marking complete, etc.

  return (
    <div class={`task-popover ${props.task.complete ? 'complete' : ''}`}>
      <h3>{props.task.title}</h3>
      <textarea class="description" placeholder="Description">{props.task.description}</textarea>
    </div>
  );
}

function TaskTile(props: { task: Task }) {
  const [state, setState] = useCalendarState();

  const selected = () => state.selectedTaskId === props.task.task_id;
  const toggleSelected = () => {
    setState('selectedTaskId', selected() ? undefined : props.task.task_id);
  }

  return (
    <div class="task-tile">
      <button
        class={`task ${props.task.complete ? 'complete' : ''} ${selected() ? 'selected' : ''}`}
        onClick={toggleSelected}
      >
        {props.task.title}
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
        <button class="new-task">
          +
        </button>
      </div>
    </div>
  );
}

function MonthView() {
  const [state, _] = useCalendarState();

  const calCache = useCalCache();

  createEffect(() => {
    console.log(`[DBG] Rerendering month view`);
    calCache.getMonth(state.selectedYear, state.selectedMonth).then((c) => {
      console.log(c);
    });
  });

  const days = createMemo(() => {
    const frame = getCalendarFrame(state.selectedYear, state.selectedMonth);
    const frameAugmented: DateData[][] = frame.map(week => week.map(date => ({
      date,
      tasks: []
    })));
    const res = frameAugmented.flat();
    // TODO: remove dummy tasks
    res[25].tasks = [
      { title: 'clean', complete: false, task_id: 1 },
      { title: 'debug code', complete: false, description: 'Debug stuff in the codebase', task_id: 2 },
      { title: 'some long task that idk what to do', complete: true, task_id: 3 },
    ]
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
