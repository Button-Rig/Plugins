<script lang="ts">
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";
  import { debounce, type DebouncedFunc } from "lodash";

  let websiteUrl = $state<string | null>(null);

  loadHandlerArgs((args) => {
    if (args[1] == "--url") {
      if (args[2]) {
        websiteUrl = args[2];
      }
    }
  });

  let debouncedSave: DebouncedFunc<() => void> | null = null;
  function save() {

    if (!debouncedSave) {
      debouncedSave = debounce(() => {
        let data: string[] | ErrorPayload;
        if (websiteUrl) {
          console.log(websiteUrl);
          data = ["open-website", "--url", websiteUrl];
        } else {
          data = new ErrorPayload("Website url not set.");
        }
        console.log(data);

        saveHandlerArgs(data);
      }, 500);
    }
    debouncedSave();
  }

  $effect(() => {
    websiteUrl;
    save();
  });
</script>

<div class="container">
  <span>Website Url</span>
  <input type="text" bind:value={websiteUrl} />
</div>
