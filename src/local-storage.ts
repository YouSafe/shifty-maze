export function useLocalStorage<T>() {
  const version = 1;
  const key = "game-state";

  function replacer(key: string, value: any): any {
    if (value instanceof Map) {
      return {
        dataType: "Map",
        value: [...value],
      };
    }
    return value;
  }

  function reviver(key: string, value: any): any {
    if (value instanceof Object && value.dataType === "Map") {
      return new Map(value.value);
    }
    return value;
  }

  const newGame = () => {
    localStorage.removeItem(key);
  };

  const save = (state: T) => {
    localStorage.setItem(key, JSON.stringify({ version, state }, replacer));
  };

  const load = () => {
    const entry = localStorage.getItem(key);
    if (entry !== null) {
      try {
        const parsed = JSON.parse(entry, reviver) as {
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
