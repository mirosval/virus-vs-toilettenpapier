<script>
  import Navbar from "./Components/Navbar.svelte";
  import Banner from "./Components/Banner.svelte";
  import GeoLoc from "./Components/GeoLoc.svelte";
  import DATA from "./Data/data";
  import router from "page.js";
  import routes from "./Routes/Route";

  let page;
  let params;

  // Iterate through the routes
  routes.forEach(route => {
    router(
      route.path,

      (ctx, next) => {
        params = ctx.params;
        next();
      },

      () => {
        page = route.component;
      }
    );
  });

  // Start the router
  router.start();
</script>

<style>
  main {
    text-align: center;
    margin: 0 auto;
  }
</style>

<main>
  <!-- Navbar -->
  <Navbar navlists={DATA.NAVBAR_DATA} header={DATA.HEADER} />
  <svelte:component this={page} {params} />
</main>
