<script lang="ts">
  import { onMount } from "svelte";
  import { gamePath } from "./stores";
  import { invoke } from "@tauri-apps/api/core";

  onMount(async () => {
    gamePath.set(await invoke("get_game_path"))
  })
</script>

<slot />
