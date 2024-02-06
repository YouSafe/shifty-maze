<script setup lang="ts">
import { computed, ref } from "vue";
import GameTile from "./GameTile.vue";
import PlayerPiece from "./PlayerPiece.vue";
import type {
  Board,
  GamePhase,
  GameStartSettings,
  Item,
  Player,
  PlayerId,
  Position,
  Rotation,
  SideIndex,
  Tile,
} from "game-core/pkg/wasm";
import GameSettings from "./GameSettings.vue";
import { NButton } from "naive-ui";
import { groupBy } from "@/array-utils";
import { PlayerColors } from "@/players";

const gameSettings = defineModel<GameStartSettings>("startSettings", {
  required: true,
});

const props = defineProps<{
  board: Board | null;
  players: Map<PlayerId, Player>;
  activePlayer: PlayerId | null;
  activePlayerItem: Item | null;
  phase: GamePhase;
}>();

const emits = defineEmits<{
  (e: "player-move", player: PlayerId, x: number, y: number): void;
  (e: "start-game", settings: GameStartSettings): void;
  (e: "shift-tiles", side_index: SideIndex, insertRotation: Rotation): void;
}>();

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
        x: 6 - Math.floor(index / board.side_length),
        y: index % board.side_length,
      },
    ])
  );
});

interface SideArrow {
  id: string;
  side_index: SideIndex;
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
      (percent: string, index: number) => ({
        side_index: { Top: index },
        top: "0",
        left: percent,
      }),
      (percent: string, index: number) => ({
        side_index: { Bottom: index },
        bottom: "0",
        left: percent,
      }),
      (percent: string, index: number) => ({
        side_index: { Left: index },
        left: "0",
        top: percent,
      }),
      (percent: string, index: number) => ({
        side_index: { Right: index },
        right: "0",
        top: percent,
      }),
    ] as const
  ).flatMap((position, mainIndex) => {
    const arrows: SideArrow[] = [];
    for (let i = 1; i < sideLength; i += 2) {
      arrows.push({
        id: `${mainIndex}-${i}`,
        ...position((i / sideLength) * 100 + "%", i),
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

function arrowStyle(arrow: SideArrow) {
  return Object.keys(arrow.side_index)[0]
}

const positionToMapKey = (position: Position) =>
  `${position.x} - ${position.y}`;
const positionPlayersMap = computed(() =>
  groupBy([...props.players.values()], (player) =>
    positionToMapKey(player.position)
  )
);
const startPositionPlayersMap = computed(() =>
  groupBy([...props.players.values()], (player) =>
    positionToMapKey(player.start_position)
  )
);

const playerRenderOffsets = [
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
    ? `scale(0.9) translate(${30 * (playerRenderOffsets[id]?.x ?? 0)}%, ${30 * (playerRenderOffsets[id]?.y ?? 0)
    }%)`
    : "";
  return {
    top: (player.position.y / board.side_length) * 100 + "%",
    left: (player.position.x / board.side_length) * 100 + "%",
    transform,
  };
}

function startCircleStyle(id: PlayerId) {
  const board = props.board;
  const player = props.players.get(id) ?? null;
  if (board === null || player === null) {
    return {
      display: "none",
    };
  }
  const hasMultiplePlayers =
    (startPositionPlayersMap.value.get(positionToMapKey(player.start_position))
      ?.length ?? 0) > 1;
  const transform = hasMultiplePlayers
    ? `scale(0.9) translate(${30 * (playerRenderOffsets[id]?.x ?? 0)}%, ${30 * (playerRenderOffsets[id]?.y ?? 0)
    }%)`
    : "";
  return {
    top: (player.start_position.y / board.side_length) * 100 + "%",
    left: (player.start_position.x / board.side_length) * 100 + "%",
    transform,
  };
}

const playerColors = computed(() => {
  return PlayerColors;
});

function tryMovePlayer(tileId: number) {
  if (props.phase !== "MovePlayer") {
    return;
  }
  if (props.activePlayer === null) {
    return;
  }
  const tile = tilesMap.value.get(tileId);
  if (tile === undefined) {
    return;
  }
  emits("player-move", props.activePlayer, tile.x, tile.y);
}

function startShiftTiles(side_index: SideIndex) {
  emits("shift-tiles", side_index, "Zero");
}

function startGame() {
  emits("start-game", gameSettings.value);
}
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height board-container">
        <div class="board">
          <div v-if="props.board === null" class="start-game">
            <n-button secondary round size="large" type="primary" :disabled="gameSettings.players.length < 2"
              @click="startGame">
              <h1>Press Start</h1>
            </n-button>
            <GameSettings v-model:cards-per-player="gameSettings.items_per_player"
              v-model:side-length="gameSettings.side_length"></GameSettings>
            <h2>
              <span>{{ gameSettings.players.length }}</span>
              <span>&nbsp;</span>
              <span v-if="gameSettings.players.length !== 1">Players</span><span v-else>Player</span>
            </h2>
            <ul>
              <li v-for="playerId in gameSettings.players" :key="playerId">
                Player {{ playerId }}
              </li>
            </ul>
          </div>
          <template v-else>
            <div class="tiles-wrapper">
              <div v-for="id in tileCount" :key="id" class="tile" :style="tileStyle(id - 1)">
                <GameTile :tile="tilesMap.get(id - 1)?.tile ?? null" :searching-for="props.activePlayerItem"
                  @click="() => tryMovePlayer(id - 1)" />
              </div>
            </div>
            <div class="tiles-wrapper">
              <div v-for="[id, player] of props.players.entries()" :key="id" class="start-circle"
                :style="startCircleStyle(id)">
                <div :style="{
                  backgroundColor: playerColors[id],
                }"></div>
              </div>
            </div>
            <div class="tiles-wrapper">
              <div v-for="[id, player] of props.players.entries()" :key="id" class="player" :style="playerStyle(id)">
                <PlayerPiece :player="player" :is-active="props.phase === 'MovePlayer' &&
                  player.id === props.activePlayer
                  " />
              </div>
            </div>
            <div class="tiles-wrapper">
              <div v-for="arrow in sideArrows" :key="arrow.id" class="arrow-wrapper" :class="arrowStyle(arrow)" :style="{
                top: arrow.top,
                left: arrow.left,
                right: arrow.right,
                bottom: arrow.bottom,
              }" @click="() => startShiftTiles(arrow.side_index)">
                <div class="arrow" :class="{
                  [arrowStyle(arrow)]: true,
                  highlight: props.phase === 'MoveTiles',
                }"></div>
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
  pointer-events: none;
}

.start-circle {
  position: absolute;
  width: var(--tile-size);
  height: var(--tile-size);
  pointer-events: none;
  display: flex;
  justify-content: center;
  align-items: center;
}

.start-circle div {
  width: 40%;
  height: 40%;
  border-radius: 50%;
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
  flex-direction: column;
}

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

.arrow-wrapper:has(.arrow.highlight) {
  animation: pulse 1.2s infinite;
}

/* animation for drop shadow */
@keyframes pulse {
  0% {
    filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0));
  }

  70% {
    filter: drop-shadow(0 0 4px rgba(0, 0, 0, 0.5));
  }

  100% {
    filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0));
  }
}
</style>
