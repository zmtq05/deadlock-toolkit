<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { gamePath } from "./stores";
  import { open } from "@tauri-apps/plugin-dialog";
  import DownloadControl from "./DownloadControl.svelte";
  let error = "";
  async function selectPath() {
    const selected = await open({
      directory: true,
      title: "Deadlock 폴더 선택",
    });
    try {
      await invoke("change_game_path", { path: selected });
      error = "";
      $gamePath = selected;
    } catch (e: any) {
      error = e;
    }
  }
</script>

<main class="container">
  <div>
    <p>Deadlock 경로</p>
    <div class="flex">
      <input type="text" disabled bind:value={$gamePath} />
      <button on:click={selectPath}>폴더 선택</button>
    </div>
    {#if error}
      <p style="color: red;">{error}</p>
    {/if}
  </div>

  {#each ["translation", "builtin_font", "external_font"] as target}
    <DownloadControl {target} />
  {/each}
</main>

<style>
  .flex {
    display: flex;
    gap: 1em;
  }
  .flex input {
    flex-grow: 1;
  }
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
