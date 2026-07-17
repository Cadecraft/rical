import './Calendar.css';
import { For } from 'solid-js';
import { type Ridate, monthName, eq, weekdayName, getCalendarFrame } from '../util/ridate';
import { useCalendarState } from '../util/StateProvider';
import { DAYS_PER_WEEK } from '../util/constants';

const todayDate = { year: 1990, month: 6, dayOfMonth: 30 };
const viewingMonth = 6;

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
  const selected = useCalendarState();
  console.log(selected);

  return (
    <button class={`task ${props.task.complete ? 'complete' : ''}`}>
      {props.task.title}
    </button>
  )
}

function MonthDay(props: { day: DateData }) {
  const isToday = () => eq(todayDate, props.day.date);

  const dayOfMonthDisp = () => {
    const dayOfMonth = props.day.date.dayOfMonth;
    if (viewingMonth === props.day.date.month) {
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
  const populateWeeks = () => {
    const frame = getCalendarFrame(2026, viewingMonth);
    const frameAugmented: DateData[][] = frame.map(week => week.map(date => ({
      date,
      tasks: []
    })));
    const res = frameAugmented.flat();
    // TODO: remove dummy tasks
    res[30].tasks = [
      { title: 'clean', complete: false, task_id: 1 },
      { title: 'debug code', complete: false, description: 'Debug stuff in the codebase', task_id: 2 },
      { title: 'some long task that idk what to do', complete: true, task_id: 3 },
    ]
    return res;
  };

  const days = populateWeeks();

  return (
    <div class="month-view">
      <div class="month-view-weekdays">
        <For each={days.slice(0, DAYS_PER_WEEK)}>
          {(day) => (
            <div>{weekdayName(day.date)}</div>
          )}
        </For>
      </div>
      <div class="month-view-grid">
        <For each={days}>
          {(day) => (
            <MonthDay day={day} />
          )}
        </For>
      </div>
    </div>
  );
}

function TopBar() {
  return (
    <div class="top-bar">
      <img src="/RicalIcon.svg" alt="Rical Home" />
    </div>
  );
}

function Page() {

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
