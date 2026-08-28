import { InvalidValue } from '../errors/invalid-value.error.js';
import { refuseUnknownShape } from './literal-shape.js';

const REFUSAL = 'a shortcut is built from an identity, a key equivalent and modifiers, and nothing else';
const KNOWN_KEYS = ['identity', 'keyEquivalent', 'modifiers'];
const KNOWN_MODIFIERS = ['alt', 'cmd', 'ctrl', 'shift'];
const MAXIMUM_IDENTITY_LENGTH = 128;

function requiredIdentity(value = '') {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new InvalidValue('a shortcut requires an identity');
  }

  if (value.length > MAXIMUM_IDENTITY_LENGTH) {
    throw new InvalidValue(`a shortcut refuses an identity beyond ${MAXIMUM_IDENTITY_LENGTH} characters`);
  }

  return value;
}

function requiredKeyEquivalent(value = '') {
  if (typeof value !== 'string' || value.trim() === '') {
    throw new InvalidValue('a shortcut requires a key equivalent');
  }

  return value;
}

function normalisedModifiers(value) {
  if (!Array.isArray(value) || value.length === 0) {
    throw new InvalidValue('a global shortcut without a modifier takes a key away from the whole system');
  }

  const unknown = value.find((modifier) => !KNOWN_MODIFIERS.includes(modifier));

  if (unknown !== undefined || new Set(value).size !== value.length) {
    throw new InvalidValue('a shortcut carries known modifiers, each of them once');
  }

  return Object.freeze([...value].sort());
}

export class Shortcut {
  #identity;
  #keyEquivalent;
  #modifiers;

  constructor(literal) {
    refuseUnknownShape(literal, KNOWN_KEYS, REFUSAL);

    this.#identity = requiredIdentity(literal.identity);
    this.#keyEquivalent = requiredKeyEquivalent(literal.keyEquivalent);
    this.#modifiers = normalisedModifiers(literal.modifiers);

    Object.freeze(this);
  }

  static of(literal) {
    return new Shortcut(literal);
  }

  get identity() {
    return this.#identity;
  }

  get keyEquivalent() {
    return this.#keyEquivalent;
  }

  get modifiers() {
    return [...this.#modifiers];
  }

  equals(other) {
    return other instanceof Shortcut && other.identity === this.#identity;
  }
}
