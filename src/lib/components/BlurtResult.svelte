<script lang="ts">
  export let question: string;
  export let answer: string;
  export let userAnswer: string;
</script>

<div class="flex flex-col gap-2">
  <div>
    <label for="question">Kérdés</label>
    <p class="text-lg font-semibold">{question}</p>
  </div>

  <div class="anwsers">
    <div>
      <label for="anwser">Általad megadott válasz</label>
      <textarea rows="6" disabled value={userAnswer} />
    </div>
    <div>
      <label for="anwser">Helyes válasz</label>
      <textarea rows="6" disabled value={answer} />
    </div>
  </div>
</div>

<style scoped>
  div * {
    @apply block w-full;
  }

  label {
    @apply text-neutral-800 font-light;
  }

  textarea {
    @apply border-2 border-black px-2 py-1 outline-none resize-none;
  }

  .anwsers {
    @apply flex flex-row gap-2;
  }
</style>