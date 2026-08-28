import { test } from 'node:test';
import { ok, strictEqual, throws } from 'node:assert/strict';

import { Bounds } from '../../../../../src/modules/system-bridge/application/model/bounds.value-object.js';
import { InvalidValue } from '../../../../../src/modules/system-bridge/application/errors/invalid-value.error.js';

const SQUARE = { x: 0, y: 0, width: 100, height: 100 };

test('bounds refuse a negative width or height', () => {
  throws(() => Bounds.of({ ...SQUARE, width: -1 }), InvalidValue);
  throws(() => Bounds.of({ ...SQUARE, height: -1 }), InvalidValue);
});

test('bounds refuse a coordinate that is not finite', () => {
  throws(() => Bounds.of({ ...SQUARE, x: Number.POSITIVE_INFINITY }), InvalidValue);
  throws(() => Bounds.of({ ...SQUARE, y: undefined }), InvalidValue);
});

test('bounds accept a negative origin, which a screen left of the main one has', () => {
  strictEqual(Bounds.of({ ...SQUARE, x: -1920 }).x, -1920);
});

test('bounds accept a null surface, which macOS exposes on a dying window', () => {
  strictEqual(Bounds.of({ ...SQUARE, width: 0, height: 0 }).width, 0);
});

test('bounds refuse a key they do not carry', () => {
  throws(() => Bounds.of({ ...SQUARE, scaleFactor: 2 }), InvalidValue);
});

test('two bounds of the same origin and size are the same bounds', () => {
  ok(Bounds.of(SQUARE).equals(Bounds.of(SQUARE)));
});

test('bounds built from nothing at all is refused like any other invalid value', () => {
  throws(() => Bounds.of(), InvalidValue);
});
