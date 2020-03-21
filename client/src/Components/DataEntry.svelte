<script>
  export let footerData = {};
  const {
    DESCRIPTION,
    CONTACT_DETAILS,
    SUBMIT,
    SEARCH_STORE
  } = footerData;
  const { HEADING, ADDRESS, MOBILE, EMAIL } = CONTACT_DETAILS;
  import CrowdednessSlider from "./CrowdednessSlider.svelte";
</script>

<main>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col-md-6">
      <hr/>
        <input
          type="email"
          class="form-control round-border"
          placeholder="{SEARCH_STORE}"
          />
        <CrowdednessSlider/>

        <button type="button" class="btn btn-outline-dark round-border">
          {SUBMIT}
        </button>
      </div>
    </div>
  </div>
</main>

<style>
  .footer-title {
    font-weight: bold;
    text-transform: uppercase;
  }

  .footer-box button {
    margin-top: 30px;
  }

  .round-border {
    border-radius: 20px !important;
  }
  .wave-img {
    width: 100%;
    height: auto;
  }
</style>
