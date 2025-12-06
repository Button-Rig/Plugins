<script lang="ts">
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";
  import { debounce, type DebouncedFunc } from "lodash";

  let delay = $state(0);

  loadHandlerArgs((args) => {
    if (args[1] == "--milliseconds") {
      if (args[2]) {
        delay = Number.parseInt(args[2]);
      }
    }
  });

  let debouncedSave:  DebouncedFunc<() => void> | null = null;
  function save() {
    if (!debouncedSave) {
      debouncedSave = debounce(() => {
        saveHandlerArgs(["delay", "--milliseconds", delay.toString()]);
      }, 500);
    }
    debouncedSave();
  }

  $effect(() => {
    delay;
    save();
  });
</script>

<div class="container">
  <span>Milliseconds</span>
  <input type="number" min="0" bind:value={delay} />
</div>
