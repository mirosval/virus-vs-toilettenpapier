<script>
  export let SEARCH_STORE;
  import AutoComplete from "simple-svelte-autocomplete";
  const stores = [
    {"id": 1, "name": "Rewe, Steendiek"},
    {"id": 2, "name": "Rewe, Dehnhaide"},
    {"id": 3, "name": "Penny, Bahnhofstraße"},
    {"id": 4, "name": "Aldi, Feldstraße"},
    {"id": 5, "name": "Famila, Rissener Straße"},
    {"id": 6, "name": "Aldi, Rissener Straße"},
    {"id": 7, "name": "Penny, Washingtonallee"},
    {"id": 8, "name": "Rewe, Rennbahnstraße"},
    {"id": 9, "name": "REWE Center Billstedt, Lorenzenweg"},
    {"id": 10, "name": "Netto, Mendelstraße"}
  ]
  let selectedStore;
</script>

<div class="container">
  <div class="row">
    <div class="col">
      {SEARCH_STORE}
    </div>
    <div class="col">
      <AutoComplete items={stores} bind:selectedItem={selectedStore} labelFieldName="name" />
    </div>
  </div>
</div>

<style>
  .round-border {
    border-radius: 10px !important;
  }
</style>
