import './Calendar.css';
import { For, createEffect, createMemo } from 'solid-js';
import { useSearchParams } from '@solidjs/router';
import { type Ridate, monthName, eq, weekdayName, getCalendarFrame, currentDate, leadingZero } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { DAYS_PER_WEEK } from '../util/constants';

type Task = {
  title: string;
  description?: string;
  task_id: number;
  complete: boolean;
}

type DateData = {
  date: Ridate;
  tasks: Task[];
};

function TaskTile(props: { task: Task }) {
  const [state, setState] = useCalendarState();

  return (
    <button class={`task ${props.task.complete ? 'complete' : ''}`}>
      {props.task.title}
    </button>
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
      </div>
    </div>
  );
}

function MonthView() {
  const [state, _] = useCalendarState();

  const days = createMemo(() => {
    const frame = getCalendarFrame(2026, state.selectedMonth);
    const frameAugmented: DateData[][] = frame.map(week => week.map(date => ({
      date,
      tasks: []
    })));
    const res = frameAugmented.flat();
    // TODO: remove dummy tasks
    res[27].tasks = [
      { title: 'clean', complete: false, task_id: 1 },
      { title: 'debug code', complete: false, description: 'Debug stuff in the codebase', task_id: 2 },
      { title: 'some long task that idk what to do', complete: true, task_id: 3 },
    ]
    return res;
  });

  // TODO: fix grid for 2026/08
  // TODO: fix grid for 2027/02

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
