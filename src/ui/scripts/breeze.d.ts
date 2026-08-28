export {};

declare global {
  interface Window {
    breeze: {
      onState(listener: (state: Record<string, any>) => void): () => void;
      send(command: string, payload?: unknown): Promise<unknown>;
    };
  }
}
