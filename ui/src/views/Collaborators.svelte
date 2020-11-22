<script>
  import Header from '../components/Header.svelte';
	import { onMount } from 'svelte';


  let collaborators = [];

	onMount(() => {
    var urly = 'http://localhost:3000/api/collaborators'      
    fetch(urly)
      .then(res => res.json())
      .then(data => {
        // console.log(data)
        collaborators = data;
      });
	});

</script>

<div class="container-home">
  

  {#each collaborators as collaborator}
     {collaborator}
  {/each}

</div>
