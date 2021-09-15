<script lang="ts">
  import pdfjs from "pdfjs-dist";
  import pdfjsWorker from "pdfjs-dist/build/pdf.worker.entry";
  import Spinner from "../components/Spinner.svelte";
  import type { PDFDocumentProxy } from "pdfjs-dist/types/display/api";

  export let filePath: string;
  let isLoading = false;
  let pageNumber = 1;
  let canvas: HTMLCanvasElement;
  let pdf: PDFDocumentProxy;

  $: if (filePath) {
    reset();
    loadPdf();
  }

  $: if (pdf) {
    renderPdf();
  }

  const reset = () => {
    isLoading = false;
    pageNumber = 1;
    pdf = null;
  };

  const loadPdf = async () => {
    isLoading = true;
    const currentPath = filePath;
    pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;

    try {
      const result = await pdfjs.getDocument(filePath).promise;
      isLoading = false;

      if (filePath !== currentPath) {
        return;
      }

      pdf = result;
    } catch (e) {
      console.error(e);
    }
  };

  const renderPdf = async () => {
    let page = await pdf.getPage(pageNumber);
    const scale = 1.0;
    const resolution = 2;
    const viewport = page.getViewport({ scale });
    const context = canvas.getContext("2d");
    canvas.height = viewport.height * resolution;
    canvas.width = viewport.width * resolution;

    const renderContext = {
      canvasContext: context,
      viewport,
      transform: [resolution, 0, 0, resolution, 0, 0],
    };
    const renderTask = page.render(renderContext);
    await renderTask.promise;
    page = null;
  };
</script>

{#if isLoading}
  <div class="w-full img-height py-6 flex flex-row justify-center items-center">
    <Spinner />
  </div>
{:else}
  <div
    class="mx-2 p-2 w-full viewer-height overflow-y-auto border-2 border-gray-500 shadow"
  >
    <canvas bind:this={canvas} />
  </div>
{/if}

<style>
  .viewer-height {
    min-height: 30rem;
    max-height: 80vh;
  }

  canvas {
    direction: ltr;
    width: 100%;
    height: 100%;
  }
</style>
