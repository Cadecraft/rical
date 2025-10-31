import './App.css'
import RicalIcon from './assets/RicalIcon.svg';
import RicalTerminal from './assets/RicalTerminal.png';
import type { ReactNode } from 'react';

function Button(props: {
  children: ReactNode,
  hotkey?: string,
  onClick: () => void,
}) {
  const { children, hotkey, onClick } = props;
  return (
    <button className="rical-button" onClick={onClick}>
      {children}
      {hotkey && (
        <div className="hotkey" title={`The hotkey for this button is ${hotkey}`}>
          {hotkey}
        </div>
      )}
    </button>
  );
}

function App() {
  return (
    <div className="outer">
      <div className="inner">
        <div className="banner">
          <div className="banner-info">
            <h1>Rical</h1>
            <span>The latest calendar app for minimalists</span>
          </div>
          <img src={RicalIcon} width={40} />
        </div>
        <div className="section">
          <h2>Get started</h2>
          <span className="secondary">
            Rical Web is coming soon! For now, you'll have to use the{" "}
            <a href="https://github.com/Cadecraft/rical">terminal UI</a>.
          </span>
        </div>
        <div className="section">
          <h2>Learn Rical terminal</h2>
          <div className="secondary">
            Manage your calendar without ever leaving your terminal
          </div>
          <br />
          <Button onClick={() => location.href="https://github.com/Cadecraft/rical"} hotkey="i">
            Install
          </Button>
          <img className="terminal-ss" src={RicalTerminal}>
          </img>
        </div>
        <div className="section">
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
        <div className="section footer">
          © 2025 Rical contributors
          <br />
          <a href="https://github.com/Cadecraft/rical">GitHub</a>
        </div>
      </div>
    </div>
  )
}

export default App
