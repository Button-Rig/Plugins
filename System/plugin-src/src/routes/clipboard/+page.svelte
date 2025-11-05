<script lang="ts">
  import FilePicker from "$lib/components/FilePicker.svelte";
  import { loadHandlerArgs, saveHandlerArgs } from "buttonrig";
  import { ErrorPayload } from "buttonrig/dist/types";

  class Clipboard {
    type: string;
    filePath: string | null;
    textContent: string | null;

    constructor(type: string) {
      this.type = type;
      this.filePath = null;
      this.textContent = null;
    }

    isText(): boolean {
      return this.type == "text";
    }

    isFile(): boolean {
      return this.type == "file";
    }

    static text(value: string | null): Clipboard {
      let clipboard = new Clipboard("text");
      if (value) {
        clipboard.textContent = value;
      }
      return clipboard;
    }

    static file(): Clipboard {
      return new Clipboard("file");
    }
  }

  let clipboard = $state(Clipboard.text(null));

  loadHandlerArgs((args) => {
    if (args[1] == "--value") {
      if (args[2]) {
        clipboard = Clipboard.text(args[2]);
      }
    }
  });

  saveHandlerArgs(() => {
    if (clipboard.isText()) {
      if (clipboard.textContent) {
        return ["copy-to-clipboard", "--value", clipboard.textContent];
      } else {
        return new ErrorPayload("Clipboard text not set.");
      }
    } else {
      return ["copy-to-clipboard", "--value"];
    }
  });
</script>

<div class="container">
  <span>Content Type</span>
  <div style="display: flex; gap: 20px;">
    <div>
      <input
        id="text"
        onclick={() => (clipboard = Clipboard.text(null))}
        name="content-type"
        type="radio"
        checked={clipboard.type == "text"}
      />
      <label class="input-label" for="text">Text</label>
    </div>
    <div>
      <input
        id="file"
        onclick={() => (clipboard = Clipboard.file())}
        name="content-type"
        type="radio"
        checked={clipboard.isFile()}
      />
      <label class="input-label" for="file">File</label>
    </div>
  </div>
</div>

{#if clipboard.isFile()}
  <div id="file-content" class="container">
    <span>File</span>
    <FilePicker bind:file={clipboard.filePath} />
  </div>
{/if}

{#if clipboard.isText()}
  <div id="text-content" class="container">
    <span>Text</span>
    <textarea bind:value={clipboard.textContent} rows="2"></textarea>
  </div>
{/if}
