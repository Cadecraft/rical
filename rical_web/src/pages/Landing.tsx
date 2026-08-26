import "./Landing.css";
import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";

function Page() {
  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <h2>Get started</h2>
          <span class="secondary">
            Rical Web is still in progress! For now, use the{" "}
            <a href="https://github.com/Cadecraft/rical">terminal UI</a> for full features.
          </span>
          <br />
          <br />
          <LinkButton href="/login" hotkey="l">
            Log in
          </LinkButton>
        </div>
        <div class="section">
          <h2>Learn Rical terminal</h2>
          <div class="secondary">Manage your calendar without ever leaving your terminal</div>
          <br />
          <LinkButton href="https://github.com/Cadecraft/rical" hotkey="i">
            Install
          </LinkButton>
          <img
            class="terminal-ss"
            alt="A screenshot of Rical Terminal"
            src="/RicalTerminal.png"
          ></img>
        </div>
        <div class="section">
          <h2>Why Rical?</h2>
          <br />
          Modern calendar apps are too slow! If you want these, Rical might be for you:
          <br />
          <ul>
            <li>Fast keyboard shortcuts</li>
            <li>Lightweight cross-device frontend(s*)</li>
            <li>
              A full-scale database that allows multiple accounts, syncing, and availability
              sharing*
            </li>
            <li>System notifications for events*</li>
          </ul>
          *{" "}
          <i>
            Rical is far from complete yet. Check back soon for more features, mark your
            calendars...
          </i>
        </div>
        <div class="section footer">
          © 2025 Rical contributors
          <br />
          <a href="https://github.com/Cadecraft/rical">GitHub</a>
        </div>
      </div>
    </div>
  );
}

export default Page;
