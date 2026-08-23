import './Button.css';
import type { JSX } from 'solid-js';
import { children, Show } from 'solid-js';
import { A, useNavigate } from '@solidjs/router';
import { useHotkey } from '../util/hooks';

// TODO: make LinkButton with same behavior but uses an a
export function Button(props: {
  children: JSX.Element,
  hotkey?: string,
  onClick: () => void,
  disabled?: boolean,
  class?: string,
}) {
  const resolved = children(() => props.children);

  const hotkeyDown = useHotkey(props.onClick, props.hotkey);

  return (
    <button class={`${props.class ?? 'rical-button'} ${hotkeyDown() ? 'pressed' : ''}`} disabled={props.disabled} onClick={props.onClick}>
      {resolved()}
      <Show when={props.hotkey}>
        <div class="hotkey" title={`Hotkey for this button: ${props.hotkey}`}>
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

  const hotkeyDown = useHotkey(() => {
    if (props.href.startsWith("http")) {
      location.href = props.href;
    } else {
      navigate(props.href);
    }
  }, props.hotkey);

  const navigate = useNavigate();

  return (
    <A draggable={false} class={`rical-button ${hotkeyDown() ? 'pressed' : ''}`} href={props.href}>
      {resolved()}
      <Show when={props.hotkey}>
        <div class="hotkey" title={`Hotkey for this link: ${props.hotkey}`}>
          {props.hotkey}
        </div>
      </Show>
    </A>
  );
}

export function PlainLink(props: {
  children: JSX.Element,
  hotkey?: string,
  href: string,
}) {
  const resolved = children(() => props.children);

  const hotkeyDown = useHotkey(() => {
    if (props.href.startsWith("http")) {
      location.href = props.href;
    } else {
      navigate(props.href);
    }
  }, props.hotkey);

  const navigate = useNavigate();

  return (
    <A draggable={false} class={`rical-link ${hotkeyDown() ? 'pressed' : ''}`} href={props.href}>
      {resolved()}
      <Show when={props.hotkey}>
        <span title={`Hotkey for this link: ${props.hotkey}`}>{' '}({props.hotkey})</span>
      </Show>
    </A>
  );
}
