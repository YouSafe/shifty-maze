export function groupBy<T, K extends string | number>(
  array: T[],
  key: (item: T) => K
): Map<K, T[]> {
  const map = new Map<K, T[]>();
  for (const item of array) {
    const k = key(item);
    if (!map.has(k)) {
      map.set(k, []);
    }
    map.get(k)!.push(item);
  }
  return map;
}
