const RESERVED_COMBINATIONS = Object.freeze([
  'ctrl+power',
  'cmd+ctrl+power',
  'alt+cmd+ctrl+power',
  'cmd+ctrl+q',
]);

export function isReservedShortcut(shortcut) {
  return RESERVED_COMBINATIONS.includes([...shortcut.modifiers, shortcut.keyEquivalent].join('+'));
}
