import type { SideIndex } from "game-core/pkg/wasm";

export const PlayerColors = [
  "#332288",
  "#117733",
  "#44aa99",
  "#88ccee",
  "#ddcc77",
  "#cc6677",
  "#aa4499",
  "#882255",
];

export const PlayerSides: SideIndex[] = [
  { Bottom: 0 },
  { Bottom: 7 },
  { Left: 0 },
  { Left: 7 },
  { Top: 0 },
  { Top: 7 },
  { Top: 0 },
  { Top: 7 },
];
