import { LinkButton } from "../components/Button";
import Banner from "../components/Banner";
import Footer from "../components/Footer";

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
        <Footer />
      </div>
    </div>
  );
}

export default Page;
