<script>
  import DataEntry from "../Components/DataEntry.svelte";
  import ContentPage from "../Components/ContentPage.svelte";
  import DATA from "../Data/data";
</script>

<ContentPage>
  <DataEntry dataEntryData={DATA.DATA_ENTRY} />
</ContentPage>
