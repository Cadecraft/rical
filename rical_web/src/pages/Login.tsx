import { Button, PlainLink } from '../components/Button';
import { Switch, Match, createSignal } from 'solid-js';
import Banner from '../components/Banner';

function Page(props: { signup: boolean }) {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");

  // TODO: make form so user can press enter at any time

  function submit() {
    // TODO: actually submit
    return;
    if (props.signup) {
      fetch(`${import.meta.env.VITE_API_URL}/signup`, {
        method: 'POST',
        body: JSON.stringify({
          username: username(),
          password: password(),
        }),
      }).then(res => {
        if (res.ok) {
          // TODO: send the user to login
        }
      });
    } else {
      fetch(`${import.meta.env.VITE_API_URL}/login`, {
        method: 'POST',
        body: JSON.stringify({
          username: username(),
          password: password(),
        }),
      }).then(res => {
        if (res.ok) {
          // TODO: store token
          // TODO: start sending token in cookie
          // TODO: send the user to their calendar
        }
      });
    }
  }

  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <Switch>
            <Match when={props.signup}>
              <h2>Sign up</h2>
              <div class="secondary">
                Sign up with a username and password.
              </div>
              <br />
              <div class="form">
                <input type="text" placeholder="New username" value={username()} onChange={e => setUsername(e.target.value)} />
                <input type="password" placeholder="New password" value={password()} onChange={e => setPassword(e.target.value)} />
                <Button onClick={submit}>
                  Sign up
                </Button>
              </div>
              <br />
              <PlainLink href={"/login"} hotkey={"l"}>
                Log in instead
              </PlainLink>
            </Match>
            <Match when={!props.signup}>
              <h2>Login</h2>
              <div class="secondary">
                Log in with a username and password.
              </div>
              <br />
              <div class="form">
                <input type="text" placeholder="Username" value={username()} onChange={e => setUsername(e.target.value)} />
                <input type="password" placeholder="Password" value={password()} onChange={e => setPassword(e.target.value)} />
                <Button onClick={submit}>
                  Log in
                </Button>
              </div>
              <br />
              <PlainLink href="/signup" hotkey={"s"}>
                Sign up instead
              </PlainLink>
            </Match>
          </Switch>
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
