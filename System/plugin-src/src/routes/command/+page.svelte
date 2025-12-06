<script lang="ts">
  import FolderPicker from "$lib/components/FolderPicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";
  import { debounce, type DebouncedFunc } from "lodash";

  let path = $state<string | null>(null);
  let command = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--command") {
      if (args[2]) {
        command = args[2];
      }
      if (args[3] == "--path") {
        if (args[4]) {
          path = args[4];
        }
      }
    }
  });

  let debouncedSave: DebouncedFunc<() => void> | null = null;
  function save() {
    if (!debouncedSave) {
      debouncedSave = debounce(() => {
        let data: string[] | ErrorPayload;
        if (command) {
          data = ["run-command", "--command", command];
          if (path) {
            data.push("--path");
            data.push(path);
          }
        } else {
          data = new ErrorPayload("Command not set.");
        }
        saveHandlerArgs(data);
      }, 500);
    }
    debouncedSave();
  }

  $effect(() => {
    command;
    save();
  });
</script>

<div class="container">
  <div class="container">
    <span>Path</span>
    <FolderPicker bind:folder={path} onchange={save} />
  </div>
  <div class="container">
    <span>Command</span>
    <input type="text" bind:value={command} />
  </div>
</div>
