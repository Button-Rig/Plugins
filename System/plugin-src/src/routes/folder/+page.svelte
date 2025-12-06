<script lang="ts">
  import FolderPicker from "$lib/components/FolderPicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";
  import { debounce, type DebouncedFunc } from "lodash";

  let folder = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--folder-path") {
      if (args[2]) {
        folder = args[2];
      }
    }
  });

  function save() {
    let data: string[] | ErrorPayload;
    if (folder) {
      data = ["open-folder", "--folder-path", folder];
    } else {
      data = new ErrorPayload("No folder selected.");
    }
    saveHandlerArgs(data);
  }
</script>

<div class="container">
  <span>Folder</span>
  <FolderPicker bind:folder onchange={save} />
</div>
