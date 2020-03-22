<script>
  export let bannerData = {};
  const { HEADING, DECRIPTION, TUTORIAL_URL, WATCH_TUTORIAL } = bannerData;
</script>

<section
  class="light-color alert alert-light shadow-sm p-4"
  id="banner"
  role="alert">
  <div class="container">
    <h5 class="text-dark">{HEADING}</h5>
    <p class="text-dark mb-0">{DECRIPTION}</p>
  </div>
</section>
