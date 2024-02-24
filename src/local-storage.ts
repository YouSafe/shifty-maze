import { JsonSerializer } from "json-safe-stringify";
import { None, Some, type Option } from "@/result";

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

  const load = (): Option<T> => {
    const entry = localStorage.getItem(key);
    if (entry !== null) {
      try {
        const parsed = serializer.parse(entry) as {
          version: number;
          state: T;
        };
        if (parsed.version === version) {
          return Some(parsed.state);
        }
      } catch (e) { }
    }
    return None();
  };

  return {
    newGame,
    save,
    load,
  };
}
