import { createContext, useContext, type JSX } from "solid-js";
import { createStore, type SetStoreFunction } from "solid-js/store";
import { currentDate, type Ridate } from "../util/ridate";
import type { Calendar, Task } from "./types";

// If a user has a task selected, then the task's day is naturally also considered selected.
type SelectionType =
  | {
      // Vibing: user has clicked a day, and unconsiously considers it focused
      type: "vibing-day";
      day: number;
    }
  | {
      // Precise: user has used motions to get here
      type: "precise-day";
      day: number;
    }
  | {
      type: "task";
      newTask: true;
      date: Ridate;
    }
  | {
      type: "task";
      newTask: false;
      id: number;
    };

export type CalendarStore = {
  selection: SelectionType;
  selectedMonth: number;
  selectedYear: number;
  calCache: {
    months: Record<string, Calendar>;
    tasks: Record<number, Task>;
  };
};

const CalendarStateContext = createContext();

export function CalendarStateProvider(props: { children: JSX.Element }) {
  const [store, setStore] = createStore<CalendarStore>({
    // TODO: better default day, e.g. today
    selection: { type: "vibing-day", day: 15 },
    selectedMonth: currentDate().month,
    selectedYear: currentDate().year,
    calCache: {
      months: {},
      tasks: {},
    },
  });

  const packagedStore = [store, setStore];

  return (
    <CalendarStateContext.Provider value={packagedStore}>
      {props.children}
    </CalendarStateContext.Provider>
  );
}

export function useCalendarState(): [CalendarStore, SetStoreFunction<CalendarStore>] {
  return useContext(CalendarStateContext) as [CalendarStore, SetStoreFunction<CalendarStore>];
}
