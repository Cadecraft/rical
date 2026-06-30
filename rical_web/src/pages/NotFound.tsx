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
          <img src="/RicalIcon.svg" width={40} />
        </div>
        <div class="section">
          <h2>Not found!</h2>
          <div class="secondary">
            This page doesn't exist.
          </div>
          <br />
          <LinkButton href="/" hotkey="h">
            Home
          </LinkButton>
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
