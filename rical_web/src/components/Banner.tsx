import './Banner.css';

function Banner() {
  return (
    <div class="banner-clip">
      <div class="banner">
        <div class="banner-inner">
          <div class="banner-info">
            <h1>Rical</h1>
            <span>The latest calendar app for minimalists</span>
          </div>
          <img src="/RicalIcon.svg" alt="Rical icon" width={40} />
        </div>
      </div>
    </div>
  );
}

export default Banner;
