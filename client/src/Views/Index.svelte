<script>
  import Banner from "../Components/Banner.svelte";
  import GeoLoc from "../Components/GeoLoc.svelte";
  import CurrentLocation from "../Components/CurrentLocation.svelte";
  import DATA from "../Data/data";
</script>

<style>
  .index-view {
    height: calc(100vh - 60px);
  }

  .left-area {
    min-width: 300px;
  }
</style>

<div class="index-view container-fluid">

  <div class="row ">
    <aside
      class="d-flex flex-column justify-content-between left-area col bg-light
      pt-3">
      <div class="mb-5">
        <CurrentLocation />
        <a href="/add-location" class="btn btn-success btn-lg btn-block">
          Standort Eintragen
        </a>
      </div>
      <Banner bannerData={DATA.BANNER_DATA} } />
    </aside>
    <div class="col-xl-9 p-0">
      <GeoLoc />
    </div>
  </div>
</div>
