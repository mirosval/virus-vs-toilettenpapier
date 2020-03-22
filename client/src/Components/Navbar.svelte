<script>
  export let navlists = [];
  export let header;
</script>

<style>
  .nav-bar {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .navbar {
    padding: 0 20px !important;
    height: 60px;
  }

  .navbar-nav li {
    padding: 0 0 0 20px;
  }

  .navbar-nav li a {
    font-weight: 600;
    float: right;
    text-align: left;
  }

  .navbar-brand {
    display: flex;
    align-items: center;
  }

  .navbar-brand__icon {
    width: 40px;
    margin-right: 20px;
  }
</style>

<section class="shadow-sm nav-bar">
  <nav class="navbar navbar-expand-md navbar-dark main-bgcolor">
    <a class="navbar-brand" href="/">
      <img
        class="navbar-brand__icon"
        src="images/vvt-shopping-cart.svg"
        alt="shopping card icon"
        role="presentation" />
      <span class="navbar-brand__label h5 mb-0">{header}</span>
    </a>
    <button
      class="navbar-toggler"
      type="button"
      data-toggle="collapse"
      data-target="#navbarNav"
      aria-controls="navbarNav"
      aria-expanded="false"
      aria-label="Toggle navigation">
      <span class="navbar-toggler-icon" />
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
      <ul class="navbar-nav ml-auto">
        {#each navlists as list}
          <li class="nav-item">
            <a class="nav-link light-color" href={list.url}>{list.label}</a>
          </li>
        {/each}
      </ul>
    </div>
  </nav>
</section>
