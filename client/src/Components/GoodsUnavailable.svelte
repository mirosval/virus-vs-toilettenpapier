<script>
  export let GOODS_UNAVAILABLE;
  import AutoComplete from "simple-svelte-autocomplete";
  const items = [
    {"id": 1, "name": "Toilettenpapier"},
    {"id": 2, "name": "Spaghetti"},
    {"id": 3, "name": "Penne"},
    {"id": 4, "name": "Toast"},
    {"id": 5, "name": "Brot"},
    {"id": 6, "name": "Milch"}
  ]
  let selectedItem;
</script>

<div class="slidecontainer container">
  <div class="row">
    <div class="col">
      {GOODS_UNAVAILABLE}
    </div>
    <div class="col">
      <AutoComplete items={items} bind:selectedItem={selectedItem} labelFieldName="name" />
    </div>
  </div>
</div>

