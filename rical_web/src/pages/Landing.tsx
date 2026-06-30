import './Landing.css'
import { LinkButton } from '../components/Button';

function Page() {
  return (
    <div class="outer">
      <div class="inner">
        <div class="banner">
          <div class="banner-info">
            <h1>Rical</h1>
            <span>The latest calendar app for minimalists</span>
          </div>
          <img src="/RicalIcon.svg" alt="Rical icon" width={40} />
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
          <LinkButton href="https://github.com/Cadecraft/rical" hotkey="i">
            Install
          </LinkButton>
          <img class="terminal-ss" alt="A screenshot of Rical Terminal" src="/RicalTerminal.png">
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

export default Page;
