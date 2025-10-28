<script lang="ts">
    class Clipboard {
        type: string;
        filePath: string | null;
        textContent: string | null;

        constructor(type: string) {
            this.type = type;
            this.filePath = null;
            this.textContent = null;
        }

        isText() : boolean {
            return this.type == "text";
        }

        isFile() : boolean {
            return this.type == "file";
        }

        static text() : Clipboard {
            return new Clipboard("text");
        }

        static file() : Clipboard {
            return new Clipboard("file");
        }
    }
  let clipboard = $state(Clipboard.text());
  
</script>

<div class="container">
  <span>Content Type</span>
  <div style="display: flex; gap: 20px;">
    <div>
      <input
        id="text"
        onclick={() => (clipboard = Clipboard.text())}
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
    <div class="selcted-file-container">
      <div class="selcted-file"></div>
      <button>Select file</button>
    </div>
  </div>
{/if}

{#if clipboard.isText()}
  <div id="text-content" class="container">
    <span>Text</span>
    <textarea rows="2"></textarea>
  </div>
{/if}

<style>
  .selcted-file-container {
    display: flex;
    flex-direction: row;
    gap: 2%;
    
  }

  .selcted-file-container button {
    width: 15%;
  }

  .selcted-file {
    width: 83%;
    height: 35px;
    background-color: var(--secondary-color);
    border-radius: 5px;
  }
</style>
