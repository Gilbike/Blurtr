<script lang="ts">
	import { goto } from "$app/navigation";
	import { blurt, type Blurts } from "$lib/blurt";
	import BlurtField from "$lib/components/BlurtField.svelte";
  import Container from "$lib/components/Container.svelte";

  let fields: Blurts = [];

  function addField() {
    fields = [...fields, {question: "", answer: "", userAnswer: ""}];
  }

  function startBlurting() {
    if (fields.length === 0 ||fields.filter(field => field.question === "" || field.answer === "").length != 0) {
      return;
    }

    blurt.set(fields);
    goto("/blurting")
  }
</script>

<Container>
  <h1 class="mt-3 font-bold text-2xl">Blurt létrehozása</h1>

  <div class="overflow-y-auto h-full py-3">
    <div class="flex flex-col gap-2 divide-y-[1px] divide-neutral-700/25">
      {#each fields as field}
        <BlurtField bind:question={field.question} bind:answer={field.answer} />
      {/each}
    </div>
    <button class="bg-blue-400 mt-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={addField}>
      Új kérdés
    </button>
  </div>

  <button class="bg-blue-400 mb-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={startBlurting}>
    Kezdés
  </button>
</Container>