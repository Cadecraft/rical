import "./Landing.css";
import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";
import Footer from "../components/Footer";

function Page() {
  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <h2>Get started</h2>
          <span class="secondary">
            All you need is a username and password.
          </span>
          <br />
          <br />
          <LinkButton href="/login" hotkey="l">
            Log in / sign up
          </LinkButton>
        </div>
        <div class="section">
          <h2>Or, learn Rical terminal</h2>
          <div class="secondary">Manage your calendar without ever leaving your terminal</div>
          <br />
          <LinkButton href="https://github.com/Cadecraft/rical" hotkey="i">
            Install
          </LinkButton>
          <img
            class="full-img"
            alt="A screenshot of Rical Terminal"
            src="/RicalTerminal.png"
          ></img>
        </div>
        <div class="section">
          <h2>Why Rical?</h2>
          <br />
          Modern calendar apps are too cluttered! If you want these, Rical might be for you:
          <br />
          <ul>
            <li>Fast keyboard shortcuts</li>
            <li>Lightweight cross-device frontends</li>
            <li>
              A full-scale database that allows multiple accounts, syncing, and availability
              sharing*
            </li>
            <li>System notifications for events*</li>
          </ul>
          *{" "}
          <i>
            Rical is still in progress and far from complete.
            Check back soon for more features, mark your calendars...
          </i>
        </div>
        <div class="section">
          <img
            class="full-img web-ss"
            alt="A screenshot of Rical Web"
            src="/RicalWeb.png"
          ></img>
        </div>
        <Footer />
      </div>
    </div>
  );
}

export default Page;
