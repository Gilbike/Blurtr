<script lang="ts">
	import BlurtField from "$lib/components/BlurtField.svelte";
  import Container from "$lib/components/Container.svelte";

  let fields: {question: string; anwser: string}[] = [];

  function addField() {
    fields = [...fields, {question: "", anwser: ""}];
  }
</script>

<Container>
  <h1 class="mt-3 font-bold text-2xl">Blurt létrehozása</h1>

  <div class="overflow-y-auto h-full py-3">
    <div class="flex flex-col gap-2 divide-y-[1px] divide-neutral-700/25">
      {#each fields as field}
        <BlurtField bind:question={field.question} bind:anwser={field.anwser} />
      {/each}
    </div>
    <button class="bg-blue-400 mt-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={addField}>
      Új kérdés
    </button>
  </div>
</Container>