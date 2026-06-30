import './App.css'
import type { JSX } from 'solid-js';
import { children, createEffect, createSignal, Show } from 'solid-js';

// TODO: make LinkButton with same behavior but uses an a
function Button(props: {
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

function App() {
  return (
    <div class="outer">
      <div class="inner">
        <div class="banner">
          <div class="banner-info">
            <h1>Rical</h1>
            <span>The latest calendar app for minimalists</span>
          </div>
          <img src="/RicalIcon.svg" width={40} />
        </div>
        <div class="section">
          <h2>Get started</h2>
          <span class="secondary">
            Rical Web is coming soon! For now, you'll have to use the{" "}
            <a href="https://github.com/Cadecraft/rical">terminal UI</a>.
          </span>
        </div>
        <div class="section">
          <h2>Learn Rical terminal</h2>
          <div class="secondary">
            Manage your calendar without ever leaving your terminal
          </div>
          <br />
          <Button onClick={() => location.href="https://github.com/Cadecraft/rical"} hotkey="i">
            Install
          </Button>
          <img class="terminal-ss" src="/RicalTerminal.png">
          </img>
        </div>
        <div class="section">
          <h2>Why Rical?</h2>
          <br />
          Modern calendar apps are too slow! If you want these, Rical might be for you:
          <br />
          <ul>
            <li>Fast keyboard shortcuts</li>
            <li>Lightweight cross-device frontend(s*)</li>
            <li>A full-scale database that allows multiple accounts, syncing, and availability sharing*</li>
            <li>System notifications for events*</li>
          </ul>
          * <i>Rical is far from complete yet. Check back soon for more features, mark your calendars...</i>
        </div>
        <div class="section footer">
          © 2025 Rical contributors
          <br />
          <a href="https://github.com/Cadecraft/rical">GitHub</a>
        </div>
      </div>
    </div>
  )
}

export default App;
