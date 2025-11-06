<script lang="ts">
  import FolderPicker from "$lib/components/FolderPicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  let folder = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--folder-path") {
      if (args[2]) {
        folder = args[2];
      }
    }
  });

  saveHandlerArgs(() => {
    if (folder) {
      return ["open-folder", "--folder-path", folder];
    } else {
      return new ErrorPayload("No folder selected.");
    }
  });
</script>

<div class="container">
  <span>Folder</span>
  <FolderPicker bind:folder />
</div>
