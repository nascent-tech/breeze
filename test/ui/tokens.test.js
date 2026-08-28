import { test } from 'node:test';
import { deepStrictEqual, ok } from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const DESIGN = readFileSync('DESIGN.md', 'utf8');
const TOKENS = readFileSync('src/ui/styles/tokens.css', 'utf8');

function blockOf(name) {
  const found = new RegExp(`^${name}:\\n((?:  .*\\n|\\n)*)`, 'm').exec(DESIGN);

  return found === null ? '' : found[1];
}

function declaredIn(name) {
  return [...blockOf(name).matchAll(/^ {2}([a-z0-9-]+): "?([^"\n]+?)"?$/gm)].map(([, key, value]) => [key, value]);
}

function missingFrom(prefix, declared) {
  return declared.filter(([key, value]) => !TOKENS.includes(`--${prefix}-${key}: ${value};`)).map(([key]) => key);
}

test('every colour of the design system is carried by the token sheet, with its value', () => {
  deepStrictEqual(missingFrom('color', declaredIn('colors')), []);
});

test('the dark theme carries every colour the light one does', () => {
  deepStrictEqual(missingFrom('color', declaredIn('colors-dark')), []);
});

test('the radii, the spacing scale and the strokes come from the design system', () => {
  deepStrictEqual(missingFrom('radius', declaredIn('rounded')), []);
  deepStrictEqual(missingFrom('space', declaredIn('spacing')), []);
  deepStrictEqual(missingFrom('stroke', declaredIn('stroke')), []);
});

test('the seven typographic levels are the ones the design system names, and no other', () => {
  const declared = [...blockOf('typography').matchAll(/^ {2}([a-z0-9]+):$/gm)].map(([, level]) => level);
  const written = [...TOKENS.matchAll(/^\.type-([a-z0-9]+) \{$/gm)].map(([, level]) => level);

  deepStrictEqual(written, declared);
});

test('no typographic level carries a size the design system did not declare', () => {
  const sizes = [...blockOf('typography').matchAll(/fontSize: (\d+px)/g)].map(([, size]) => size);

  for (const size of sizes) {
    ok(TOKENS.includes(`font-size: ${size};`));
  }
});
