import {
  GROUP_PANEL_DEFAULT_WIDTH,
  GROUP_PANEL_MIN_WIDTH,
  ENTRY_PANEL_DEFAULT_WIDTH,
  ENTRY_PANEL_MIN_WIDTH,
  COLUMN_RESIZE_STEP,
} from "./constants.js";

const WIDTHS_KEY = "passman.columnWidths";

export function createColumnResize() {
  let columnWidths = {
    groups: GROUP_PANEL_DEFAULT_WIDTH,
    entries: ENTRY_PANEL_DEFAULT_WIDTH,
  };

  let resizing = null;
  let startX = 0;
  let startWidth = 0;

  function loadWidths() {
    try {
      const saved = JSON.parse(localStorage.getItem(WIDTHS_KEY));
      if (saved) {
        columnWidths.groups = Math.max(
          GROUP_PANEL_MIN_WIDTH,
          saved.groups || GROUP_PANEL_DEFAULT_WIDTH,
        );
        columnWidths.entries = Math.max(
          ENTRY_PANEL_MIN_WIDTH,
          saved.entries || ENTRY_PANEL_DEFAULT_WIDTH,
        );
      }
    } catch {
      // ignore invalid saved config
    }
  }

  function saveWidths() {
    localStorage.setItem(WIDTHS_KEY, JSON.stringify(columnWidths));
  }

  function getMinWidth(panel) {
    return panel === "groups" ? GROUP_PANEL_MIN_WIDTH : ENTRY_PANEL_MIN_WIDTH;
  }

  function startResize(panel, event) {
    resizing = panel;
    startX = event.clientX;
    startWidth = columnWidths[panel];
    window.addEventListener("mousemove", onResize);
    window.addEventListener("mouseup", stopResize);
  }

  function onResize(event) {
    if (!resizing) return;
    const delta = event.clientX - startX;
    columnWidths[resizing] = Math.max(getMinWidth(resizing), startWidth + delta);
  }

  function stopResize() {
    resizing = null;
    window.removeEventListener("mousemove", onResize);
    window.removeEventListener("mouseup", stopResize);
    saveWidths();
  }

  function handleKeyResize(panel, event) {
    if (event.key === "ArrowLeft") {
      columnWidths[panel] = Math.max(
        getMinWidth(panel),
        columnWidths[panel] - COLUMN_RESIZE_STEP,
      );
      saveWidths();
    } else if (event.key === "ArrowRight") {
      columnWidths[panel] += COLUMN_RESIZE_STEP;
      saveWidths();
    }
  }

  return {
    columnWidths,
    loadWidths,
    startResize,
    handleKeyResize,
  };
}
