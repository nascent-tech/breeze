import { InvalidValue } from '../errors/invalid-value.error.js';

export function refuseUnknownShape(literal, knownKeys = [], refusal = '') {
  if (literal === null || typeof literal !== 'object') {
    throw new InvalidValue(refusal);
  }

  if (Object.keys(literal).some((key) => !knownKeys.includes(key))) {
    throw new InvalidValue(refusal);
  }
}
