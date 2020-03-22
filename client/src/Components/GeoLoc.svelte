<script>
  const url = "http://localhost:3000/v1/checkins/around";
  let geoloc;
  function getNearbyCheckins(lat, lng) {
    const payload = {
      gps: [lat, lng],
      radius: 1000,
      offset: 0,
      limit: 10
    };
    fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(payload)
    }).then(data => {
      console.log(data);
    });
  }
  function showLocation(position) {
    var latitude = position.coords.latitude;
    var longitude = position.coords.longitude;
    geoloc = "Latitude : " + latitude + " Longitude: " + longitude;
    getNearbyCheckins(latitude, longitude);
  }
  function errorHandler(err) {
    if (err.code == 1) {
      alert("Error: Access is denied!");
    } else if (err.code == 2) {
      alert("Error: Position is unavailable!");
    }
  }
  function getLocation() {
    if (navigator.geolocation) {
      var options = { timeout: 60000 }; // timeout at 60 seconds
      navigator.geolocation.getCurrentPosition(
        showLocation,
        errorHandler,
        options
      );
    } else {
      alert("Sorry, browser does not support geolocation!");
    }
  }
  getLocation();
</script>

<style>
  .image {
    width: 100%;
    object-fit: cover;
    height: calc(100vh - 60px);
  }
</style>

<main>
  <img
    class="image"
    src="/images/supermarket-heatmap.png"
    alt="Heatmap dummy" />
</main>
