import "./Docs.css";
import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";
import Footer from "../components/Footer";
import { REPO_URL } from "../util/constants";

function Page() {
  function highlighted() {
    return true;
  }

  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <h2>Rical Docs</h2>
          <div class="secondary">Get help with Rical here</div>
        </div>
        <div class="section">
          <h2 id="hotkeys" class={highlighted() ? "highlighted" : ""}>
            Hotkeys (web calendar)
          </h2>
          <br />
          <h3>Global</h3>
          <table>
            <thead>
              <tr>
                <th>Key</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>?</td>
                <td>Hotkeys help</td>
              </tr>
              <tr>
                <td>h, j, k, l</td>
                <td>Navigate left, down, up, right in the calendar</td>
              </tr>
              <tr>
                <td>o</td>
                <td>Open a new task in the selected day</td>
              </tr>
              <tr>
                <td>Enter</td>
                <td>Enter the list of tasks for the current day</td>
              </tr>
              <tr>
                <td>p</td>
                <td>Profile menu</td>
              </tr>
            </tbody>
          </table>
          <br />
          <h3>When a task is selected</h3>
          <table>
            <thead>
              <tr>
                <th>Key</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>#</td>
                <td>Delete the task</td>
              </tr>
              <tr>
                <td>d</td>
                <td>Toggle whether the task is done</td>
              </tr>
              <tr>
                <td>Enter</td>
                <td>Edit the task, or confirm saving changes</td>
              </tr>
              <tr>
                <td>Tab, Shift+Tab</td>
                <td>Navigate between inputs</td>
              </tr>
              <tr>
                <td>Esc</td>
                <td>Cancel editing a task/deselect the task</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div class="section">
          <h2>Further Support</h2>
          <p>If you need help using Rical Terminal, check out the GitHub repository's README.</p>
          <p>
            Feel free to make a GitHub issue if you have bug reports or feature requests for either
            app.
          </p>
          <LinkButton href={REPO_URL} hotkey="i" newTab>
            GitHub repository
          </LinkButton>
        </div>
        <Footer />
      </div>
    </div>
  );
}

export default Page;
