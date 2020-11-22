<style>
  .header {
    height: auto;
    width: 100%;
    background: skyblue;
    box-sizing: border-box;
    padding: 7px;
  }
  .header .menu {
    display: flex;
    align-items: center;
    list-style-type: none;
    justify-content: flex-end;
  }
  .header .menu .menuItem {
    display: flex;
    justify-content: center;
    align-items: center;
    background: black;
    color: white;
    font-size: 16px;
    padding: 10px 20px;
    margin: 0 10px;
    cursor: pointer;
  }
  .header .menu .menuItem.active {
    background: blueviolet;
  }
  .header .menu .menuItem:hover {
    background: lightgray;
    color: black;
  }
  .header .menu .logo {
    flex: 1 0 auto;
  }
</style>

<script>
  const logoUrl = 'http://placehold.it/180x60';
  let menuOptions = [
    {
      title: 'Option 1',
      url: '#',
      active: false
    },
    {
      title: 'Option 2',
      url: '#',
      active: false
    },
    {
      title: 'Option 3',
      url: '#',
      active: false
    }
  ];

  const setActivate = ({ title }) => {
    menuOptions = menuOptions.map(item => ({
      ...item,
      active: item.title === title
    }))
  }
</script>

<div class="header">
  <ul class="menu">
    <li class="logo"><img src={logoUrl} alt="logo" /></li>
    {#each menuOptions as option}
      <li
        class="menuItem{option.active ? ' active' : ''}"
        on:click={() => setActivate(option)}>
        {option.title}
      </li>
    {/each}
  </ul>
</div>
