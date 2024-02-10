export function useLocalStorage<T>() {
  const version = 1;
  const key = "game-state";

  const newGame = () => {
    localStorage.removeItem(key);
  };

  const save = (state: T) => {
    localStorage.setItem(key, JSON.stringify({ version, state }));
  };

  const load = () => {
    const entry = localStorage.getItem(key);
    if (!entry) return null;
    try {
      const parsed = JSON.parse(entry) as { version: number; state: T };
      if (parsed.version === version) {
        return parsed.state;
      } else {
        return null;
      }
    } catch (e) {
      return null;
    }
  };

  return {
    newGame,
    save,
    load,
  };
}
