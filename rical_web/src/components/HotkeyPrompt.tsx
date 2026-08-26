import "./HotkeyPrompt.css";
import { useHotkey } from "../util/hooks";

/** A hovering hotkey prompt. Use inside a position: relative container */
function HotkeyPrompt(props: { hotkey: string; onActivated: () => void; actionDescr?: string }) {
  const hotkeyDown = useHotkey(props.onActivated, props.hotkey);

  const hotkeyDisp = () => (props.hotkey === "Enter" ? "↩" : props.hotkey);

  return (
    <div
      class={`${hotkeyDown() ? "down " : ""}hotkey-prompt`}
      title={`Hotkey for ${props.actionDescr ?? "this action"}: ${props.hotkey}`}
    >
      {hotkeyDisp()}
    </div>
  );
}

export default HotkeyPrompt;
