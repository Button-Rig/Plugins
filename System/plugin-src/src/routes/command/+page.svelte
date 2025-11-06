<script lang="ts">
  import FolderPicker from "$lib/components/FolderPicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  let path = $state<string | null>(null);
  let command = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--command") {
      if (args[2]) {
        command = args[2];
      }
    }
  });

  saveHandlerArgs(() => {
    if (command) {
      let handlerArgs = ["run-command", "--command", command];
      if (path) {
        handlerArgs.push("--path");
        handlerArgs.push(path);
      }
      return handlerArgs;
    } else {
      return new ErrorPayload("Command not set.");
    }
  });
</script>

<div class="container">
  <div class="container">
    <span>Path</span>
    <FolderPicker bind:folder={path} />
  </div>
  <div class="container">
    <span>Command</span>
    <input type="text" bind:value={command} />
  </div>
</div>
