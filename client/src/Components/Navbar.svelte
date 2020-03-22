<script>
  export let navlists = [];
  export let header;
</script>

<style>
  @media (min-width: 1200px) {
    .navbar {
      height: 60px;
    }
  }
  .navbar-nav li {
    padding: 0 0 0 20px;
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

<nav
  class="shadow-sm navbar sticky-top navbar-expand-lg navbar-light main-bgcolor">
  <a class="navbar-brand" href="/">
    <img
      class="navbar-brand__icon"
      src="images/vvt-shopping-cart.svg"
      alt="shopping card icon"
      role="presentation" />
    <span class="h6 text-white navbar-brand__label h5 mb-0">{header}</span>
  </a>
  <button
    class="navbar-toggler bg-light"
    type="button"
    data-toggle="collapse"
    data-target="#navbarSupportedContent"
    aria-controls="navbarSupportedContent"
    aria-expanded="false"
    aria-label="Toggle navigation">
    <span class="navbar-toggler-icon" />
  </button>
  <div class="collapse navbar-collapse" id="navbarSupportedContent">
    <ul class="navbar-nav ml-auto">
      {#each navlists as list}
        <li class="nav-item">
          <a class="nav-link light-color" href={list.url}>{list.label}</a>
        </li>
      {/each}
    </ul>
  </div>
</nav>
