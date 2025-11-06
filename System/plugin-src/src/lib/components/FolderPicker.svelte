<script lang="ts">
  import { pickFolder } from "buttonrig";

  let { folder = $bindable() }: { folder: string | null } = $props();

  async function selectfolder() {
    folder = await pickFolder();
  }
</script>

<div class="selcted-folder-container">
  <div class="selcted-folder">{folder}</div>
  <button onclick={selectfolder}>Select folder</button>
</div>

<style>
  .selcted-folder-container {
    display: flex;
    flex-direction: row;
    gap: 2%;
  }

  .selcted-folder-container button {
    width: 15%;
  }

  .selcted-folder {
    width: 83%;
    height: 35px;
    background-color: var(--secondary-color);
    border-radius: 5px;
    display: flex;
    align-items: center;
    padding: 0px 10px;
  }
</style>
