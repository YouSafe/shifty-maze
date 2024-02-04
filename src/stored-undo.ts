export function useStoredUndo<T>() {
  const version = 1;
  const key = "game-state";
  const maxEntries = 10;

  const add = (state: T) => {
    const history = get();
    history.push(state);
    while (history.length > maxEntries) {
      history.shift();
    }
    save(history);
  };

  const save = (history: T[]) => {
    localStorage.setItem(key, JSON.stringify({ version, data: history }));
  };

  const get = () => {
    const history = localStorage.getItem(key);
    if (!history) return [];
    try {
      const parsed = JSON.parse(history) as { version: number; data: T[] };
      if (parsed.version === version) {
        return parsed.data;
      } else {
        return [];
      }
    } catch (e) {
      return [];
    }
  };

  const undo = () => {
    const history = get();
    if (history.length > 0) {
      const last = history.pop() as T;
      save(history);
      return last;
    }
    return null;
  };

  const newGame = () => {
    localStorage.removeItem(key);
  };

  const load = () => {
    const history = get();
    if (history.length > 0) {
      return history[history.length - 1];
    }
    return null;
  };

  return {
    add,
    undo,
    newGame,
    load,
  };
}
