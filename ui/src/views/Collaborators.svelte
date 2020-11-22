<script>
  import Header from '../components/Header.svelte';
	import { onMount } from 'svelte';


  let collaborators = [];

	onMount(() => {
    var urly = 'https://bukhgalter-collaborators.vercel.app/api/collaborators'      
    fetch(urly)
      .then(res => res.json())
      .then(data => {
        // console.log(data)
        collaborators = data;
      });
	});

</script>

{#each collaborators as collaborator}
      <div class="w-full md:w-1/3 p-6 flex flex-col flex-grow flex-shrink">
        <div class="flex-1 bg-white rounded-t rounded-b-none overflow-hidden shadow">
          <a href="#" class="flex flex-wrap no-underline hover:no-underline">
            <div class="w-full font-bold text-xl text-gray-800 px-6">
              {collaborator.name}
            </div>
            <p class="text-gray-800 text-base px-6 mb-5">
              {collaborator.description}
            </p>
          </a>
        </div>
      </div>
{/each}
