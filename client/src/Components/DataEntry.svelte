<script>
  export let dataEntryData = {};
  const {
    GET_LOCATION,
    GOODS_UNAVAILABLE,
    CROWDEDNESS,
    SUBMIT,
    SEARCH_STORE
  } = dataEntryData;
  import SearchStore from "./SearchStore.svelte";
  import CrowdednessSlider from "./CrowdednessSlider.svelte";
</script>

<main>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col">
      <hr/>
        <button type="button" class="btn btn-outline-dark round-border">
          {GET_LOCATION}
        </button>

        <SearchStore SEARCH_STORE={SEARCH_STORE}/>

        {CROWDEDNESS}
        <CrowdednessSlider/>

        <input
          type="text"
          class="form-control round-border"
          placeholder="{GOODS_UNAVAILABLE}"
          />
        <button type="button" class="btn btn-outline-dark round-border">
          {SUBMIT}
        </button>
      </div>
    </div>
  </div>
</main>

<style>
  .round-border {
    border-radius: 10px !important;
  }
</style>
