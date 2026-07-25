import { LinkButton, PlainLink } from '../components/Button';

function Page(props: { signup: boolean }) {

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
          <h2>Login</h2>
          <div class="secondary">
            Log in with a username and password.
          </div>
          <br />
          <PlainLink href={props.signup ? "/login" : "/signup"} hotkey={props.signup ? "l" : "s"}>
            {props.signup ? "Log in instead" : "Sign up instead"}
          </PlainLink>
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
