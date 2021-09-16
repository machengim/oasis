<script lang="ts">
  import Spinner from "../components/Spinner.svelte";

  export let filePath: string;
  let isLoading = false;
  let currentPage = 1;
  let totalPage = 0;
  let container: HTMLElement;
  let pdf: any;

  $: if (filePath) {
    reset();
    loadPdf();
  }

  $: if (pdf && currentPage <= totalPage) {
    renderPdf();
  } else if (pdf && currentPage > totalPage) {
    pdf.cleanup();
    pdf.destroy();
  }

  const reset = () => {
    pdf = null;
    container = null;
    isLoading = false;
    currentPage = 1;
    totalPage = 0;
  };

  const loadPdf = async () => {
    const pdfjs = window["pdfjs-dist/build/pdf"];
    pdfjs.GlobalWorkerOptions.workerSrc = "/pdf.worker.min.js";

    isLoading = true;
    const currentPath = filePath;
    try {
      const result = await pdfjs.getDocument(filePath).promise;
      isLoading = false;

      if (filePath !== currentPath) {
        return;
      }

      pdf = result;
      totalPage = pdf.numPages;
    } catch (e) {
      console.error(e);
    }
  };

  const renderPdf = async () => {
    const page = await pdf.getPage(currentPage);
    const scale = 2.0;
    const resolution = 2;
    const viewport = page.getViewport({ scale });
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    canvas.height = viewport.height * resolution;
    canvas.width = viewport.width * resolution;
    canvas.style.height = "100%";
    canvas.style.width = "100%";

    const renderContext = {
      canvasContext: context,
      viewport,
      transform: [resolution, 0, 0, resolution, 0, 0],
    };
    await page.render(renderContext).promise;
    container.appendChild(canvas);
    page.cleanup();
    currentPage++;
  };
</script>

{#if isLoading}
  <div class="w-full img-height py-6 flex flex-row justify-center items-center">
    <Spinner />
  </div>
{:else}
  <div
    class="mx-2 p-2 w-full viewer-height overflow-y-auto border-2 border-gray-500 shadow"
    bind:this={container}
  />
{/if}

<style>
  .viewer-height {
    min-height: 30rem;
    max-height: 80vh;
  }
</style>
