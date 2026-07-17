import { createContext, useContext, children, type JSX } from 'solid-js';
import { createStore } from 'solid-js/store';

export type CalendarStore = {
  selectedTaskId: number;
};

const CalendarStateContext = createContext();

export function CalendarStateProvider(props: { children: JSX.Element }) {
  const [store, setStore] = createStore<CalendarStore>({
    selectedTaskId: 0,
  });

  const packagedStore = [store, setStore];

  const resolved = children(() => props.children);

  return (
    <CalendarStateContext.Provider value={packagedStore}>
      {resolved()}
    </CalendarStateContext.Provider>
  );
}

export function useCalendarState() {
  return useContext(CalendarStateContext);
}
