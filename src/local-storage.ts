import { JsonSerializer } from "json-safe-stringify";

export function useLocalStorage<T>() {
  const version = 1;
  const key = "game-state";
  const serializer = new JsonSerializer();

  const newGame = () => {
    localStorage.removeItem(key);
  };

  const save = (state: T) => {
    localStorage.setItem(key, serializer.stringify({ version, state }));
  };

  const load = () => {
    const entry = localStorage.getItem(key);
    if (entry !== null) {
      try {
        const parsed = serializer.parse(entry) as {
          version: number;
          state: T;
        };
        if (parsed.version === version) {
          return parsed.state;
        }
      } catch (e) {}
    }
    return null;
  };

  return {
    newGame,
    save,
    load,
  };
}
