<script lang="ts">
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  let delay = $state(0);

  loadHandlerArgs((args)=> {
    if (args[1] == "--milliseconds") {
      if (args[2]) {
        delay = Number.parseInt(args[2]);
      }
    }
  });
  
  saveHandlerArgs(() => {
      return ["delay", "--milliseconds", delay.toString()]
  });
</script>

<div class="container">
    <span>Milliseconds</span>
    <input type="number" min="0" bind:value={delay} />    
</div>
