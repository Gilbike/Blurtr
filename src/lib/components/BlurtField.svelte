<script lang="ts">
  import SvgIcon from "$lib/components/SvgIcon.svelte";
  import { mdiDelete } from "@mdi/js";
	import { createEventDispatcher } from "svelte";

  export let question: string;
  export let answer: string;

  export let mode: "creation" | "input" = "creation";

  const dispatch = createEventDispatcher();

  function deleteBlurtQuestion() {
    dispatch("delete");
  }
</script>

<div class="flex flex-col gap-1">
  <div>
    <label for="question" class="flex flex-row gap-1 items-end justify-between mb-1">
      Kérdés
      <button class="deletebutton" on:click={deleteBlurtQuestion}>
        <SvgIcon icon={mdiDelete} />
      </button>
    </label>
    {#if mode == "creation"}

    <input type="text" name="question" id="question" bind:value={question} autocomplete="off" />
    {:else}
      <p class="text-lg font-semibold">{question}</p>
    {/if}
  </div>

  <div>
    <label for="anwser">Válasz</label>
    <textarea name="anwser" id="anwser" rows="6" bind:value={answer} />
  </div>
</div>

<style scoped>
  div * {
    @apply w-full;
  }

  label {
    @apply text-neutral-800 font-light;
  }

  input, textarea {
    @apply border-2 border-black px-2 py-1 outline-none;
  }

  textarea {
    @apply resize-none
  }

  .deletebutton {
    @apply w-fit self-end p-1 bg-neutral-100 hover:bg-red-300 transition;
  }
</style>