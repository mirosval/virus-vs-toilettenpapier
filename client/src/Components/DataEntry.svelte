<script>
  export let footerData = {};
  const {
    DESCRIPTION,
    CONTACT_DETAILS,
    SUBSCRIBE_NEWSLETTER,
    SEARCH_STORE
  } = footerData;
  const { HEADING, ADDRESS, MOBILE, EMAIL } = CONTACT_DETAILS;
</script>

<main>
    <div class="container">
      <div class="col-sm-3">
      <p/>
      </div>
      <div class="col-sm-3">
        <input
          type="email"
          class="form-control round-border"
          placeholder="{SEARCH_STORE}"
          />

        <button type="button" class="btn btn-outline-dark round-border">
          {SUBSCRIBE_NEWSLETTER}
        </button>
      </div>
      <div>
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
