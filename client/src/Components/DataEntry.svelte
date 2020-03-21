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
  import GoodsUnavailable from "./GoodsUnavailable.svelte";
</script>

<main>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col">
        <button type="button" class="btn btn-outline-dark round-border">
          {GET_LOCATION}
        </button>
      <hr/>

        <SearchStore SEARCH_STORE={SEARCH_STORE}/>
      <hr/>

        <CrowdednessSlider CROWDEDNESS={CROWDEDNESS}/>
      <hr/>

        <GoodsUnavailable GOODS_UNAVAILABLE={GOODS_UNAVAILABLE}/>
      <hr/>

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
