<script setup lang="ts">
import { computed, ref } from "vue";
import GameTile from "./GameTile.vue";
import PlayerPiece from "./PlayerPiece.vue";
import type {
  Board,
  Player,
  PlayerId,
  Position,
  Side,
  Tile,
} from "game-core/pkg/wasm";
import GameSettings from "./GameSettings.vue";
import type { GameStartSettings } from "@/game";
import { NButton } from "naive-ui";

const props = defineProps<{
  board: Board | null;
  players: Map<PlayerId, Player>;
  activePlayer: PlayerId | null;
  activePlayerItem: number | null;
}>();

const emits = defineEmits<{
  (e: "player-move", player: PlayerId, x: number, y: number): void;
  (e: "start-game", settings: GameStartSettings): void;
}>();

const gameSettings = ref<GameStartSettings>({
  board_side_length: 7,
  cards_per_player: 6,
});

const tileCount = computed(() => props.board?.tiles.length ?? 0);

interface TileData {
  tile: Tile;
  x: number;
  y: number;
}

const tileSize = computed(() => {
  const board = props.board;
  if (board === null) {
    return "0";
  }
  return 100 / board.side_length + "cqw";
});

const tilesMap = computed(() => {
  const board = props.board;
  if (board === null) {
    return new Map<number, TileData>();
  }
  return new Map(
    board.tiles.map((tile, index) => [
      tile.id,
      {
        tile,
        x: index % board.side_length,
        y: Math.floor(index / board.side_length),
      },
    ])
  );
});

interface SideArrow {
  id: string;
  side: Side;
  top?: string;
  left?: string;
  right?: string;
  bottom?: string;
}

const sideArrows = computed(() => {
  const board = props.board;
  if (board === null) {
    return [];
  }
  const sideLength = board.side_length;
  return (
    [
      (percent: string) => ({
        side: "Top" as const,
        top: "0",
        left: percent,
      }),
      (percent: string) => ({
        side: "Bottom" as const,
        bottom: "0",
        left: percent,
      }),
      (percent: string) => ({
        side: "Left" as const,
        left: "0",
        top: percent,
      }),
      (percent: string) => ({
        side: "Right" as const,
        right: "0",
        top: percent,
      }),
    ] as const
  ).flatMap((position, mainIndex) => {
    const arrows: SideArrow[] = [];
    for (let i = 1; i < sideLength; i += 2) {
      arrows.push({
        id: `${mainIndex}-${i}`,
        ...position((i / sideLength) * 100 + "%"),
      });
    }
    return arrows;
  });
});

function tileStyle(id: number) {
  const board = props.board;
  const tile = tilesMap.value.get(id) ?? null;
  if (board === null || tile === null) {
    return {};
  }
  return {
    top: (tile.y / board.side_length) * 100 + "%",
    left: (tile.x / board.side_length) * 100 + "%",
  };
}

const positionToMapKey = (position: Position) =>
  `${position.x} - ${position.y}`;
const positionPlayersMap = computed(() =>
  groupBy([...props.players.values()], (player) =>
    positionToMapKey(player.position)
  )
);

function groupBy<T, K extends string | number>(
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

const playerOffsets = [
  { x: -1, y: -1 },
  { x: 0, y: -1 },
  { x: 1, y: -1 },
  { x: -1, y: 0 },
  //
  { x: -1, y: 0 },
  { x: -1, y: 1 },
  { x: 0, y: 1 },
  { x: 1, y: 1 },
];

function playerStyle(id: PlayerId) {
  const board = props.board;
  const player = props.players.get(id) ?? null;
  if (board === null || player === null) {
    return {
      display: "none",
    };
  }

  const hasMultiplePlayers =
    (positionPlayersMap.value.get(positionToMapKey(player.position))?.length ??
      0) > 1;
  const transform = hasMultiplePlayers
    ? `scale(0.9) translate(${30 * (playerOffsets[id]?.x ?? 0)}%, ${
        30 * (playerOffsets[id]?.y ?? 0)
      }%)`
    : "";
  console.log(transform);
  return {
    top: (player.position.y / board.side_length) * 100 + "%",
    left: (player.position.x / board.side_length) * 100 + "%",
    transform,
  };
}

function startGame() {
  emits("start-game", gameSettings.value);
}

// TODO: Deal with stacked players
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height board-container">
        <div class="board">
          <div v-if="props.board === null" class="start-game">
            <n-button
              secondary
              round
              size="large"
              type="primary"
              @click="startGame"
            >
              <h1>Press Start</h1>
            </n-button>
            <GameSettings
              v-model:cards-per-player="gameSettings.cards_per_player"
              v-model:side-length="gameSettings.board_side_length"
            ></GameSettings>
          </div>
          <template v-else>
            <div class="tiles-wrapper">
              <div
                v-for="id in tileCount"
                :key="id"
                class="tile"
                :style="tileStyle(id - 1)"
              >
                <GameTile
                  :tile="tilesMap.get(id - 1)?.tile ?? null"
                  :searching-for="props.activePlayerItem"
                />
              </div>
            </div>
            <div class="tiles-wrapper">
              <div
                v-for="[id, player] of props.players.entries()"
                :key="id"
                class="player"
                :style="playerStyle(id)"
              >
                <PlayerPiece
                  :player="player"
                  :is-active="player.id === props.activePlayer"
                />
              </div>
            </div>
            <div class="tiles-wrapper">
              <div
                v-for="arrow in sideArrows"
                :key="arrow.id"
                class="arrow-wrapper"
                :class="arrow.side"
                :style="{
                  top: arrow.top,
                  left: arrow.left,
                  right: arrow.right,
                  bottom: arrow.bottom,
                }"
              >
                <div class="arrow" :class="arrow.side"></div>
              </div>
            </div>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tile,
.player {
  position: absolute;
  transition: top 1s ease-in-out, left 1s ease-in-out;
  width: var(--tile-size);
  height: var(--tile-size);
}
.player {
  pointer-events: none;
}
.board-container {
  margin: 1vmin;
  padding: 1vmin;
  background-color: #e0e0e0;
}
.board {
  /** The frick is this? https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment/Container_queries */
  container-type: size;
  --tile-size: v-bind(tileSize);
  width: 100%;
  height: 100%;
  position: relative;
}
.start-game {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  gap: 20px;
}
/* I hath nu idea. The numbers are magic*/
.arrow-wrapper {
  position: absolute;
  height: calc(var(--tile-size));
  width: calc(var(--tile-size));
  transform-origin: 50% 50%;
}
.arrow-wrapper.Top {
  transform: translateY(-2vmin);
}
.arrow-wrapper.Bottom {
  transform: rotate(180deg) translateY(-2vmin);
}
.arrow-wrapper.Right {
  transform: rotate(90deg) translateY(-2vmin);
}
.arrow-wrapper.Left {
  transform: rotate(-90deg) translateY(-2vmin);
}
.arrow {
  width: 100%;
  height: 2vmin;
  margin-top: 0vmin;
  background-color: #7c7c7c;
  clip-path: polygon(50% 100%, 100% 0%, 0 0%);
}
.arrow-wrapper:hover .arrow {
  background-color: #4c4c4c;
}
</style>
