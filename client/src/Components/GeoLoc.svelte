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
    } else if( err.code == 2) {
        alert("Error: Position is unavailable!");
    }
}

function getLocation() {
    if (navigator.geolocation) {
        var options = {timeout:60000}; // timeout at 60 seconds
        navigator.geolocation.getCurrentPosition(showLocation, errorHandler, options);
    } else {
        alert("Sorry, browser does not support geolocation!");
    }
}

getLocation();

</script>

<main>
	<p>It seems you are here: {geoloc}</p>
</main>
