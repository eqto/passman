import { writable } from "svelte/store";

export function createDragList({ axis = "vertical", getKey = (x) => x, onReorder, onDropInto }) {
  let _dragItem = null;
  const dragItem = writable(null);
  const dropTarget = writable(null);

  // Center 50% of an item triggers drop-into, top/bottom 25% triggers reorder
  const centerThreshold = 0.25;

  function getZone(event, rect) {
    const pos = axis === "horizontal" ? event.clientX : event.clientY;
    const size = axis === "horizontal" ? rect.width : rect.height;
    const start = axis === "horizontal" ? rect.left : rect.top;
    let relativePos = (pos - start) / size;
    relativePos = Math.max(0, Math.min(1, relativePos));

    if (relativePos > centerThreshold && relativePos < 1 - centerThreshold) {
      return "into";
    }
    return relativePos < 0.5 ? "before" : "after";
  }

  return {
    dragItem,
    dropTarget,

    dragStart(event, item) {
      _dragItem = item;
      dragItem.set(item);
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(getKey(item)));
    },

    dragEnd() {
      _dragItem = null;
      dragItem.set(null);
      dropTarget.set(null);
    },

    handleDragOver(event, item) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dropTarget.set(null);
        return;
      }
      const rect = event.currentTarget.getBoundingClientRect();
      const zone = getZone(event, rect);
      dropTarget.set({ type: zone, item });
    },

    dragLeave() {
      dropTarget.set(null);
    },

    handleDragOverFirst(event, item) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dropTarget.set(null);
        return;
      }
      dropTarget.set({ type: "before", item });
    },

    dropFirst(event, items, target) {
      event.preventDefault();
      dropTarget.set(null);
      if (!_dragItem || getKey(_dragItem) === getKey(target)) {
        _dragItem = null;
        dragItem.set(null);
        return null;
      }
      const current = [...items];
      const fromIndex = current.findIndex((i) => getKey(i) === getKey(_dragItem));
      const toIndex = current.findIndex((i) => getKey(i) === getKey(target));
      if (fromIndex === -1 || toIndex === -1) {
        _dragItem = null;
        dragItem.set(null);
        return null;
      }
      let newIndex = toIndex;
      if (fromIndex < toIndex) newIndex = toIndex - 1;
      const [moved] = current.splice(fromIndex, 1);
      current.splice(newIndex, 0, moved);
      _dragItem = null;
      dragItem.set(null);
      onReorder(current, { source: moved, target, zone: "before" });
      return current;
    },

    drop(event, items, target) {
      event.preventDefault();
      dropTarget.set(null);
      if (!_dragItem || getKey(_dragItem) === getKey(target)) {
        _dragItem = null;
        dragItem.set(null);
        return null;
      }

      const current = [...items];
      const fromIndex = current.findIndex((i) => getKey(i) === getKey(_dragItem));
      const toIndex = current.findIndex((i) => getKey(i) === getKey(target));
      if (fromIndex === -1 || toIndex === -1) {
        _dragItem = null;
        dragItem.set(null);
        return null;
      }

      const rect = event.currentTarget.getBoundingClientRect();
      const zone = getZone(event, rect);

      if (zone === "into" && onDropInto) {
        onDropInto({ source: _dragItem, target });
        _dragItem = null;
        dragItem.set(null);
        return null;
      }

      const before = zone === "before";
      let newIndex = toIndex;
      if (before) {
        if (fromIndex < toIndex) newIndex = toIndex - 1;
      } else {
        if (fromIndex > toIndex) newIndex = toIndex + 1;
      }
      const [moved] = current.splice(fromIndex, 1);
      current.splice(newIndex, 0, moved);
      _dragItem = null;
      dragItem.set(null);
      onReorder(current, { source: moved, target, zone });
      return current;
    },
  };
}
