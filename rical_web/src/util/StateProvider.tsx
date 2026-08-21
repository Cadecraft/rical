import { createContext, useContext, type JSX } from 'solid-js';
import { createStore, type SetStoreFunction } from 'solid-js/store';
import { currentDate, type Ridate } from '../util/ridate';
import { type Calendar } from './types';

export type CalendarStore = {
  selectedTaskId: number | undefined;
  selectedMonth: number;
  selectedYear: number;
  creatingNewTask: Ridate | undefined;
  calCache: Record<string, Calendar>;
};

const CalendarStateContext = createContext();

export function CalendarStateProvider(props: { children: JSX.Element }) {
  const [store, setStore] = createStore<CalendarStore>({
    selectedTaskId: undefined,
    selectedMonth: currentDate().month,
    selectedYear: currentDate().year,
    creatingNewTask: undefined,
    calCache: {},
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
