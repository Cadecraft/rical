import "./Footer.css";
import { REPO_URL } from "../util/constants";

function Footer() {
  return (
    <div class="section footer">
      © 2025-26 Rical contributors
      <br />
      <a target="_blank" href={REPO_URL}>
        GitHub
      </a>
    </div>
  );
}

export default Footer;
