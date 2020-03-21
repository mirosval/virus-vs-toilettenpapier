<script>
  import Banner from "../Components/Banner.svelte";
  import GeoLoc from "../Components/GeoLoc.svelte";
  import DATA from "../Data/data";
</script>

<style>
  .index-view {
    height: calc(100vh - 50px);
  }
</style>

<div class="row index-view">
  <div class="col main-bgcolor mw-100">
    <Banner bannerData={DATA.BANNER_DATA} } />
  </div>
  <div class="col-9">
    <GeoLoc />
  </div>
</div>
