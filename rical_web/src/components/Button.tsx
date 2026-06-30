import './Button.css';
import type { JSX } from 'solid-js';
import { children, createEffect, createSignal, Show } from 'solid-js';

// TODO: make LinkButton with same behavior but uses an a
export function Button(props: {
  children: JSX.Element,
  hotkey?: string,
  onClick: () => void,
}) {
  const resolved = children(() => props.children);

  const [hotkeyDown, setHotkeyDown] = createSignal(false);

  createEffect(() => {
    if (!props.hotkey) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key == props.hotkey) {
        setHotkeyDown(true);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.key == props.hotkey) {
        if (hotkeyDown()) {
          props.onClick();
        }
        setHotkeyDown(false);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
  });

  return (
    <button class={`rical-button ${hotkeyDown() ? 'pressed' : ''}`} onClick={props.onClick}>
      {resolved()}
      <Show when={props.hotkey}>
        <div class="hotkey" title={`The hotkey for this button is ${props.hotkey}`}>
          {props.hotkey}
        </div>
      </Show>
    </button>
  );
}

export function LinkButton(props: {
  children: JSX.Element,
  hotkey?: string,
  href: string,
}) {
  const resolved = children(() => props.children);

  const [hotkeyDown, setHotkeyDown] = createSignal(false);

  createEffect(() => {
    if (!props.hotkey) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key == props.hotkey) {
        setHotkeyDown(true);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (e.key == props.hotkey) {
        if (hotkeyDown()) {
          location.href = props.href;
        }
        setHotkeyDown(false);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
  });

  return (
    <a draggable={false} class={`rical-button ${hotkeyDown() ? 'pressed' : ''}`} href={location.href}>
      {resolved()}
      <Show when={props.hotkey}>
        <div class="hotkey" title={`The hotkey for this button is ${props.hotkey}`}>
          {props.hotkey}
        </div>
      </Show>
    </a>
  );
}
