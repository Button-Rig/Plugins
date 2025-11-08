<script lang="ts">
  import FilePicker from "$lib/components/FilePicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";
  import { debounce } from "lodash";

  let file = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--file-path") {
      if (args[2]) {
        file = args[2];
      }
    }
  });

  function save() {
    let data: string[] | ErrorPayload;
    if (file) {
      data = ["open-file", "--file-path", file];
    } else {
      data = new ErrorPayload("No file selected.");
    }
    debounce(() => {
      saveHandlerArgs(data);
    }, 500);
  }
</script>

<div class="container">
  <span>File</span>
  <FilePicker bind:file onchange={save} />
</div>
