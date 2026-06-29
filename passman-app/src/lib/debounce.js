/**
 * Return a debounced wrapper around `fn` that waits `delayMs` after the last
 * call before invoking the function.
 */
export function debounce(fn, delayMs) {
  let timer = null;
  return (...args) => {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      fn(...args);
    }, delayMs);
  };
}
