import { PdfPreview } from "/preview.js";
import { createInlineSelect } from "/inline-select.js";

const byId = (id) => document.getElementById(id);
const setText = (element, value) => {
  if (element.textContent !== value) element.textContent = value;
};

function setPreviewSketch() {
  const sketches = [
    "leon",
    "shawshank",
    "the-tiger-and-the-snow",
    "resurrection",
    "the-last-10-years",
    "the-handmaiden",
    "hana-and-alice",
    "bright-heart",
    "cutting-it-short",
    "confessions",
    "milk-of-sorrow",
    "fong-sai-yuk-ii",
    "yumeji",
    "close-up",
    "what-dreams-may-come",
    "taste-of-tea",
    "purple-sunset",
  ];
  const cookie = "aeolus_workspace_sketch";
  let previous;
  try {
    // The loopback port changes on launch, so use a host-scoped preference.
    previous = document.cookie.split("; ").find((entry) => entry.startsWith(cookie + "="))?.slice(cookie.length + 1);
  } catch {
    // A fresh random choice still works when cookies are unavailable.
  }
  const choices = sketches.filter((sketch) => sketch !== previous);
  const sketch = choices[Math.floor(Math.random() * choices.length)];
  byId("document-preview").style.setProperty("--stage-sketch", `url("/sketches/${sketch}.webp")`);

  try {
    document.cookie = `${cookie}=${sketch}; Path=/; Max-Age=31536000; SameSite=Strict`;
  } catch {
    // Remembering the last choice is optional for rendering the workspace.
  }
}

const sourceToggle = byId("source-toggle");
const upload = byId("workspace-upload");
const uploadList = byId("upload-list");
const fileInput = byId("file-input");
const linkForm = byId("link-form");
const linkInput = byId("workspace-link");
const linkImport = byId("workspace-link-import");
const docSelect = byId("workspace-files");
const previewFilename = byId("preview-filename");
const versions = byId("workspace-preview-version");
const previewPane = byId("workspace-preview");
const workspaceStatus = byId("workspace-job-status");
const languageSwap = byId("workspace-swap");
const settingsProvider = byId("settings-provider");
const profileHeading = byId("settings-profile-heading");
const customProfileSections = document.querySelectorAll("[data-custom-profile]");
const profileList = byId("settings-profile-list");
const profileName = byId("settings-profile-name");
const profileModel = byId("settings-profile-model");
const profileUrl = byId("settings-profile-url");
const profileKey = byId("settings-profile-key");
const profileDelete = byId("settings-profile-delete");
const profileTest = byId("settings-profile-test");
const profileSave = byId("settings-profile-save");
const profileApply = byId("settings-profile-apply");
const profileStatus = byId("settings-profile-status");
const settingsStatus = byId("settings-status");
const downloadControl = byId("download-control");
const downloadButton = byId("workspace-download-button");
const downloads = byId("workspace-downloads");

const languageOptions = [
  ["auto", "Auto-detect"], ["en", "English"], ["zh-CN", "Simplified Chinese"], ["zh-TW", "Traditional Chinese (Taiwan)"],
  ["zh-HK", "Traditional Chinese (Hong Kong)"], ["ja", "Japanese"], ["ko", "Korean"], ["fr", "French"], ["de", "German"],
  ["es", "Spanish"], ["ru", "Russian"], ["it", "Italian"], ["pt", "Portuguese"], ["ar", "Arabic"],
  ["hi", "Hindi"], ["id", "Indonesian"], ["nl", "Dutch"], ["sv", "Swedish"], ["th", "Thai"],
  ["tr", "Turkish"], ["uk", "Ukrainian"], ["vi", "Vietnamese"]
];
const standardService = "SiliconFlowFree";
const standardLabel = "SiliconFlow Free";
const servicePresets = new Map([
  ["openai", { name: "OpenAI · GPT-6 Luna", url: "https://api.openai.com/v1", model: "gpt-6-luna" }],
  ["deepseek", { name: "DeepSeek · Flash", url: "https://api.deepseek.com", model: "deepseek-flash" }],
  ["gemini", {
    name: "Google · Gemini 4 Hydrogen",
    url: "https://generativelanguage.googleapis.com/v1beta/openai",
    model: "gemini-4-hydrogen",
  }],
  ["kimi", {
    name: "Kimi · K3.1 Pro (Global)",
    legacyName: "Kimi · K3.1 Pro（国际）",
    url: "https://api.moonshot.ai/v1",
    model: "kimi-k3.1-pro",
  }],
]);
byId("settings-model-presets").replaceChildren(
  ...Array.from(servicePresets, ([id, preset]) => new Option(preset.name, id)),
);
const inlineSelects = new Map(
  Array.from(document.querySelectorAll("#yepage-main select, #yepage-settings select"), (select) => [
    select, createInlineSelect(select, closeInlineSelects),
  ]),
);

function closeInlineSelects(scope = document) {
  for (const [select, picker] of inlineSelects) {
    if (scope.contains(select)) picker.close();
  }
}

document.addEventListener("click", (event) => {
  for (const [select, picker] of inlineSelects) {
    if (!select.parentElement.contains(event.target)) picker.close();
  }
});

const working = new Set(["pending", "queued", "starting", "preparing", "running", "translating", "cancelling", "canceling"]);
const finished = new Set(["completed", "done", "succeeded"]);
const retryable = new Set(["failed", "error", "cancelled", "canceled"]);
const knownFormats = ["both", "mono", "dual"];
const importLabel = "Importing from link…";
const uploadLabel = "Adding PDFs…";
const cancelingLabel = "Canceling…";
const canceledLabel = "Canceled";

let snapshot = { documents: [], profiles: {} };
let options = {
  from: "en", to: "zh-CN", pages: "", format: "both",
  translated_only: false, translated_first: false, alternate: false, prompt: "", json_mode: false,
  glossary: false, min_text_length: 5, font_family: "",
};
let source = "file";
let chosen = { file: null, link: null };
const versionChoices = new Map();
let stagedAction = "";
let apiReady = false;
let workspaceNotice = "";
let workspaceError = false;
let profilePending = false;
let settingsPending = false;
let cancelingImport = false;
let readSequence = 0;
let appliedSequence = 0;
let refreshTimer;
let priorPreviewId;
let activeProfile;
let selectedProvider = standardService;
const resultRevisions = new Map();

function localUrl(value) {
  if (typeof value !== "string" || !value) return "";
  try {
    const url = new URL(value, location.href);
    return url.origin === location.origin ? url.href : "";
  } catch {
    return "";
  }
}

const viewer = new PdfPreview({
  surface: byId("document-preview"),
  canvas: byId("pdf-canvas"),
  wrap: byId("canvas-wrap"),
  empty: byId("workspace-preview-empty"),
  message: byId("preview-message"),
  controls: byId("preview-navigation"),
  previous: byId("preview-prev"),
  next: byId("preview-next"),
  pageField: byId("preview-page-index"),
  pageTotal: byId("preview-page-total"),
});

function status(element, text, isError = false) {
  setText(element, text);
  element.toggleAttribute("data-error", Boolean(isError && text));
}

function optionsFor(select, entries, value) {
  // Keep option nodes stable so an open picker preserves focus during refreshes.
  if (select.options.length !== entries.length || entries.some(([key, label], index) => {
    const option = select.options[index];
    return option.value !== String(key) || option.text !== label;
  })) {
    select.replaceChildren(...entries.map(([key, label]) => new Option(label, String(key))));
  }
  if (select.value !== value) select.value = value;
  inlineSelects.get(select)?.sync();
}

function selectedScope() {
  return snapshot.documents.filter((document) => document.source === source);
}

function selectedDocument() {
  const docs = selectedScope();
  return docs.find((document) => String(document.id) === chosen[source]) || docs[0];
}

function selectedVersion(document) {
  if (!document) return "original";
  const chosenVersion = versionChoices.get(String(document.id));
  if (chosenVersion && localUrl(document.versions?.[chosenVersion])) return chosenVersion;
  return localUrl(document.versions?.dual) ? "dual" : localUrl(document.versions?.mono) ? "mono" : "original";
}

function isWorking() {
  return working.has(snapshot.job?.status);
}

function isImporting() {
  return snapshot.importing || stagedAction === importLabel;
}

function updateLinkImport() {
  const url = linkInput.value.trim();
  linkImport.disabled = linkInput.disabled || !linkInput.validity.valid || !/^https?:\/\//i.test(url);
}

function setDownloadMenu(open, returnFocus = false) {
  if (downloadControl.hidden) open = false;
  downloads.hidden = !open;
  downloadButton.setAttribute("aria-expanded", String(open));
  if (returnFocus) downloadButton.focus();
}

function downloadAnchor(label, path) {
  const src = localUrl(path);
  if (!src) return null;
  const href = new URL(src);
  href.searchParams.set("download", "1");
  const anchor = document.createElement("a");
  anchor.textContent = label;
  anchor.href = href.href;
  anchor.download = "";
  anchor.addEventListener("click", () => setDownloadMenu(false, true));
  return anchor;
}

function updateDownloads(doc) {
  const labels = { mono: "Translated PDF", dual: "Bilingual PDF", glossary: "Glossary" };
  const zipLabels = { all: "All results (ZIP)", mono: "Translated PDFs (ZIP)", dual: "Bilingual PDFs (ZIP)", glossary: "Glossaries (ZIP)" };
  const links = Object.entries(labels)
    .map(([kind, label]) => downloadAnchor(label, doc?.versions?.[kind])).filter(Boolean);
  const archives = Object.entries(zipLabels)
    .map(([kind, label]) => downloadAnchor(label, snapshot.archives?.[kind])).filter(Boolean);
  downloadControl.hidden = links.length + archives.length === 0;
  const entries = links.concat(links.length && archives.length ? [document.createElement("hr")] : [], archives);
  const signature = JSON.stringify(entries.map((link) => [link.textContent, link.href]));
  if (downloads.dataset.signature !== signature) {
    const active = downloads.contains(document.activeElement) ? document.activeElement.href : "";
    downloads.replaceChildren(...entries);
    downloads.dataset.signature = signature;
    if (downloadControl.hidden) setDownloadMenu(false);
    else if (active) (entries.find(({ href }) => href === active) || downloadButton).focus();
  }
}

function selectDocument(id) {
  chosen[source] = String(id);
  renderWorkspace();
}

function retainedParagraphsLabel(count) {
  return `${count} ${count === 1 ? "paragraph" : "paragraphs"}`;
}

function updateUploadList(docs, selected, locked) {
  const signature = JSON.stringify([docs.map((doc) => [doc.id, doc.name, doc.retained_paragraphs]), selected?.id, locked]);
  if (uploadList.dataset.signature === signature) return;
  uploadList.dataset.signature = signature;
  const focusedButton = uploadList.contains(document.activeElement) ? document.activeElement : null;
  const focusedIndex = focusedButton ? [...uploadList.children].indexOf(focusedButton.parentElement) : -1;
  const items = docs.map((doc) => {
    const li = document.createElement("li");
    const show = document.createElement("button");
    show.type = "button";
    show.className = "document-name";
    show.dataset.documentId = doc.id;
    show.textContent = doc.name;
    show.title = doc.retained_paragraphs
      ? `${doc.name} · Original text retained in ${retainedParagraphsLabel(doc.retained_paragraphs)}`
      : doc.name;
    if (doc === selected) show.setAttribute("aria-current", "true");
    show.addEventListener("click", () => selectDocument(doc.id));
    const remove = document.createElement("button");
    remove.type = "button";
    remove.className = "remove-document";
    remove.textContent = "×";
    remove.setAttribute("aria-label", `Remove ${doc.name}`);
    remove.disabled = locked;
    remove.addEventListener("click", () => performWorkspace("Removing…", async () => {
      await api(`/api/documents/${encodeURIComponent(doc.id)}`, { method: "DELETE" });
    }));
    li.append(show, remove);
    return li;
  });
  uploadList.replaceChildren(...items);
  uploadList.hidden = !items.length;
  if (focusedIndex >= 0) {
    const focusedId = focusedButton.parentElement.firstElementChild.dataset.documentId;
    const previousRow = items.find((item) => item.firstElementChild.dataset.documentId === focusedId);
    const nearbyRow = previousRow || items[Math.min(focusedIndex, items.length - 1)];
    const keepRemove = previousRow && !locked && focusedButton.matches(".remove-document");
    const nextButton = keepRemove ? previousRow.lastElementChild : nearbyRow?.firstElementChild;
    (nextButton || (source === "file" ? byId("pick-files") : linkInput)).focus();
  }
}

function jobPresentation(available) {
  if (cancelingImport) return [cancelingLabel, "progress"];
  if (stagedAction) return [stagedAction, "progress"];
  if (isImporting()) return [importLabel, "progress"];
  if (workspaceNotice) return [workspaceNotice, workspaceError ? "error" : "neutral"];
  const job = snapshot.job;
  if (isWorking()) {
    const count = job.total > 1 ? ` · ${job.done || 0}/${job.total} done` : "";
    const message = ["canceling", "cancelling"].includes(job.status) ? cancelingLabel : "Translating…";
    return [(job.message || message) + count, "progress"];
  }
  if (["failed", "error"].includes(job?.status)) return [job.error || job.message || "Translation failed", "error"];
  if (["canceled", "cancelled"].includes(job?.status)) return [job.message || canceledLabel, "neutral"];
  const retained = available.reduce((count, doc) => count + (doc.retained_paragraphs ?? 0), 0);
  if (retained) return [`Exported with original text in ${retainedParagraphsLabel(retained)}.`, "warning"];
  if (available.length && finished.has(job?.status)) return ["Translation complete", "success"];
  return ["", ""];
}

function renderWorkspace() {
  const docs = selectedScope();
  const selected = selectedDocument();
  if (!docs.length) chosen[source] = null;
  else chosen[source] = String(selected.id);
  const translating = isWorking();
  const importing = isImporting();
  const locked = !apiReady || translating || importing || Boolean(stagedAction);
  sourceToggle.disabled = locked;
  upload.hidden = source !== "file";
  uploadList.hidden = !docs.length;
  linkForm.hidden = source !== "link";
  byId("pick-files").disabled = locked;
  byId("workspace-preview-empty").disabled = locked;
  setText(byId("preview-empty-label"), source === "link" ? "Paste a PDF link" : "Choose PDF");
  byId("preview-drop-hint").hidden = source === "link";
  linkInput.disabled = locked;
  updateLinkImport();
  updateUploadList(docs, selected, locked);
  const count = byId("file-count");
  count.hidden = docs.length < 2;
  setText(count, `${docs.length} PDFs`);

  byId("document-select").hidden = docs.length < 2;
  if (docs.length > 1) optionsFor(docSelect, docs.map((doc) => [doc.id, doc.name]), String(selected.id));
  else inlineSelects.get(docSelect).close();
  byId("preview-heading").classList.toggle("sr-only", Boolean(selected));
  previewFilename.hidden = docs.length !== 1;
  setText(previewFilename, selected?.name || "");
  previewFilename.title = selected?.name || "";
  versions.hidden = !["mono", "dual"].some((kind) => localUrl(selected?.versions?.[kind]));
  const version = selectedVersion(selected);
  for (const radio of versions.querySelectorAll("input")) {
    radio.closest("label").hidden = !localUrl(selected?.versions?.[radio.value]);
    radio.checked = radio.value === version;
  }
  const path = localUrl(selected?.versions?.[version]);
  const revision = resultRevisions.get(String(selected?.id));
  const url = path ? new URL(path) : null;
  if (url && revision && version !== "original") url.searchParams.set("preview_revision", revision);
  viewer.show(url?.href || "", priorPreviewId === selected?.id);
  priorPreviewId = selected?.id;
  updateDownloads(selected);

  const translate = byId("workspace-translate");
  translate.disabled = locked || !docs.length;
  const translateLabel = docs.length > 1 ? `Translate ${docs.length} PDFs` : "Translate";
  setText(translate, translating ? "Translating…" : docs.length ? translateLabel : "Add a PDF");
  byId("workspace-cancel").hidden = !translating && !importing;
  byId("workspace-cancel").disabled = cancelingImport || ["canceling", "cancelling"].includes(snapshot.job?.status)
    || Boolean(stagedAction && !importing);
  byId("workspace-retry").hidden = translating || !retryable.has(snapshot.job?.status);
  byId("workspace-retry").disabled = locked;
  const progress = byId("workspace-progress");
  const progressValue = byId("workspace-progress-value");
  progress.hidden = !translating && !importing && stagedAction !== uploadLabel;
  const percent = translating && Number.isFinite(snapshot.job.percent)
    ? Math.max(0, Math.min(100, snapshot.job.percent)) : null;
  progressValue.hidden = percent === null;
  if (!progress.hidden) {
    const translationLabel = snapshot.job?.total > 1 ? "Current document progress" : "Translation progress";
    const label = translating ? translationLabel : importing ? "Link import progress" : "File upload progress";
    if (progress.getAttribute("aria-label") !== label) progress.setAttribute("aria-label", label);
  }
  if (percent === null) {
    progress.removeAttribute("value");
  } else {
    if (!progress.hasAttribute("value") || progress.value !== percent) progress.value = percent;
    setText(progressValue, `${Math.round(percent)}%`);
  }
  const [message, tone] = jobPresentation(docs);
  workspaceStatus.classList.toggle("sr-only", !message);
  if (tone) {
    if (workspaceStatus.dataset.tone !== tone) workspaceStatus.dataset.tone = tone;
  } else {
    delete workspaceStatus.dataset.tone;
  }
  if (tone === "error" || tone === "warning") workspaceStatus.tabIndex = 0;
  else workspaceStatus.removeAttribute("tabindex");
  status(workspaceStatus, message, tone === "error");
}

function activeProvider() {
  const service = snapshot.service || {};
  const name = snapshot.active_service || service.name || standardService;
  if (name === standardService) return standardService;
  for (const [id, preset] of servicePresets) {
    if ((preset.name === name || preset.legacyName === name) && preset.url === service.base_url && preset.model === service.model) return id;
  }
  return "custom";
}

function renderService() {
  const service = snapshot.service || {};
  const name = snapshot.active_service || service.name || standardService;
  const active = activeProvider();
  const activeLabel = active === standardService ? standardLabel : servicePresets.get(active)?.name ?? name;
  setText(byId("workspace-active-service"), activeLabel);
  byId("active-service").title = activeLabel;
  setText(byId("settings-active-service"), activeLabel);
  settingsProvider.value = selectedProvider;
  settingsProvider.disabled = profilePending;

  const preset = servicePresets.get(selectedProvider);
  const usePreset = Boolean(preset);
  setText(profileHeading, preset?.name ?? "Custom service");
  for (const section of customProfileSections) section.hidden = usePreset;
  profileName.disabled = profileModel.disabled = profileUrl.disabled = usePreset;
  profileKey.required = usePreset;
  profileKey.disabled = profilePending;
  profileKey.placeholder = usePreset && service.has_key && active === selectedProvider
    ? "Enter a replacement key"
    : "";

  const profiles = snapshot.profiles || {};
  if (!(activeProfile && Object.hasOwn(profiles, activeProfile))) activeProfile = "";
  const entries = Object.keys(profiles).sort((first, second) => first.localeCompare(second, "zh-CN"))
    .map((name) => [name, name]);
  const placeholder = entries.length ? "New profile" : "No saved profiles";
  profileList.disabled = profilePending || usePreset;
  optionsFor(profileList, [["", placeholder], ...entries], activeProfile);
  profileDelete.disabled = profilePending || usePreset || !activeProfile;
  profileSave.disabled = profilePending || usePreset;
  profileTest.disabled = profileApply.disabled = profilePending;
  inlineSelects.get(settingsProvider).sync();
}

async function api(path, { method = "GET", json, body, headers = {} } = {}) {
  if (method !== "GET") headers["X-Hermeneuein"] = "1";
  if (json !== undefined) {
    headers["Content-Type"] = "application/json";
    body = JSON.stringify(json);
  }
  const response = await fetch(path, { method, headers, body, cache: "no-store" });
  const payload = await response.json().catch(() => null);
  if (!response.ok) throw new Error(payload?.error || `Request failed (${response.status})`);
  return payload;
}

function scheduleRefresh() {
  clearTimeout(refreshTimer);
  if (!isWorking() && !isImporting()) return;
  refreshTimer = setTimeout(() => {
    refresh().catch((error) => {
      workspaceNotice = error.message;
      workspaceError = true;
      renderWorkspace();
      scheduleRefresh();
    });
  }, 350);
}

async function refresh({ initialize = false } = {}) {
  clearTimeout(refreshTimer);
  const sequence = ++readSequence;
  const data = await api("/api/state");
  if (sequence < appliedSequence) return;
  appliedSequence = sequence;
  const oldStatuses = new Map(snapshot.documents.map((doc) => [doc.id, doc.status]));
  snapshot = { ...data, documents: data.documents || [], profiles: data.profiles || {} };
  for (const doc of snapshot.documents) {
    if (doc.status === "done" && oldStatuses.get(doc.id) !== "done") {
      const id = String(doc.id);
      versionChoices.delete(id);
      resultRevisions.set(id, (resultRevisions.get(id) || 0) + 1);
    }
  }
  if (initialize) {
    restoreSettings(data.settings || {});
    const service = snapshot.service;
    selectedProvider = activeProvider();
    if (selectedProvider === "custom" && service?.name) {
      activeProfile = service.name;
      profileName.value = service.name;
      profileUrl.value = service.base_url || "";
      profileModel.value = service.model || "";
    }
  }
  apiReady = true;
  renderService();
  renderWorkspace();
  scheduleRefresh();
}

async function performWorkspace(label, activity) {
  if (stagedAction) return;
  stagedAction = label;
  workspaceNotice = "";
  workspaceError = false;
  renderWorkspace();
  try {
    await activity();
  } catch (error) {
    workspaceNotice = error.message;
    workspaceError = workspaceNotice !== canceledLabel;
  } finally {
    stagedAction = "";
    try {
      await refresh();
    } catch (error) {
      workspaceNotice ||= error.message;
      workspaceError = true;
      renderWorkspace();
      scheduleRefresh();
    }
  }
}

function switchSource(value) {
  if (source !== value) {
    source = value;
    workspaceNotice = "";
    workspaceError = false;
  }
  for (const radio of sourceToggle.querySelectorAll("input")) radio.checked = radio.value === value;
  renderWorkspace();
}

function addFiles(files) {
  if (!files.length || stagedAction || isWorking() || isImporting() || !apiReady) return;
  switchSource("file");
  performWorkspace(uploadLabel, async () => {
    const errors = [];
    for (const file of files) {
      try {
        await api("/api/documents/upload", {
          method: "POST",
          body: file,
          headers: { "X-Filename": encodeURIComponent(file.name), "Content-Type": "application/pdf" },
        });
      } catch (error) {
        errors.push(`${file.name}: ${error.message}`);
      }
    }
    if (errors.length) throw new Error(errors.join(" · "));
  });
}

function jobOptions() {
  const { pages, ...rest } = options;
  if (rest.from !== "auto" && rest.from === rest.to) throw new Error("Choose different source and target languages.");
  const minLength = Number(rest.min_text_length);
  if (!Number.isSafeInteger(minLength) || minLength < 0) throw new Error("Minimum text length must be a whole number of 0 or more.");
  return { ...rest, pages: pages.trim(), min_text_length: minLength,
    font_family: rest.font_family.trim() || null };
}

function setSetting(key, value) {
  options[key] = value;
  for (const control of document.querySelectorAll(`[data-setting="${key}"]`)) {
    if (control.type === "checkbox") control.checked = Boolean(value);
    else if (control.value !== String(value)) control.value = value;
    inlineSelects.get(control)?.sync();
  }
  if (key === "from") {
    languageSwap.disabled = value === "auto";
    languageSwap.title = languageSwap.disabled ? "Choose a source language to swap" : "Swap languages";
  }
}

function syncLanguageSelects() {
  const from = byId("workspace-language-from");
  const to = byId("workspace-language-to");
  const localized = (entries, selected) => entries.some(([key]) => key === selected) ? entries : [[selected, selected], ...entries];
  optionsFor(from, localized(languageOptions, options.from), options.from);
  optionsFor(to, localized(languageOptions.filter(([code]) => code !== "auto"), options.to), options.to);
}

function restoreSettings(data) {
  for (const key of ["from", "to", "translated_only", "translated_first", "alternate", "prompt", "json_mode",
    "glossary", "min_text_length", "font_family"]) {
    setSetting(key, data[key] ?? options[key]);
  }
  setSetting("format", knownFormats.includes(data.format) ? data.format : options.format);
  setSetting("pages", typeof data.pages === "string" ? data.pages : "");
  syncLanguageSelects();
}

function configureTabs(list, onChoose = () => {}) {
  const tabs = [...list.querySelectorAll("[role=tab]")];
  function choose(tab) {
    const previous = tabs.find((candidate) => candidate.getAttribute("aria-selected") === "true");
    const active = document.activeElement;
    const focusLeaves = tabs.some((candidate) => candidate !== tab &&
      (candidate === active || byId(candidate.getAttribute("aria-controls")).contains(active)));
    if (focusLeaves) tab.focus({ preventScroll: true });
    for (const candidate of tabs) {
      const selected = candidate === tab;
      candidate.setAttribute("aria-selected", String(selected));
      candidate.tabIndex = selected ? 0 : -1;
      byId(candidate.getAttribute("aria-controls")).hidden = !selected;
    }
    onChoose(tab, previous === tab ? null : previous);
  }
  for (const tab of tabs) tab.addEventListener("click", () => choose(tab));
  list.addEventListener("keydown", (event) => {
    const arrows = list.getAttribute("aria-orientation") === "vertical" ? ["ArrowUp", "ArrowDown"] : ["ArrowLeft", "ArrowRight"];
    const offset = arrows.indexOf(event.key);
    if (offset === -1 && !["Home", "End"].includes(event.key)) return;
    const current = tabs.indexOf(event.target);
    if (current < 0) return;
    const index = event.key === "Home" ? 0 : event.key === "End" ? tabs.length - 1
      : (current + (offset === 0 ? -1 : 1) + tabs.length) % tabs.length;
    event.preventDefault();
    choose(tabs[index]);
    tabs[index].focus();
  });
  return choose;
}

const app = document.querySelector(".app");
const reducedMotion = matchMedia("(prefers-reduced-motion: reduce)");
let enteringPanel;
let exitingPanel;
let settingsCategoryAnimation;

function finishTopPanelTransition() {
  enteringPanel?.classList.remove("top-panel-entering");
  if (exitingPanel) {
    exitingPanel.classList.remove("top-panel-exiting");
    exitingPanel.inert = false;
  }
  enteringPanel = exitingPanel = null;
  delete app.dataset.panelDirection;
}

function finishSettingsCategoryTransition() {
  settingsCategoryAnimation?.cancel();
  settingsCategoryAnimation = null;
}

app.addEventListener("animationend", (event) => {
  if (event.target === enteringPanel && event.animationName === "top-panel-enter") finishTopPanelTransition();
});
reducedMotion.addEventListener("change", (event) => {
  if (!event.matches) return;
  finishTopPanelTransition();
  finishSettingsCategoryTransition();
});
const chooseTop = configureTabs(document.querySelector(".top-tabs"), (tab, previous) => {
  if (!previous) return;
  closeInlineSelects();
  finishTopPanelTransition();
  finishSettingsCategoryTransition();
  if (reducedMotion.matches) return;

  enteringPanel = byId(tab.getAttribute("aria-controls"));
  exitingPanel = byId(previous.getAttribute("aria-controls"));
  app.dataset.panelDirection = tab.id === "settings-tab" ? "forward" : "back";
  exitingPanel.inert = true;
  exitingPanel.classList.add("top-panel-exiting");
  enteringPanel.classList.add("top-panel-entering");
});
const settingsPanel = byId("yepage-settings");
const settingsCategories = document.querySelector(".settings-categories");
const settingsContent = document.querySelector(".settings-content");
const chooseSettings = configureTabs(settingsCategories, (tab, previous) => {
  settingsContent.scrollTop = 0;
  if (!previous) return;
  closeInlineSelects();
  finishSettingsCategoryTransition();
  if (settingsPanel.hidden || enteringPanel === settingsPanel || reducedMotion.matches) return;

  const movesForward = previous.compareDocumentPosition(tab) & Node.DOCUMENT_POSITION_FOLLOWING;
  const offset = movesForward ? 7 : -7;
  const axis = settingsCategories.getAttribute("aria-orientation") === "horizontal" ? "X" : "Y";
  const category = byId(tab.getAttribute("aria-controls"));
  settingsCategoryAnimation = category.animate([
    { opacity: .76, transform: `translate${axis}(${offset}px)` },
    { opacity: 1, transform: "none" },
  ], { duration: 180, easing: "ease-out" });
});
const compactLayout = matchMedia("(width < 760px)");
const workspaceTabs = byId("workspace-view-tabs");
const workspaceConfigTab = byId("workspace-config-tab");
const workspacePreviewTab = byId("workspace-preview-tab");
const chooseWorkspaceView = configureTabs(workspaceTabs, (tab) => {
  closeInlineSelects(byId("yepage-main"));
  if (tab !== workspacePreviewTab) setDownloadMenu(false);
});

function syncCompactLayout() {
  const compact = compactLayout.matches;
  settingsCategories.setAttribute("aria-orientation", compact ? "horizontal" : "vertical");
  if (compact) workspaceTabs.hidden = false;
  else if (workspaceTabs.contains(document.activeElement)) byId("workspace-tab").focus({ preventScroll: true });

  for (const tab of [workspaceConfigTab, workspacePreviewTab]) {
    const panel = byId(tab.getAttribute("aria-controls"));
    if (compact) {
      panel.setAttribute("role", "tabpanel");
      panel.setAttribute("aria-labelledby", tab.id);
    } else {
      panel.removeAttribute("role");
      panel.removeAttribute("aria-labelledby");
      panel.hidden = false;
    }
  }
  if (compact) chooseWorkspaceView(workspaceTabs.querySelector('[aria-selected="true"]') || workspaceConfigTab);
  else workspaceTabs.hidden = true;
}
compactLayout.addEventListener("change", syncCompactLayout);
syncCompactLayout();
byId("active-service").addEventListener("click", () => {
  chooseTop(byId("settings-tab"));
  chooseSettings(byId("settings-services-tab"));
  inlineSelects.get(settingsProvider).focus();
});

function profileFormValues() {
  const preset = servicePresets.get(selectedProvider);
  const key = profileKey.value.trim();
  if (preset) {
    const { name, url, model } = preset;
    return { name, url, model, key };
  }
  return {
    name: profileName.value.trim(),
    url: profileUrl.value.trim(),
    model: profileModel.value.trim(),
    key,
  };
}

async function manageProfile(planned, action, feedback = profileStatus) {
  if (profilePending) return;
  profilePending = true;
  status(feedback, planned);
  renderService();
  try {
    const message = await action();
    status(feedback, message);
  } catch (error) {
    status(feedback, error.message, true);
  } finally {
    profilePending = false;
    renderService();
  }
}

profileList.addEventListener("change", () => {
  activeProfile = profileList.value;
  const profile = snapshot.profiles[activeProfile];
  profileName.value = activeProfile;
  profileModel.value = profile?.model ?? "";
  profileUrl.value = profile?.base_url ?? "";
  profileKey.value = "";
  setSecretVisibility(false);
  status(profileStatus, "");
  renderService();
});

function setSecretVisibility(visible) {
  profileKey.type = visible ? "text" : "password";
  const toggle = byId("profile-key-toggle");
  toggle.setAttribute("aria-label", visible ? "Hide API key" : "Show API key");
  toggle.setAttribute("aria-pressed", String(visible));
}

byId("profile-key-toggle").addEventListener("click", () => setSecretVisibility(profileKey.type === "password"));
profileSave.addEventListener("click", () => manageProfile("Saving…", async () => {
  const { name, url, model } = profileFormValues();
  await api("/api/profiles", { method: "POST", json: { name, url, model } });
  activeProfile = name;
  await refresh();
  return "Profile saved without the API key.";
}));
profileDelete.addEventListener("click", () => manageProfile("Deleting…", async () => {
  if (!activeProfile) return "";
  await api(`/api/profiles/${encodeURIComponent(activeProfile)}`, { method: "DELETE" });
  activeProfile = "";
  profileName.value = profileUrl.value = profileModel.value = profileKey.value = "";
  setSecretVisibility(false);
  await refresh();
  return "Profile deleted";
}));
profileTest.addEventListener("click", () => {
  if (profileKey.required && !profileKey.reportValidity()) return;
  manageProfile("Testing connection…", async () => {
    const { url, model, key } = profileFormValues();
    const response = await api("/api/profiles/test", { method: "POST", json: { url, model, key } });
    return typeof response === "string" ? response : response?.message || "Connection successful";
  });
});
byId("profile-form").addEventListener("submit", (event) => {
  event.preventDefault();
  manageProfile("Applying…", async () => {
    const preset = servicePresets.has(selectedProvider);
    await api("/api/service", { method: "POST", json: profileFormValues() });
    if (!preset) selectedProvider = "custom";
    setSecretVisibility(false);
    await refresh();
    return preset ? "Service applied. Save settings to keep it after restart." : "Service applied";
  });
});
settingsProvider.addEventListener("change", () => {
  const previousProvider = selectedProvider;
  selectedProvider = settingsProvider.value;
  profileKey.value = "";
  setSecretVisibility(false);
  status(profileStatus, "");
  if (selectedProvider !== standardService) {
    renderService();
    (servicePresets.has(selectedProvider) ? profileKey : profileName).focus();
    return;
  }
  manageProfile("Applying…", async () => {
    try {
      await api("/api/service", { method: "POST", json: { name: standardService } });
    } catch (error) {
      selectedProvider = previousProvider;
      throw error;
    }
    await refresh();
    return "Service applied";
  }, settingsStatus);
});

for (const control of document.querySelectorAll("[data-setting]")) {
  const key = control.dataset.setting;
  control.addEventListener(control.type === "checkbox" || control.tagName === "SELECT" ? "change" : "input", () => {
    setSetting(key, control.type === "checkbox" ? control.checked : control.value);
  });
}
languageSwap.addEventListener("click", () => {
  if (options.from === "auto") return;
  const from = options.from;
  const to = options.to;
  setSetting("from", to);
  setSetting("to", from);
  syncLanguageSelects();
});

byId("settings-save").addEventListener("click", async (event) => {
  if (settingsPending) return;
  const button = event.currentTarget;
  settingsPending = button.disabled = true;
  status(settingsStatus, "Saving…");
  try {
    await api("/api/settings", { method: "POST", json: jobOptions() });
    status(settingsStatus, "Settings saved");
  } catch (error) {
    status(settingsStatus, error.message, true);
  } finally {
    settingsPending = button.disabled = false;
  }
});

sourceToggle.addEventListener("change", (event) => switchSource(event.target.value));
byId("pick-files").addEventListener("click", () => fileInput.click());
byId("workspace-preview-empty").addEventListener("click", () => {
  if (source === "link") {
    if (compactLayout.matches) chooseWorkspaceView(workspaceConfigTab);
    linkInput.focus();
  } else {
    fileInput.click();
  }
});
fileInput.addEventListener("change", () => {
  const files = [...fileInput.files];
  fileInput.value = "";
  addFiles(files);
});
linkForm.addEventListener("submit", (event) => {
  event.preventDefault();
  const url = linkInput.value.trim();
  if (linkImport.disabled) return;
  performWorkspace(importLabel, async () => {
    const data = await api("/api/documents/link", { method: "POST", json: { url } });
    const imported = data?.document || data;
    if (imported?.id !== undefined) chosen.link = String(imported.id);
    linkInput.value = "";
  });
});
linkInput.addEventListener("input", updateLinkImport);
docSelect.addEventListener("change", () => selectDocument(docSelect.value));
versions.addEventListener("change", (event) => {
  const doc = selectedDocument();
  if (!doc) return;
  versionChoices.set(String(doc.id), event.target.value);
  renderWorkspace();
});
byId("workspace-translate").addEventListener("click", () => performWorkspace("Starting translation…", async () => {
  const document_ids = selectedScope().map((doc) => doc.id);
  await api("/api/jobs", { method: "POST", json: { document_ids, options: jobOptions() } });
}));
byId("workspace-cancel").addEventListener("click", async () => {
  const cancel = () => api("/api/jobs/cancel", { method: "POST", json: {} });
  if (!isImporting()) return performWorkspace(cancelingLabel, cancel);
  cancelingImport = true;
  renderWorkspace();
  try {
    await cancel();
  } catch (error) {
    workspaceNotice = error.message;
    workspaceError = true;
  } finally {
    cancelingImport = false;
    renderWorkspace();
  }
});
byId("workspace-retry").addEventListener("click", () => performWorkspace("Retrying…", () =>
  api("/api/jobs/retry", { method: "POST", json: {} })
));
downloadButton.addEventListener("click", () => setDownloadMenu(downloads.hidden));
document.addEventListener("pointerdown", (event) => {
  if (!downloadControl.contains(event.target)) setDownloadMenu(false);
});
document.addEventListener("keydown", (event) => {
  if (!event.defaultPrevented && event.key === "Escape" && !downloads.hidden) setDownloadMenu(false, true);
});

const fileDrag = (event) => event.dataTransfer?.types.includes("Files");
const dropZones = [upload, previewPane];
for (const zone of dropZones) {
  zone.addEventListener("dragover", (event) => {
    if (!fileDrag(event)) return;
    event.preventDefault();
    if (!isWorking() && !isImporting() && !stagedAction && apiReady) {
      event.dataTransfer.dropEffect = "copy";
      zone.classList.add("drop-active");
    } else {
      event.dataTransfer.dropEffect = "none";
    }
  });
  zone.addEventListener("dragleave", (event) => {
    if (!zone.contains(event.relatedTarget)) zone.classList.remove("drop-active");
  });
  zone.addEventListener("drop", (event) => {
    if (!fileDrag(event)) return;
    event.preventDefault();
    zone.classList.remove("drop-active");
    addFiles([...event.dataTransfer.files]);
  });
}
for (const type of ["dragover", "drop"]) {
  document.addEventListener(type, (event) => {
    if (fileDrag(event)) event.preventDefault();
  });
}

setPreviewSketch();
restoreSettings({});
renderWorkspace();
refresh({ initialize: true }).catch((error) => {
  workspaceNotice = error.message;
  workspaceError = true;
  renderWorkspace();
});
