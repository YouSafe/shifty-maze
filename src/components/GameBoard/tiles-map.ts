import type { Board, Side, SideIndex, Tile } from "game-core/pkg/wasm";
import { computed, ref, watchEffect, type Ref, watch } from "vue";

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
      b.tiles.map((tile, index): [number, TileData] => [
        tile.id,
        {
          tile,
          x: index % b.side_length,
          y: Math.floor(index / b.side_length),
        },
      ])
    );
  });

  const animatedTiles = ref(new Map<number, TileData>());
  const animationTimeouts = ref<number[]>([]);
  watch(
    () => board.value?.free_tile?.tile?.id,
    (_) => {
      const b = board.value;
      const freeTile = b?.free_tile;
      // Free tile, if it exists, is the newly freed up tile
      animatedTiles.value.clear();
      animationTimeouts.value.forEach((timeout) => {
        globalThis.clearTimeout(timeout);
      });
      if (b === null || freeTile === undefined) {
        return;
      }

      tiles.value.forEach((tile, id) => {
        animatedTiles.value.set(id, tile);
      });

      // Animate the inserted tile coming from the edge
      if (freeTile.side_with_index !== undefined) {
        const { x, y } = sideIndexToXY(
          oppositeSideIndex(freeTile.side_with_index),
          b.side_length
        );
        const insertedTile = [...tiles.value.values()].find(
          (tile) => tile.x === x && tile.y === y
        );
        if (insertedTile === undefined) {
          console.error("Inserted tile not found");
        } else {
          const startXY = stepTowards(
            { x, y },
            oppositeSide(freeTile.side_with_index.side)
          );
          animatedTiles.value.set(insertedTile.tile.id, {
            tile: insertedTile.tile,
            x: startXY.x,
            y: startXY.y,
          });
          animationTimeouts.value.push(
            globalThis.setTimeout(() => {
              animatedTiles.value.set(insertedTile.tile.id, {
                tile: insertedTile.tile,
                x,
                y,
              });
            }, 0)
          );
        }
      }

      // Animate the removed tile going beyond the edge
      if (freeTile.side_with_index !== undefined) {
        const { x, y } = stepTowards(
          sideIndexToXY(freeTile.side_with_index, b.side_length),
          freeTile.side_with_index.side
        );

        animatedTiles.value.set(freeTile.tile.id, {
          tile: freeTile.tile,
          x,
          y,
        });
      }

      animationTimeouts.value.push(
        globalThis.setTimeout(() => {
          animatedTiles.value.delete(freeTile.tile.id);
        }, 900)
      );
    },
    {
      immediate: true,
    }
  );

  function tileStyle(id: number) {
    const tile = animatedTiles.value.get(id) ?? null;
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
    animatedTiles: computed(() => animatedTiles.value),
    tileStyle,
  };
}

function sideIndexToXY(sideIndex: SideIndex, side_length: number) {
  let x: number, y: number;
  if (sideIndex.side === "Top") {
    x = sideIndex.index;
    y = 0;
  } else if (sideIndex.side === "Right") {
    x = side_length - 1;
    y = sideIndex.index;
  } else if (sideIndex.side === "Bottom") {
    x = sideIndex.index;
    y = side_length - 1;
  } else if (sideIndex.side === "Left") {
    x = 0;
    y = sideIndex.index;
  } else {
    throw new Error("Invalid side");
  }

  return { x, y };
}

function oppositeSide(side: Side): Side {
  if (side === "Top") {
    return "Bottom";
  } else if (side === "Right") {
    return "Left";
  } else if (side === "Bottom") {
    return "Top";
  } else if (side === "Left") {
    return "Right";
  } else {
    throw new Error("Invalid side");
  }
}

function oppositeSideIndex(sideIndex: SideIndex): SideIndex {
  return {
    side: oppositeSide(sideIndex.side),
    index: sideIndex.index,
  };
}

function stepTowards(
  value: { x: number; y: number },
  side: Side
): { x: number; y: number } {
  if (side === "Top") {
    return { x: value.x, y: value.y - 1 };
  } else if (side === "Right") {
    return { x: value.x + 1, y: value.y };
  } else if (side === "Bottom") {
    return { x: value.x, y: value.y + 1 };
  } else if (side === "Left") {
    return { x: value.x - 1, y: value.y };
  } else {
    throw new Error("Invalid side");
  }
}
