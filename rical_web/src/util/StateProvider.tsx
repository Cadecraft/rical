import { createContext, useContext, type JSX } from 'solid-js';
import { createStore, type SetStoreFunction } from 'solid-js/store';
import { currentDate, type Ridate } from '../util/ridate';
import type { Calendar, Task } from './types';

export type CalendarStore = {
  selectedTask: { newTask: false, id: number } | { newTask: true, date: Ridate } | undefined;
  selectedMonth: number;
  selectedYear: number;
  calCache: {
    months: Record<string, Calendar>;
    tasks: Record<number, Task>;
  };
  selectedDay: number | undefined;
};

const CalendarStateContext = createContext();

export function CalendarStateProvider(props: { children: JSX.Element }) {
  const [store, setStore] = createStore<CalendarStore>({
    selectedTask: undefined,
    selectedMonth: currentDate().month,
    selectedYear: currentDate().year,
    calCache: {
      months: {},
      tasks: {},
    },
    selectedDay: undefined,
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
