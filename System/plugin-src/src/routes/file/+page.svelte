<script lang="ts">
  import FilePicker from "$lib/components/FilePicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  let file = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--file-path") {
      if (args[2]) {
        file = args[2];
      }
    }
  });

  saveHandlerArgs(() => {
    if (file) {
      return ["open-file", "--file-path", file];
    } else {
      return new ErrorPayload("No file selected.");
    }
  });
</script>

<div class="container">
  <span>File</span>
  <FilePicker bind:file />
</div>
