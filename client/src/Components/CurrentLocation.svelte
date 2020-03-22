<script>
  let geoloc;

  function showLocation(position) {
    var latitude = position.coords.latitude;
    var longitude = position.coords.longitude;
    geoloc = "Latitude : " + latitude + " Longitude: " + longitude;
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

<section
  class="light-color alert alert-light shadow-sm p-4"
  id="banner"
  role="alert">
  <div class="container">
    <h5 class="text-dark">Dein aktueller Standort:</h5>
    <p class="text-dark mb-0">{geoloc}</p>
  </div>
</section>
