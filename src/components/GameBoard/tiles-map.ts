import type { Board, Tile } from "game-core/pkg/wasm";
import { computed, type Ref } from "vue";

export interface TileData {
  tile: Tile;
  x: number;
  y: number;
}

export function useTilesMap(board: Ref<Board | null>) {
  const tiles = computed(() => {
    if (board.value === null) {
      return new Map<number, TileData>();
    }
    const b = board.value;

    return new Map(
      b.tiles.map((tile, index) => [
        tile.id,
        {
          tile,
          x: index % b.side_length,
          y: Math.floor(index / b.side_length),
        } satisfies TileData,
      ])
    );
  });

  function tileStyle(id: number) {
    const tile = tiles.value.get(id) ?? null;
    if (board.value === null || tile === null) {
      return {};
    }
    return {
      top: (tile.y / board.value.side_length) * 100 + "%",
      left: (tile.x / board.value.side_length) * 100 + "%",
    };
  }

  return {
    tiles,
    tileStyle,
  };
}
