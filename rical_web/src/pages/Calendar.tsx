import "./Calendar.css";
import { For, createEffect, createMemo, Show, createSignal, onCleanup } from "solid-js";
import { useSearchParams } from "@solidjs/router";
import {
  type Ridate,
  monthName,
  eq,
  weekdayName,
  getCalendarFrame,
  currentDate,
} from "../util/ridate";
import { useCalendarState } from "../util/StateProvider";
import { DAYS_PER_WEEK } from "../util/constants";
import { useCalCache, type DateData } from "../util/calCache";
import type { Task } from "../util/types";
import { NewTaskPopover, ExistingTaskPopover } from "../components/TaskPopover";
import useMotions from "../util/motions";
import { useDateNav, useGlobalKey } from "../util/hooks";
import { leadingZero, compareTasks, formatMin } from "../util/helpers";
import { fetchWhoami } from "../util/apiInterface";
import { IoPersonCircleSharp } from "solid-icons/io";
import { Button } from "../components/Button";

import { IoArrowBackSharp, IoArrowForwardSharp } from "solid-icons/io";

function NewTaskButton(props: { date: Ridate }) {
  const [state, setState] = useCalendarState();

  const creatingNewTaskDate = () =>
    state.selection.type === "task" && state.selection.newTask ? state.selection.date : undefined;
  const startNewTask = () => {
    if (creatingNewTaskDate() && eq(creatingNewTaskDate()!, props.date)) {
      setState("selection", { type: "vibing-day", day: props.date.dayOfMonth });
    } else {
      setState("selection", { type: "task", newTask: true, date: props.date });
    }
  };

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

  const selected = () =>
    state.selection.type === "task" &&
    !state.selection.newTask &&
    state.selection.id === props.task.task_id;
  const toggleSelected = () => {
    if (selected()) {
      setState("selection", { type: "vibing-day", day: props.task.day });
    } else {
      setState("selection", { type: "task", newTask: false, id: props.task.task_id });
      // TODO: automatically transport us (probably down near calendar) to the proper year and month if they go out of range
      toYearMonth(props.task.year, props.task.month);
    }
  };

  const formattedTitle = () => {
    return props.task.start_min
      ? `${formatMin(props.task.start_min, true)} ${props.task.title}`
      : `${props.task.title}`;
  };

  return (
    <div class="task-tile">
      <button
        class={`task ${props.task.complete ? "complete" : ""} ${selected() ? "selected" : ""}`}
        onClick={toggleSelected}
      >
        {formattedTitle() || <>&nbsp;</>}
      </button>
    </div>
  );
}

function MonthDay(props: { day: DateData }) {
  const [state, setState] = useCalendarState();
  const isToday = () => eq(currentDate(), props.day.date);
  const isSelected = () =>
    state.selectedMonth === props.day.date.month &&
    state.selection.type === "precise-day" &&
    state.selection.day === props.day.date.dayOfMonth;

  const dayOfMonthDisp = () => {
    const dayOfMonth = props.day.date.dayOfMonth;
    if (state.selectedMonth === props.day.date.month) {
      return dayOfMonth.toString();
    } else {
      return `${monthName(props.day.date)} ${dayOfMonth}`;
    }
  };

  const clickDay = () => {
    if (state.selection.type === "precise-day" || state.selection.type === "vibing-day") {
      setState("selection", { type: "vibing-day", day: props.day.date.dayOfMonth });
    }
  };

  const sortedTasks = createMemo(() => {
    return props.day.tasks.toSorted(compareTasks);
  });

  // TODO: document 'o' for new task hotkey, only when this day is selected?

  return (
    <div
      class={`month-day ${isToday() ? "today" : ""} ${isSelected() ? "selected" : ""} ${state.selection.type === "task" ? "task-focused" : ""}`}
      onClick={clickDay}
    >
      <div class="month-day-top">
        <div class="day-of-month">{dayOfMonthDisp()}</div>
      </div>
      <div class="tasks">
        <For each={sortedTasks()}>{(task) => <TaskTile task={task} />}</For>
        <NewTaskButton date={props.day.date} />
      </div>
    </div>
  );
}

function MonthView() {
  const [state, _] = useCalendarState();
  const [frameFromApi, setFrameFromApi] = createSignal<DateData[][] | undefined>(undefined);

  const calCache = useCalCache();

  createEffect(() => {
    console.log(`[DBG] Rerendering month view`);
    setFrameFromApi(undefined);

    calCache.getMonthFrame(state.selectedYear, state.selectedMonth).then((c) => {
      setFrameFromApi(c);
    });
  });

  const days = createMemo(() => {
    if (frameFromApi()) {
      return frameFromApi()!.flat();
    }
    const frame = getCalendarFrame(state.selectedYear, state.selectedMonth);
    const emptyFrame: DateData[][] = frame.map((week) =>
      week.map((date) => ({
        date,
        tasks: []
      })),
    );
    return emptyFrame.flat();
  });

  return (
    <div class="month-view">
      <div class="month-view-weekdays">
        <For each={days().slice(0, DAYS_PER_WEEK)}>
          {(day) => <div>{weekdayName(day.date)}</div>}
        </For>
      </div>
      <div class="month-view-grid">
        <For each={days()}>{(day) => <MonthDay day={day} />}</For>
      </div>
    </div>
  );
}

function TopBar() {
  const [state] = useCalendarState();
  const { prevMonth, nextMonth } = useDateNav();

  const [whoami, setWhoami] = createSignal<undefined | { username: string }>(undefined);

  // eslint-disable-next-line no-unassigned-vars
  let whoamiElem!: HTMLDivElement;

  createEffect(() => {
    fetchWhoami()
      .then((res) => {
        setWhoami(res);
      })
      .catch(() => {
        // Kick the user back to login
        location.href = "/login?reason=unauthorized";
      });
  });

  const [visible, setVisible] = createSignal(false);

  const signOut = () => {
    // TODO: localStorage is temporary. Switch to cookie
    localStorage.removeItem("tok");
    location.href = "/login?reason=loggedout";
  };

  createEffect(() => {
    const onClick = (e: MouseEvent) => {
      if (visible() && e.target instanceof Element && !whoamiElem.contains(e.target)) {
        setVisible(false);
      }
    };

    document.addEventListener("click", onClick);
    onCleanup(() => {
      document.removeEventListener("click", onClick);
    });
  });

  useGlobalKey(() => setVisible(false), "Escape");

  return (
    <div class="top-bar">
      <img src="/RicalIcon.svg" alt="Rical Home" />
      <div class="month-select">
        <button onClick={prevMonth}>
          <IoArrowBackSharp />
        </button>
        <h2>
          {state.selectedYear}/{leadingZero(state.selectedMonth)}
        </h2>
        <button onClick={nextMonth}>
          <IoArrowForwardSharp />
        </button>
      </div>
      <div class="whoami" ref={whoamiElem}>
        <button class="icon" onClick={() => setVisible((v) => !v)}>
          <IoPersonCircleSharp />
        </button>
        <Show when={visible()}>
          <div class="whoami-popover">
            Signed in as {whoami()?.username ?? "..."}
            <Button onClick={signOut} hotkey="q">
              Sign out
            </Button>
          </div>
        </Show>
      </div>
    </div>
  );
}

function Page() {
  const [params] = useSearchParams();
  const [state, setState] = useCalendarState();

  // TODO: fix creating/clicking tasks in the top/bottom rows outside of this month (focus should be allowed to be on diff day)

  createEffect(() => {
    const year = () => Number(params.y || currentDate().year);
    const month = () => Number(params.m || currentDate().month);

    setState("selectedYear", year());
    setState("selectedMonth", month());
  });

  useMotions();

  return (
    <div class="cal-root">
      <TopBar />
      <div class="main-cal">
        <MonthView />
      </div>
      <Show
        when={
          state.selection.type === "task" && state.selection.newTask === false && state.selection.id
        }
      >
        {(id) => <ExistingTaskPopover taskId={id()} />}
      </Show>
      <Show
        when={
          state.selection.type === "task" &&
          state.selection.newTask === true &&
          state.selection.date
        }
      >
        {(date) => <NewTaskPopover date={date()} />}
      </Show>
    </div>
  );
}

export default Page;
