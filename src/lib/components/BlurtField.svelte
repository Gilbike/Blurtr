<script lang="ts">
  export let question: string;
  export let answer: string;

  export let mode: "creation" | "input" = "creation";
</script>

<form class="flex flex-col gap-2">
  <div>
    <label for="question">Kérdés</label>
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
</form>

<style scoped>
  form * {
    @apply block w-full;
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
</style>