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

<div class="container-fluid">

  <div class="row index-view">
    <aside class="left-area col main-bgcolor pt-3">
      <h4 class="text-light">Meine Aktuelle Poistion lautet:</h4>
      <CurrentLocation />
      <a href="/add-location" class="btn btn-success btn-lg btn-block">
        Standort Eintragen
      </a>
      <Banner bannerData={DATA.BANNER_DATA} } />
    </aside>
    <div class="col-xl-9">
      <GeoLoc />
    </div>
  </div>
</div>
