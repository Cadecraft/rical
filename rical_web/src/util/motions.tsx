import { createEffect, onCleanup } from "solid-js";
import { useCalendarState } from "./StateProvider";
import { type Ridate, addDays } from "./ridate";
import { useDateNav } from "./hooks";
import { isAnythingFocused, compareTasks } from "./helpers";
import { useCalCache } from "./calCache";

function useMotions() {
  const [state, setState] = useCalendarState();
  const { toYearMonth } = useDateNav();
  const calCache = useCalCache();

  const selectedDate = (): Ridate | undefined => {
    let selectedDay = -1;
    if (state.selection.type === "vibing-day" || state.selection.type === "precise-day") {
      selectedDay = state.selection.day;
    } else if (state.selection.type === "task" && state.selection.newTask) {
      selectedDay = state.selection.date.dayOfMonth;
    } else if (state.selection.type === "task" && state.selection.newTask === false) {
      selectedDay = calCache.getTaskUnsafe(state.selection.id).day;
    } else {
      console.error("[err] Motions selected day could not be determined");
    }

    return {
      year: state.selectedYear,
      month: state.selectedMonth,
      dayOfMonth: selectedDay,
    };
  };

  const defaultSelectedDate = (): Ridate => {
    return {
      year: state.selectedYear,
      month: state.selectedMonth,
      // TODO: better logic. Ideally 'today', or maybe day with the selected task
      dayOfMonth: 15,
    };
  };

  const pseudoSelectedDate = () => selectedDate() ?? defaultSelectedDate();

  const selectDate = (date: Ridate) => {
    if (date.year !== state.selectedYear || date.month !== state.selectedMonth) {
      toYearMonth(date.year, date.month);
    }
    setState("selection", { type: "precise-day", day: date.dayOfMonth });
  };

  const navTasks = (direction: 1 | -1) => {
    const date = selectedDate();
    if (!date) return;

    const tasksHere = calCache.getTasksAtDate(date);
    const sorted = tasksHere.toSorted(compareTasks);
    if (sorted.length === 0) {
      return;
    }
    const existingSelectedTaskId =
      state.selection.type === "task" && !state.selection.newTask ? state.selection.id : undefined;
    const selectedHere = sorted.findIndex((t) => t.task_id === existingSelectedTaskId);
    if (selectedHere === undefined) {
      setState("selection", { type: "task", newTask: false, id: sorted[0].task_id });
      return;
    }
    const newSelected = selectedHere + direction;
    const newSelectedInRange =
      newSelected < 0 ? sorted.length - 1 : newSelected >= sorted.length ? 0 : newSelected;
    const newSelectedId = sorted[newSelectedInRange].task_id;
    setState("selection", { type: "task", newTask: false, id: newSelectedId });
  };

  const motionMap: Record<string, () => void> = {
    h: () => {
      selectDate(addDays(pseudoSelectedDate(), -1));
    },
    l: () => {
      selectDate(addDays(pseudoSelectedDate(), 1));
    },
    j: () => {
      if (state.selection.type === "task") {
        navTasks(1);
      } else {
        selectDate(addDays(pseudoSelectedDate(), 7));
      }
    },
    k: () => {
      if (state.selection.type === "task") {
        navTasks(-1);
      } else {
        selectDate(addDays(pseudoSelectedDate(), -7));
      }
    },
    o: () => {
      const date = selectedDate();
      if (date) {
        setState("selection", { type: "task", newTask: true, date });
      }
    },
    Enter: () => {
      if (state.selection.type !== "task") {
        const date = selectedDate();
        if (date) {
          navTasks(1);
        }
      }
    },
    Escape: () => {
      if (state.selection.type === "precise-day") {
        setState("selection", { type: "vibing-day", day: state.selection.day });
      }
    },
  };

  createEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (e.key in motionMap && !e.ctrlKey) {
        e.preventDefault();
        motionMap[e.key]();
      }
    };

    window.addEventListener("keydown", handleKeyDown);

    onCleanup(() => {
      window.removeEventListener("keydown", handleKeyDown);
    });
  });
}

export default useMotions;
