import { writable } from "svelte/store";

export function createDragList({ axis = "vertical", getKey = (x) => x, onReorder }) {
  let _dragItem = null;
  const dragItem = writable(null);
  const dragOver = writable(null);
  const insertBefore = writable(null);

  function getPos(event, rect) {
    return axis === "horizontal"
      ? event.clientX < rect.left + rect.width / 2
      : event.clientY < rect.top + rect.height / 2;
  }

  return {
    dragItem,
    dragOver,
    insertBefore,

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
    },

    handleDragOver(event, item) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dragOver.set(null);
        insertBefore.set(null);
        return;
      }
      const rect = event.currentTarget.getBoundingClientRect();
      insertBefore.set(getPos(event, rect));
      dragOver.set(item);
    },

    dragLeave() {
      dragOver.set(null);
      insertBefore.set(null);
    },

    drop(event, items, target) {
      event.preventDefault();
      dragOver.set(null);
      insertBefore.set(null);
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
