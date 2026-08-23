import './HotkeyPrompt.css';
import { useHotkey } from '../util/hooks';

/** A hovering hotkey prompt. Use inside a position: relative container */
function HotkeyPrompt(props: { hotkey: string, onActivated: () => void, actionDescr?: string }) {
  useHotkey(props.onActivated, props.hotkey);

  return (
    <div class="hotkey-prompt" title={`Hotkey for ${props.actionDescr ?? 'this action'}: ${props.hotkey}`}>
      {props.hotkey}
    </div>
  );
}

export default HotkeyPrompt;
