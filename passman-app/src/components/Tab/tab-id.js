let counter = 0;

export function nextTabId() {
  counter += 1;
  return `tab-${counter}`;
}
