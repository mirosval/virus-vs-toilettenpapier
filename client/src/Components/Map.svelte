<script>
  import mapboxgl from "mapbox-gl/dist/mapbox-gl.js";
  import { activeListItem, activeMapItem } from "./stores.js";
  import { onMount, onDestroy } from "svelte";
  import { accessToken, listItems } from "./consts";

  let mapRef;

  function generateFeature({ name, image, coordinates }, index) {
    return {
      type: "Feature",
      properties: {
        description: `<img width="100%" src="${image}"/><b>${name}</b>`,
        id: index
      },
      geometry: {
        type: "Point",
        coordinates
      }
    };
  }

  onMount(async () => {
    mapboxgl.accessToken = accessToken;
    // Create the map
    mapRef = new mapboxgl.Map({
      container: "map",
      style: "mapbox://styles/mapbox/light-v10?optimize=true",
      center: listItems[0].coordinates,
      zoom: 13
    });

    mapRef.on("load", function() {
      // Add markers to map
      mapRef.addLayer({
        id: "places",
        type: "symbol",
        source: {
          type: "geojson",
          data: {
            type: "FeatureCollection",
            features: listItems.map(generateFeature)
          }
        },
        layout: {
          "icon-image": "cafe-15",
          "icon-size": 2,
          "icon-allow-overlap": true
        }
      });
    });
  });
</script>

<style>
  #map {
    width: 100%;
    height: 100%;
  }

  #map:before {
    box-shadow: 20px 0 10px -10px rgba(0, 0, 0, 0.15) inset;
    content: "";
    height: 100%;
    left: 0;
    position: absolute;
    width: 20px;
    z-index: 1000;
  }
</style>

<div class="container" id="map" />
