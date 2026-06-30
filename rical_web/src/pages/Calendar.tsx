import './Calendar.css';
import { For } from 'solid-js';
import { type Ridate, monthName, eq } from '../util/ridate';

const todayDate = { year: 1990, month: 6, dayOfMonth: 30 };
const viewingMonth = 6;

type Task = {
  title: string;
}

type DateData = {
  date: Ridate;
  tasks: Task[];
};

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
      <div class="day-of-month">{dayOfMonthDisp()}</div>
    </div>
  );
}

function MonthView() {
  const days: DateData[] = [];

  // TODO: remove dummy code
  const populateWeeks = () => {
    let date = 0;
    let currMonth = 5;
    for (let i = 0; i < 35; ++i) {
      // TODO: replace dummy logic with real logic
      if (date == 1 && currMonth == 5) {
        currMonth += 1;
      }
      if (date == 31) {
        date = 1;
        currMonth = 7;
      }
      days.push({
        date: {
          year: 1990,
          month: currMonth,
          dayOfMonth: date === 0 ? 31 : date,
        },
        tasks: [],
      });
      date += 1;
    }
    days[30].tasks = [
      { title: 'clean' }, { title: 'debug code' }
    ]

    console.log(days);
  };

  populateWeeks();

  return (
    <div class="month-view">
      <For each={days}>
        {(day) => (
          <MonthDay day={day} />
        )}
      </For>
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
