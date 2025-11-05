<script lang="ts">
  import { pickFile } from "buttonrig";

  let { file = $bindable() }: { file: string | null } = $props();

  async function selectFile() {
    file = await pickFile([]);
    console.log("file", file);
  }
</script>

<div class="selcted-file-container">
  <div class="selcted-file">{file}</div>
  <button onclick={selectFile}>Select file</button>
</div>

<style>
  .selcted-file-container {
    display: flex;
    flex-direction: row;
    gap: 2%;
  }

  .selcted-file-container button {
    width: 15%;
  }

  .selcted-file {
    width: 83%;
    height: 35px;
    background-color: var(--secondary-color);
    border-radius: 5px;
    display: flex;
    align-items: center;
    padding: 0px 10px;
  }
</style>
