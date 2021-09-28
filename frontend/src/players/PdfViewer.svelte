<script lang="ts">
  import PdfObject from "pdfobject";

  export let filePath: string;
  let container: HTMLElement;

  $: if (filePath && container) {
    loadPdf();
  }

  const loadPdf = () => {
    const options = {
      pdfOpenParams: {
        navpanes: 1,
        toolbar: 1,
        statusbar: 1,
        pagemode: "none",
        view: "FitV",
      },
      forcePDFJS: true,
      PDFJS_URL: "/vendor/pdfjs/web/viewer.html",
    };

    PdfObject.embed(filePath, "#container", options);
  };
</script>

<div
  class="p-2 w-full viewer-height overflow-y-auto"
  id="container"
  bind:this={container}
/>
