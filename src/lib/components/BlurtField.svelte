<form class="flex flex-col gap-2">
  <div>
    <label for="question">Kérdés</label>
    <input type="text" name="question" id="question" />
  </div>

  <div>
    <label for="anwser">Válasz</label>
    <textarea name="anwser" id="anwser" rows="6" />
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