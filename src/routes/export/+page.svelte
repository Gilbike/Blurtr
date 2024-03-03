<script lang="ts">
  import {page} from "$app/stores";
  import {invoke} from "@tauri-apps/api"

  const defaultJSON = "{title: '', fields: []}";

  const data = JSON.parse($page.url.searchParams.get("data") || defaultJSON);

  function closeWindow() {
    invoke("close_export");
  }
</script>

<div class="p-2">
  <h1 class="font-bold text-2xl">Exportálás</h1>
<pre class="p-2 bg-neutral-100 my-2 border-[1px] border-solid border-black">
{JSON.stringify(data, null, 4)}
</pre>
  <button class="bg-red-400 hover:bg-red-500 transition w-full py-1 text-white" on:click={closeWindow}>Bezárás</button>
</div>