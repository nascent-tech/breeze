export class CycleError extends Error {
  constructor(message = '') {
    super(message);

    this.name = new.target.name;
  }
}
