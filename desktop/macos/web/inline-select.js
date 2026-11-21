export function createInlineSelect(select, closeOtherSelects) {
  const wrapper = select.parentElement;
  const label = select.getAttribute("aria-labelledby");
  const trigger = document.createElement("button");
  const value = document.createElement("span");
  const arrow = document.createElement("span");
  const list = document.createElement("div");
  let rows = new Map();
  let search = "";
  let lastKeystroke = 0;

  trigger.id = `${select.id}-trigger`;
  trigger.type = "button";
  trigger.className = "inline-select-trigger";
  trigger.setAttribute("aria-labelledby", `${label} ${select.id}-value`);
  trigger.setAttribute("aria-expanded", "false");
  trigger.setAttribute("aria-controls", `${select.id}-options`);
  value.id = `${select.id}-value`;
  arrow.className = "chevron down";
  arrow.setAttribute("aria-hidden", "true");
  trigger.append(value, arrow);
  list.id = `${select.id}-options`;
  list.className = "inline-select-options";
  list.setAttribute("role", "listbox");
  list.setAttribute("aria-labelledby", label);
  list.hidden = true;
  select.hidden = true;
  wrapper.classList.replace("select-wrap", "inline-select");
  wrapper.append(trigger, list);

  function close() {
    if (list.hidden) return;
    list.hidden = true;
    trigger.setAttribute("aria-expanded", "false");
    search = "";
  }

  function sync() {
    const selected = select.selectedOptions[0];
    const text = selected?.label || "";
    if (value.textContent !== text) value.textContent = text;
    trigger.title = text;
    trigger.disabled = select.disabled;
    if (trigger.disabled) close();
    if (list.hidden) return;
    if (rows.size !== select.options.length || [...select.options].some((option) => !rows.has(option))) {
      if (list.contains(document.activeElement)) trigger.focus({ preventScroll: true });
      close();
      return;
    }
    for (const [option, row] of rows) row.setAttribute("aria-selected", String(option.selected));
  }

  function makeRow(option) {
    const row = document.createElement("button");
    row.type = "button";
    row.tabIndex = -1;
    row.dataset.value = option.value;
    row.textContent = option.label;
    row.disabled = option.disabled || Boolean(option.closest("optgroup")?.disabled);
    row.setAttribute("role", "option");
    row.setAttribute("aria-selected", String(option.selected));
    row.addEventListener("click", () => {
      const changed = select.selectedIndex !== option.index;
      close();
      trigger.focus({ preventScroll: true });
      if (!changed) return;
      select.selectedIndex = option.index;
      sync();
      select.dispatchEvent(new Event("change", { bubbles: true }));
    });
    rows.set(option, row);
    return row;
  }

  function open() {
    if (trigger.disabled || !list.hidden) return;
    closeOtherSelects();
    rows = new Map();
    list.replaceChildren(...Array.from(select.children, (child) => {
      if (child.tagName !== "OPTGROUP") return makeRow(child);
      const group = document.createElement("div");
      const heading = document.createElement("span");
      group.setAttribute("role", "group");
      group.setAttribute("aria-label", child.label);
      heading.className = "inline-select-group-label";
      heading.textContent = child.label;
      heading.setAttribute("aria-hidden", "true");
      group.append(heading, ...Array.from(child.children, makeRow));
      return group;
    }));
    list.hidden = false;
    trigger.setAttribute("aria-expanded", "true");
    const active = rows.get(select.selectedOptions[0]);
    (active && !active.disabled ? active : list.querySelector("button:not(:disabled)"))?.focus();
  }

  trigger.addEventListener("click", () => {
    if (list.hidden) open();
    else {
      close();
      trigger.focus({ preventScroll: true });
    }
  });
  wrapper.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !list.hidden) {
      event.preventDefault();
      close();
      trigger.focus({ preventScroll: true });
      return;
    }
    if (event.target === trigger && ["ArrowDown", "ArrowUp"].includes(event.key)) {
      event.preventDefault();
      if (list.hidden) open();
      else (rows.get(select.selectedOptions[0]) || list.querySelector("button:not(:disabled)"))?.focus();
      return;
    }
    if (!list.contains(event.target)) return;
    const enabled = [...list.querySelectorAll("button:not(:disabled)")];
    const index = enabled.indexOf(event.target);
    const end = enabled.length - 1;
    const destinations = {
      ArrowUp: Math.max(0, index - 1),
      ArrowDown: Math.min(end, index + 1),
      Home: 0,
      End: end,
    };
    if (Object.hasOwn(destinations, event.key)) {
      event.preventDefault();
      enabled[destinations[event.key]]?.focus();
      return;
    }
    if (event.key === "Tab") {
      close();
      trigger.focus({ preventScroll: true });
      return;
    }
    if (event.key.length !== 1 || event.key === " " || event.ctrlKey || event.metaKey || event.altKey) return;
    const now = performance.now();
    const letter = event.key.toLocaleLowerCase();
    search = now - lastKeystroke < 550 ? search + letter : letter;
    lastKeystroke = now;
    const afterCurrent = [...enabled.slice(index + 1), ...enabled.slice(0, index + 1)];
    const match = afterCurrent.find((row) => row.textContent.toLocaleLowerCase().startsWith(search));
    if (match) {
      event.preventDefault();
      match.focus();
    }
  });

  sync();
  return { close, sync, focus: () => trigger.focus() };
}
