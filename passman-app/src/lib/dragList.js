import { writable } from "svelte/store";

export function createDragList({ axis = "vertical", getKey = (x) => x, onReorder, onDropInto }) {
  let _dragItem = null;
  const dragItem = writable(null);
  const dragOver = writable(null);
  const insertBefore = writable(null);
  const dropInto = writable(null);

  function getPos(event, rect) {
    return axis === "horizontal"
      ? event.clientX < rect.left + rect.width / 2
      : event.clientY < rect.top + rect.height / 2;
  }

  return {
    dragItem,
    dragOver,
    insertBefore,
    dropInto,

    dragStart(event, item) {
      _dragItem = item;
      dragItem.set(item);
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(getKey(item)));
    },

    dragEnd() {
      _dragItem = null;
      dragItem.set(null);
      dragOver.set(null);
      insertBefore.set(null);
      dropInto.set(null);
    },

    handleDragOver(event, item) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dragOver.set(null);
        insertBefore.set(null);
        dropInto.set(null);
        return;
      }
      const rect = event.currentTarget.getBoundingClientRect();
      const pos = axis === "horizontal" ? event.clientX : event.clientY;
      const size = axis === "horizontal" ? rect.width : rect.height;
      const start = axis === "horizontal" ? rect.left : rect.top;

      // Clamp relative position to [0, 1] to handle edge cases
      let relativePos = (pos - start) / size;
      relativePos = Math.max(0, Math.min(1, relativePos));

      // Check if cursor is in center 40% zone (top/bottom 30% are reorder zones)
      const isCenterZone = relativePos > 0.3 && relativePos < 0.7;

      if (isCenterZone) {
        dropInto.set(item);
        insertBefore.set(null);
      } else {
        insertBefore.set(getPos(event, rect));
        dropInto.set(null);
      }
      dragOver.set(item);
    },

    dragLeave() {
      dragOver.set(null);
      insertBefore.set(null);
      dropInto.set(null);
    },

    drop(event, items, target) {
      event.preventDefault();
      dragOver.set(null);
      insertBefore.set(null);
      dropInto.set(null);
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
      const pos = axis === "horizontal" ? event.clientX : event.clientY;
      const size = axis === "horizontal" ? rect.width : rect.height;
      const start = axis === "horizontal" ? rect.left : rect.top;
      const relativePos = (pos - start) / size;
      const isCenterZone = relativePos > 0.3 && relativePos < 0.7;

      if (isCenterZone && onDropInto) {
        // Drop into as child
        onDropInto({ source: _dragItem, target });
        _dragItem = null;
        dragItem.set(null);
        return null;
      }

      // Reorder as before
      const before = getPos(event, rect);
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
      onReorder(current);
      return current;
    },
  };
}
