import { Button, PlainLink } from "../components/Button";
import { Switch, Match, createSignal, createEffect } from "solid-js";
import Banner from "../components/Banner";
import Footer from "../components/Footer";
import { fetchSignup, fetchLogin, fetchWhoami } from "../util/apiInterface";
import { useSearchParams } from "@solidjs/router";

function Page(props: { signup: boolean }) {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");
  const [loading, setLoading] = createSignal(false);

  const [params] = useSearchParams();

  // TODO: make form so user can press enter at any time

  createEffect(() => {
    fetchWhoami()
      .then((_) => {
        // Kick logged-in users to /cal
        window.location.replace("/cal");
      })
      .catch(() => {});
  });

  function submit() {
    setError("");
    setLoading(true);

    if (props.signup) {
      fetchSignup(username(), password())
        .then(() => {
          location.href = "/login?signupSuccess=true";
        })
        .catch(() => {
          setError("Error signing up. This username may already be taken.");
          setLoading(false);
        });
    } else {
      fetchLogin(username(), password())
        .then((res) => {
          // TODO: localStorage is temporary. start sending token in cookie
          localStorage.setItem("tok", res.token);
          location.href = "/cal";
        })
        .catch(() => {
          setError("Username or password did not match an existing account");
          setLoading(false);
        });
    }
  }

  const loginMessage = () => {
    if (params.signupSuccess) {
      return "Your account was created successfully. Log in to start using Rical!";
    } else {
      return "Log in with a username and password.";
    }
  };

  const loginTitle = () => {
    if (params.reason === "loggedout") {
      return "Successfully logged out";
    } else {
      return "Login";
    }
  };

  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <Switch>
            <Match when={props.signup}>
              <h2>Sign up</h2>
              <div class="secondary">Sign up with a username and password.</div>
              <br />
              <div class="form">
                <input
                  type="text"
                  placeholder="New username"
                  value={username()}
                  onChange={(e) => setUsername(e.target.value)}
                />
                <input
                  type="password"
                  placeholder="New password"
                  value={password()}
                  onChange={(e) => setPassword(e.target.value)}
                />
                <Button disabled={loading()} onClick={submit}>
                  Sign up
                </Button>
              </div>
              <br />
              <div class="error">{error()}</div>
              <PlainLink href={"/login"} hotkey={"l"}>
                Log in instead
              </PlainLink>
            </Match>
            <Match when={!props.signup}>
              <h2>{loginTitle()}</h2>
              <div class="secondary">{loginMessage()}</div>
              <br />
              <div class="form">
                <input
                  type="text"
                  placeholder="Username"
                  value={username()}
                  onChange={(e) => setUsername(e.target.value)}
                />
                <input
                  type="password"
                  placeholder="Password"
                  value={password()}
                  onChange={(e) => setPassword(e.target.value)}
                />
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
        <Footer />
      </div>
    </div>
  );
}

export default Page;
