export function isAnythingFocused() {
  return document.activeElement &&
    document.activeElement !== document.body &&
    document.activeElement.nodeName !== "BUTTON";
}

export function isAnythingFocusedInclButton() {
  return document.activeElement &&
    document.activeElement !== document.body
}
