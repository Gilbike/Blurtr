<script lang="ts">
	import Container from "$lib/components/Container.svelte";
	import { blurt, type Blurts } from "$lib/blurt";
	import BlurtResult from "$lib/components/BlurtResult.svelte";
  import { page } from '$app/stores';
    
  const timeSpent = parseInt($page.url.searchParams.get('time') || "0");

  let fields: Blurts = $blurt;
</script>

<Container>
  <h1 class="mt-3 font-bold text-2xl">Blurt kiértékelése</h1>

  <p>Kitöltési idő: {`${Math.floor(timeSpent / 60)}:${(timeSpent % 60).toString().padStart(2, "0")}`}</p>
  
  <div class="overflow-y-auto h-full py-3 flex flex-col gap-2 divide-y-[1px] divide-neutral-700/25">
    {#each fields as field, i (i)}
      <BlurtResult {...field} />
    {/each}
  </div>

  <a href="/" class="block text-center bg-blue-400 mb-3 w-full py-1 text-white hover:bg-blue-500 transition">
    Főoldalra
  </a>
</Container>