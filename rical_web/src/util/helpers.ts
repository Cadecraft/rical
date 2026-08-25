export function isAnythingFocused() {
  return document.activeElement &&
    document.activeElement !== document.body &&
    document.activeElement.nodeName !== "BUTTON";
}
