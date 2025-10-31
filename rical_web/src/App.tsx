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
      {hotkey && <div className="hotkey">{hotkey}</div>}
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
          <h2>Learn rical terminal</h2>
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
        <div className="section footer">
          © 2025 rical contributors
          <br />
          <a href="https://github.com/Cadecraft/rical">GitHub</a>
        </div>
      </div>
    </div>
  )
}

export default App
