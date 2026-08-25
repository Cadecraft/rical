import { createEffect, onCleanup } from 'solid-js';
import { useCalendarState } from './StateProvider';
import { type Ridate, addDays } from './ridate';
import { useDateNav } from './hooks';
import { isAnythingFocused } from './helpers';
import { useCalCache } from './calCache';

function useMotions() {
  const [state, setState] = useCalendarState();
  const { toYearMonth } = useDateNav();
  const calCache = useCalCache();

  const selectedDate = (): Ridate | undefined => {
    if (!state.selectedDay) {
      return undefined;
    }

    return {
      year: state.selectedYear,
      month: state.selectedMonth,
      dayOfMonth: state.selectedDay,
    }
  }

  const defaultSelectedDate = (): Ridate => {
    return {
      year: state.selectedYear,
      month: state.selectedMonth,
      // TODO: better logic. Ideally 'today', or maybe day with the selected task
      dayOfMonth: 15,
    }
  }

  const pseudoSelectedDate = () => selectedDate() ?? defaultSelectedDate();

  const selectDate = (date: Ridate) => {
    if (date.year !== state.selectedYear || date.month !== state.selectedMonth) {
      toYearMonth(date.year, date.month);
    }
    setState('selectedDay', date.dayOfMonth);
  }

  const navTasks = (direction: 1 | -1) => {
    const date = selectedDate();
    if (!date) return;

    const tasksHere = calCache.getTasksAtDate(date);
    if (tasksHere.length === 0) {
      setState('selectedTask', undefined);
      return;
    }
    const selectedTask = state.selectedTask;
    const selectedHere = (!selectedTask || selectedTask.newTask) ? undefined : tasksHere.findIndex((t) => t.task_id === selectedTask.id);
    if (selectedHere === undefined) {
      setState('selectedTask', { newTask: false, id: tasksHere[0].task_id });
      return;
    }
    const newSelected = selectedHere + direction;
    const newSelectedInRange = newSelected < 0 ? tasksHere.length - 1 : (newSelected >= tasksHere.length ? 0 : newSelected);
    const newSelectedId = tasksHere[newSelectedInRange].task_id;
    setState('selectedTask', { newTask: false, id: newSelectedId });
  }

  const motionMap: Record<string, () => void> = {
    'h': () => {
      selectDate(addDays(pseudoSelectedDate(), -1));
      setState('selectedTask', undefined);
    },
    'l': () => {
      selectDate(addDays(pseudoSelectedDate(), 1));
      setState('selectedTask', undefined);
    },
    'j': () => {
      if (state.selectedTask) {
        navTasks(1);
      } else {
        selectDate(addDays(pseudoSelectedDate(), 7));
      }
    },
    'k': () => {
      if (state.selectedTask) {
        navTasks(-1);
      } else {
        selectDate(addDays(pseudoSelectedDate(), -7));
      }
    },
    'o': () => {
      const date = selectedDate();
      if (date) {
        setState('selectedTask', { newTask: true, date });
      }
    },
    'Enter': () => {
      if (!state.selectedTask) {
        const date = selectedDate();
        if (date) {
          navTasks(1);
        }
      }
    }
  };

  createEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (e.key in motionMap) {
        e.preventDefault();
        motionMap[e.key]();
      }
    }

    window.addEventListener("keydown", handleKeyDown);

    onCleanup(() => {
      window.removeEventListener("keydown", handleKeyDown);
    });
  });
}

export default useMotions;
