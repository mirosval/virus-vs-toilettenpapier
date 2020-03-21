<script>
  export let bannerData = {};
  const { HEADING, DECRIPTION, TUTORIAL_URL, WATCH_TUTORIAL } = bannerData;
</script>

<section class="main-bgcolor light-color" id="banner">
  <div class="container">
    <div class="row">
      <div class="col-md-9">
        <h1>{HEADING}</h1>
        <p>{DECRIPTION}</p>
      </div>
      <div class="col-md-3">
        <img src="images/cart.png" alt="" class="img-fluid" />
      </div>
    </div>
  </div>
</section>

<style>
  section {
    padding-top: 5%;
  }

  h1 {
    font-size: 40px;
    font-weight: 600;
    margin-top: 100px;
    text-transform: uppercase;
  }

  .watch-btn {
    margin: auto 20px;
    position: relative;
    top: 8px;
  }

  section a {
    text-decoration: none;
  }
</style>
