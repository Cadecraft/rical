import { createEffect, createSignal, onCleanup } from "solid-js";
import { useCalendarState } from "./StateProvider";
import { useSearchParams } from "@solidjs/router";
import { isAnythingFocused } from "./helpers";

/** Binds a function to be called by a keypress. Returns whether the hotkey is down */
export function useHotkey(onActivated: () => void, hotkey?: string) {
  // Keydown may fire with #, but if the user releases shift first, keyup fires with 3
  const SHIFT_HOTKEY_EQUIV: Record<string, string> = {
    "#": "3",
  };

  const [hotkeyDown, setHotkeyDown] = createSignal(false);

  createEffect(() => {
    if (!hotkey) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (e.key === hotkey && !e.ctrlKey) {
        setHotkeyDown(true);
      } else if (e.key == "Escape") {
        setHotkeyDown(false);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (
        e.key === hotkey ||
        (hotkey in SHIFT_HOTKEY_EQUIV && e.key === SHIFT_HOTKEY_EQUIV[hotkey])
      ) {
        if (hotkeyDown()) {
          onActivated();
        }
        setHotkeyDown(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    onCleanup(() => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    });
  });

  return hotkeyDown;
}

export function useGlobalKey(onActivated: () => void, hotkey: string) {
  createEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === hotkey && !e.ctrlKey && !e.shiftKey) {
        onActivated();
      }
    };

    window.addEventListener("keydown", handleKeyDown);

    onCleanup(() => {
      window.removeEventListener("keydown", handleKeyDown);
    });
  });
}

export function useDateNav() {
  const [state] = useCalendarState();
  const [_, setParams] = useSearchParams();

  const prevMonth = () => {
    setParams({
      y: state.selectedMonth === 1 ? state.selectedYear - 1 : state.selectedYear,
      m: state.selectedMonth === 1 ? 12 : state.selectedMonth - 1,
    });
  };

  const nextMonth = () => {
    setParams({
      y: state.selectedMonth === 12 ? state.selectedYear + 1 : state.selectedYear,
      m: state.selectedMonth === 12 ? 1 : state.selectedMonth + 1,
    });
  };

  const toYearMonth = (year: number, month: number) => {
    setParams({
      y: year,
      m: month,
    });
  };

  return {
    prevMonth,
    nextMonth,
    toYearMonth,
  };
}
