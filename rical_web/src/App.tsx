import './App.css'
import RicalIcon from './assets/RicalIcon.svg';

function App() {
  return (
    <div className="outer">
      <div className="inner">
        <div className="banner">
          <div className="banner-info">
            <h1>Rical</h1>
            <span>The latest calendar app for minimalists</span>
          </div>
          <img src={RicalIcon} width={40} />
        </div>
        <div className="section">
          <h2>Get started</h2>
          <span className="secondary">
            Rical Web is coming soon! For now, you'll have to use the{" "}
            <a href="https://github.com/Cadecraft/rical">terminal UI</a>.
          </span>
        </div>
        <div className="section footer">
          © 2025 rical contributors
          <br />
          <a href="https://github.com/Cadecraft/rical">GitHub</a>
        </div>
      </div>
    </div>
  )
}

export default App
