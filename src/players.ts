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
  { side: "Top", index: 0 },
  { side: "Right", index: 0 },
  { side: "Bottom", index: 7 },
  { side: "Left", index: 7 },
  { side: "Top", index: 0 },
  { side: "Right", index: 0 },
  { side: "Bottom", index: 7 },
  { side: "Left", index: 7 },
];
