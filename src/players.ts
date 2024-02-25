import { useLastChanged } from "@vueuse/core";
import type { PlayerId, Side } from "game-core/pkg";
import { computed, type WatchSource } from "vue";

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
