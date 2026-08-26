import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";

function Page() {
  return (
    <div class="outer">
      <div class="inner">
        <Banner />
        <div class="section">
          <h2>Not found!</h2>
          <div class="secondary">This page doesn't exist.</div>
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
  );
}

export default Page;
