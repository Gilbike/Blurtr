<script lang="ts">
  import {blurt} from "$lib/blurt";
  import {goto} from "$app/navigation";

  let inputString: string = "";
  let incorrectInput: boolean = false;

  function importBlurt() {
    try {
      const data = JSON.parse(inputString);
      if (data.title == undefined || data.fields == undefined || data.fields.length == 0) throw new Error("incorrect json");

      blurt.set(data.fields);
      goto("/?refresh=true");
    } catch (e) {
      incorrectInput = true;
    }
  }
</script>

<div class="p-2">
  <div class="flex flex-row justify-between items-center">
    <h1 class="font-bold text-2xl">Importálás</h1>
    <a href="/">Mégse</a>
  </div>
  {#if incorrectInput}
    <p class="font-light mt-2 text-red-500 text-sm">Hibás JSON</p>
  {/if}
  <textarea
    on:keydown={() => incorrectInput = false}
    bind:value={inputString}
    class="w-full outline-none p-2 bg-neutral-100 my-2 border-[1px] border-solid border-black resize-none"
    class:border-red-500={incorrectInput}
    rows="19"
  ></textarea>
  <button disabled={inputString.length == 0} class="disabled:bg-emerald-400/30 bg-emerald-400 hover:bg-emerald-500 transition w-full py-1 text-white" on:click={importBlurt}>Importálás</button>
</div>