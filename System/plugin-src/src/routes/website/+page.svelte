<script lang="ts">
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  let websiteUrl = $state<string | null>(null);

  loadHandlerArgs((args)=> {
    if (args[1] == "--url") {
      if (args[2]) {
        websiteUrl = args[2];
      }
    }
  });
  
  saveHandlerArgs(() => {
      if (websiteUrl) {
        return ["open-website", "--url", websiteUrl];
      } else {
        return new ErrorPayload("Website url not set.");
      }
  });
</script>

<div class="container">
    <span>Website Url</span>
    <input type="text" bind:value={websiteUrl} />    
</div>
