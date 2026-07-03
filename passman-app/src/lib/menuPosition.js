import { CONTEXT_MENU_WIDTH, CONTEXT_MENU_PADDING } from "./constants.js";

export function computeSubmenuLeft(baseLeft, width) {
  const w = width || CONTEXT_MENU_WIDTH;
  let nextLeft = baseLeft + w;
  if (nextLeft + w > window.innerWidth - CONTEXT_MENU_PADDING) {
    nextLeft = baseLeft - w;
  }
  if (nextLeft < CONTEXT_MENU_PADDING) nextLeft = CONTEXT_MENU_PADDING;
  return nextLeft;
}
