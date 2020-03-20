<script>
export let name;
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
	<h1>Lokal-Tracker {name}!</h1>
	<p>It seems you are at: {geoloc}</p>
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
