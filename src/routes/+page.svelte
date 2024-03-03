<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { blurt, type Blurts } from "$lib/blurt";
	import BlurtField from "$lib/components/BlurtField.svelte";
	import Container from "$lib/components/Container.svelte";
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from "svelte";

  let blurtName: string = "";
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

  function removeBlurt(index: number) {
    fields = [...fields.slice(0, index), ...fields.slice(index + 1)]
  }

  function exportBlurt() {
    const exportData = {
      title: blurtName,
      fields: fields.map((obj) => {
        const { ["userAnswer"]: _, ...rest } = obj;
        return rest;
      })
    };
    invoke('export_blurt', { params: JSON.stringify(exportData) });
  }

  onMount(() => {
    if ($page.url.searchParams.get("refresh") !== "true") return;

    fields = $blurt;
  });
</script>

<Container>
  <h1 class="mt-3 font-bold text-2xl">Blurt létrehozása</h1>

  <div class="overflow-y-auto h-full py-3">
    <div class="flex flex-col">
      <label for="blurtname">
        Blurt neve
      </label>
      <input type="text" id="blurtname" name="blurtname" class="border-2 border-black px-2 py-1 outline-none" bind:value={blurtName} />
    </div>
    <div class="flex flex-col gap-2 divide-y-[1px] divide-neutral-700/25 mt-1">
      {#each fields as field, i (i)}
        <BlurtField bind:question={field.question} bind:answer={field.answer} on:delete={() => removeBlurt(i)} />
      {/each}
    </div>
    <button class="bg-blue-400 mt-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={addField}>
      Új kérdés
    </button>
    <div class="flex flex-row gap-2">
      <button disabled={fields.length == 0} class="bg-red-400 mt-3 w-full py-1 text-white hover:bg-red-500 transition disabled:bg-red-300/50" on:click={exportBlurt}>
        Exportálás
      </button>
      <a href="/import" class="text-center bg-emerald-400 mt-3 w-full py-1 text-white hover:bg-emerald-500 transition">
        Importálás
      </a>
    </div>
  </div>

  <button class="bg-blue-400 mb-3 w-full py-1 text-white hover:bg-blue-500 transition" on:click={startBlurting}>
    Kezdés
  </button>
</Container>