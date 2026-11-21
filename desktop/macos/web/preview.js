/** Canvas viewer with a single in-flight page render and independent page positions. */
export class PdfPreview {
  #surface;
  #canvas;
  #wrap;
  #empty;
  #message;
  #controls;
  #previous;
  #next;
  #pageField;
  #pageTotal;
  #library;
  #loading;
  #pdf;
  #drawTask;
  #url = "";
  #page = 1;
  #generation = 0;
  #drawGeneration = 0;
  #resizeFrame;

  constructor(elements) {
    this.#surface = elements.surface;
    this.#canvas = elements.canvas;
    this.#wrap = elements.wrap;
    this.#empty = elements.empty;
    this.#message = elements.message;
    this.#controls = elements.controls;
    this.#previous = elements.previous;
    this.#next = elements.next;
    this.#pageField = elements.pageField;
    this.#pageTotal = elements.pageTotal;
    this.#previous.addEventListener("click", () => this.goTo(this.#page - 1));
    this.#next.addEventListener("click", () => this.goTo(this.#page + 1));
    this.#pageField.addEventListener("change", () => this.goTo(Number(this.#pageField.value)));
    this.#pageField.addEventListener("keydown", (event) => {
      if (event.key === "Enter") this.#pageField.blur();
    });
    const turnPage = (event) => {
      if (event.target === this.#pageField || !["ArrowLeft", "ArrowRight"].includes(event.key)) return;
      event.preventDefault();
      this.goTo(this.#page + (event.key === "ArrowRight" ? 1 : -1));
    };
    this.#surface.addEventListener("keydown", turnPage);
    this.#controls.addEventListener("keydown", turnPage);
    new ResizeObserver(() => {
      cancelAnimationFrame(this.#resizeFrame);
      this.#resizeFrame = requestAnimationFrame(() => this.#render());
    }).observe(this.#surface);
  }

  #updateControls() {
    const total = this.#pdf?.numPages || 0;
    this.#controls.hidden = this.#wrap.hidden || total === 0;
    this.#pageField.value = this.#page;
    this.#pageTotal.textContent = total;
    this.#previous.disabled = this.#page < 2;
    this.#next.disabled = this.#page >= total;
  }

  #announce(message, isError = false) {
    this.#message.hidden = !message;
    this.#message.textContent = message;
    this.#message.toggleAttribute("data-error", isError);
  }

  async show(url, keepPage = false) {
    if (url === this.#url) return;
    const generation = ++this.#generation;
    this.#drawTask?.cancel();
    this.#loading?.destroy().catch(() => {});
    this.#loading = undefined;
    this.#pdf = undefined;
    this.#url = url;
    if (!keepPage) this.#page = 1;
    this.#wrap.hidden = true;
    this.#empty.hidden = Boolean(url);
    this.#updateControls();
    this.#announce(url ? "Loading preview…" : "");
    if (!url) return;

    try {
      this.#library ||= import("/pdfjs/build/pdf.min.mjs").catch((error) => {
        this.#library = undefined;
        throw error;
      });
      const pdfjs = await this.#library;
      if (generation !== this.#generation) return;
      pdfjs.GlobalWorkerOptions.workerSrc = "/pdfjs/build/pdf.worker.min.mjs";
      const loading = pdfjs.getDocument({
        url,
        cMapUrl: "/pdfjs/cmaps/",
        cMapPacked: true,
        standardFontDataUrl: "/pdfjs/standard_fonts/",
        wasmUrl: "/pdfjs/wasm/",
        iccUrl: "/pdfjs/iccs/",
      });
      this.#loading = loading;
      const pdf = await loading.promise;
      if (generation !== this.#generation) return;
      this.#pdf = pdf;
      this.goTo(this.#page);
    } catch {
      if (generation === this.#generation) this.#announce("Couldn’t preview this PDF", true);
    }
  }

  goTo(page) {
    if (!this.#pdf) return;
    this.#page = Number.isInteger(page) ? Math.min(this.#pdf.numPages, Math.max(1, page)) : this.#page;
    this.#updateControls();
    this.#render();
  }

  async #render() {
    const pdf = this.#pdf;
    const width = this.#surface.clientWidth;
    const height = this.#surface.clientHeight;
    if (!pdf || width < 35 || height < 35) return;
    const generation = this.#generation;
    const drawGeneration = ++this.#drawGeneration;
    const current = () => generation === this.#generation && drawGeneration === this.#drawGeneration;
    this.#drawTask?.cancel();
    try {
      const page = await pdf.getPage(this.#page);
      if (!current()) return;
      const size = page.getViewport({ scale: 1 });
      const scale = Math.min((width - 24) / size.width, (height - 24) / size.height);
      const viewport = page.getViewport({ scale });
      const pixelRatio = Math.min(2, window.devicePixelRatio || 1);
      const staged = document.createElement("canvas");
      staged.width = Math.ceil(viewport.width * pixelRatio);
      staged.height = Math.ceil(viewport.height * pixelRatio);
      const render = page.render({
        canvas: staged,
        canvasContext: staged.getContext("2d"),
        viewport,
        transform: pixelRatio === 1 ? null : [pixelRatio, 0, 0, pixelRatio, 0, 0],
      });
      this.#drawTask = render;
      await render.promise;
      if (!current()) return;
      this.#drawTask = undefined;
      this.#canvas.width = staged.width;
      this.#canvas.height = staged.height;
      this.#canvas.style.width = `${viewport.width}px`;
      this.#canvas.style.height = `${viewport.height}px`;
      this.#canvas.getContext("2d").drawImage(staged, 0, 0);
      this.#wrap.hidden = false;
      this.#announce("");
      this.#updateControls();
    } catch (error) {
      if (current() && error?.name !== "RenderingCancelledException") {
        this.#announce("Couldn’t display this page", true);
      }
    }
  }
}
