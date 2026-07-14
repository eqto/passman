import { writable } from "svelte/store";

type Axis = "vertical" | "horizontal";

type Zone = "before" | "after" | "into";

interface DragListOptions<T> {
  axis?: Axis;
  getKey?: (item: T) => string | number;
  onReorder?: (items: T[], info: { source: T; target: T; zone: Zone }) => void;
  onDropInto?: (info: { source: T; target: T }) => void;
}

interface DropTarget<T> {
  type: Zone;
  item: T;
}

interface DragList<T> {
  dragItem: ReturnType<typeof writable<T | null>>;
  dropTarget: ReturnType<typeof writable<DropTarget<T> | null>>;
  dragStart: (event: DragEvent, item: T) => void;
  dragEnd: () => void;
  handleDragOver: (event: DragEvent, item: T) => void;
  dragLeave: () => void;
  handleDragOverFirst: (event: DragEvent, item: T) => void;
  dropFirst: (event: DragEvent, items: T[], target: T) => T[] | null;
  drop: (event: DragEvent, items: T[], target: T) => T[] | null;
}

export function createDragList<T>({
  axis = "vertical",
  getKey = (x) => x as unknown as string | number,
  onReorder,
  onDropInto,
}: DragListOptions<T> = {}): DragList<T> {
  let _dragItem: T | null = null;
  const dragItem = writable<T | null>(null);
  const dropTarget = writable<DropTarget<T> | null>(null);

  const centerThreshold = 0.25;

  function getZone(event: DragEvent, rect: DOMRect): Zone {
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

    dragStart(event: DragEvent, item: T) {
      _dragItem = item;
      dragItem.set(item);
      event.dataTransfer!.effectAllowed = "move";
      event.dataTransfer!.setData("text/plain", String(getKey(item)));
    },

    dragEnd() {
      _dragItem = null;
      dragItem.set(null);
      dropTarget.set(null);
    },

    handleDragOver(event: DragEvent, item: T) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dropTarget.set(null);
        return;
      }
      const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
      const zone = getZone(event, rect);
      dropTarget.set({ type: zone, item });
    },

    dragLeave() {
      dropTarget.set(null);
    },

    handleDragOverFirst(event: DragEvent, item: T) {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "move";
      if (!_dragItem || getKey(_dragItem) === getKey(item)) {
        dropTarget.set(null);
        return;
      }
      dropTarget.set({ type: "before", item });
    },

    dropFirst(event: DragEvent, items: T[], target: T): T[] | null {
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
      onReorder?.(current, { source: moved, target, zone: "before" });
      return current;
    },

    drop(event: DragEvent, items: T[], target: T): T[] | null {
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

      const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
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
      onReorder?.(current, { source: moved, target, zone });
      return current;
    },
  };
}
