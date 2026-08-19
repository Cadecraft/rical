import { Button, PlainLink } from '../components/Button';
import { Switch, Match, createSignal } from 'solid-js';
import Banner from '../components/Banner';

function Page(props: { signup: boolean }) {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");
  const [loading, setLoading] = createSignal(false);

  // TODO: make form so user can press enter at any time

  function submit() {
    // TODO: actually submit
    setError("");
    setLoading(true);
    if (props.signup) {
      fetch(`${import.meta.env.VITE_API_URL}/account/signup`, {
        method: 'POST',
        body: JSON.stringify({
          username: username(),
          password: password(),
        }),
        headers: {
          'Content-Type': 'application/json'
        }
      }).then(res => {
        if (res.ok) {
          location.href = "/login";
        } else {
          setError("Error signing up.");
          setLoading(false);
        }
      });
    } else {
      fetch(`${import.meta.env.VITE_API_URL}/account/login`, {
        method: 'POST',
        body: JSON.stringify({
          username: username(),
          password: password(),
        }),
        headers: {
          'Content-Type': 'application/json'
        }
      }).then(res => {
        if (res.ok) {
          res.json().then(j => {
            // TODO: localStorage is temporary. start sending token in cookie
            localStorage.setItem("tok", j.token);
            location.href = "/cal";
          });
        } else {
          setError("Username or password did not match an existing account");
          setLoading(false);
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
                <Button disabled={loading()} onClick={submit}>
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
                <Button disabled={loading()} onClick={submit}>
                  Log in
                </Button>
              </div>
              <br />
              <div class="error">{error()}</div>
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
