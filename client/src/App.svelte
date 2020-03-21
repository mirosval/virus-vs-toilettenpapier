<script>
  import Navbar from "./Components/Navbar.svelte";
  import Banner from "./Components/Banner.svelte";
  import GeoLoc from "./Components/GeoLoc.svelte";
  import DataEntry from "./Components/DataEntry.svelte";
  import DATA from "./Data/data";
</script>

<main>
    <Navbar navlists={DATA.NAVBAR_DATA} header={DATA.HEADER} />
    <Banner bannerData={DATA.BANNER_DATA} } />
    <GeoLoc/>
    <DataEntry footerData={DATA.FOOTER_DATA} header={DATA.HEADER} />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
