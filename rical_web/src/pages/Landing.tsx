import "./Landing.css";
import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";
import Footer from "../components/Footer";
import { IoCalendarSharp } from 'solid-icons/io';

function VibeyIcon() {
  return (
    <IoCalendarSharp class="vibey-icon" />
  )
}

function Page() {
  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section intro">
          <h2>Get started</h2>
          <span>
            Try out Rical in seconds
          </span>
          <br />
          <br />
          <LinkButton href="/login" hotkey="l">
            Log in / sign up
          </LinkButton>
          <VibeyIcon />
        </div>
        <div class="section">
          <h2>Why Rical?</h2>
          <p>
            Modern calendar apps are too cluttered and slow! If you want these, Rical might be for you:
          </p>
          <ul>
            <li>Fast keyboard shortcuts</li>
            <li>Lightweight cross-device interface</li>
            <li>Fully open-source</li>
          </ul>
        </div>
        <img
          class="full-img"
          alt="A screenshot of Rical Web"
          src="/RicalWeb.png"
        ></img>
        <div class="section">
          <h2>Also available in your terminal</h2>
          <div class="secondary">Manage your calendar from the command line</div>
          <br />
          <LinkButton href="https://github.com/Cadecraft/rical" hotkey="i">
            Install
          </LinkButton>
          <img
            class="wide-img terminal-ss"
            alt="A screenshot of Rical Terminal"
            src="/RicalTerminal.png"
          ></img>
        </div>
        <div class="section">
          <h2>Coming soon</h2>
          <p>
            Rical is still in progress and far from complete.
            Check back soon for more features, mark your calendars...
          </p>
          <p>
            Here's a sneak peek at what's planned:
          </p>
          <ul>
            <li>Syncing and availability sharing</li>
            <li>Desktop and mobile notifications for upcoming events</li>
            <li>Better mobile experience</li>
          </ul>
        </div>
        <Footer />
      </div>
    </div>
  );
}

export default Page;
