<script lang="ts">
	import { blurt, type Blurts } from "$lib/blurt";
	import Container from "$lib/components/Container.svelte";
	import BlurtField from "$lib/components/BlurtField.svelte";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
  
  let fields: Blurts = $blurt;
  let secondsSpent = 0;

  $: timeSpentFormatted = `${Math.floor(secondsSpent / 60)}:${(secondsSpent % 60).toString().padStart(2, "0")}`

  onMount(() => {
    const timer = setInterval(() => secondsSpent += 1, 1000);

    return () => {
      clearInterval(timer);
    }
  });

  function submitBlurt() {
    blurt.set(fields);
    goto(`/results?time=${secondsSpent}`);
  }
</script>

<Container>
  <h1 class="mt-3 font-bold text-2xl">Blurt kitöltése ({timeSpentFormatted})</h1>
  
  <div class="overflow-y-auto h-full py-3 flex flex-col gap-2 divide-y-[1px] divide-neutral-700/25">
    {#each fields as field, i (i)}
      <BlurtField mode="input" question={field.question} bind:answer={field.userAnswer} />
    {/each}
  </div>

  <button class="bg-blue-400 mb-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={submitBlurt}>
    Befejezés
  </button>
</Container>