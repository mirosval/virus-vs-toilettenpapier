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

<section
  class="light-color alert alert-light shadow-sm p-4"
  id="banner"
  role="alert">
  <div class="container mb-4">
    <h6 class="text-dark">Dein aktueller Standort:</h6>
    <p class="text-dark mb-0">{geoloc}</p>
  </div>
  <form>
    <div class="form-group mb-3">
      <label class="text-dark" for="locationInput">Standort festlegen</label>
      <input
        type="text"
        class="form-control form-control-lg"
        id="locationInput"
        placeholder="Altona Hauptbahnhof"
        aria-describedby="location help" />
    </div>

    <button type="submit" class="btn btn-outline-dark btn-lg btn-block mb-3">
      Submit
    </button>
  </form>
</section>
