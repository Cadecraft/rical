import { createEffect, createSignal, onCleanup } from 'solid-js';

/** Binds a function to be called by a keypress. Returns whether the hotkey is down */
export function useHotkey(onActivated: () => void, hotkey?: string) {
  const [hotkeyDown, setHotkeyDown] = createSignal(false);

  function isAnythingFocused() {
    return document.activeElement &&
      document.activeElement !== document.body &&
      document.activeElement.nodeName !== "BUTTON";
  }

  createEffect(() => {
    if (!hotkey) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (e.key === hotkey && !e.ctrlKey) {
        setHotkeyDown(true);
      } else if (e.key == 'Escape') {
        setHotkeyDown(false);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (isAnythingFocused()) {
        return;
      }
      if (e.key === hotkey) {
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
