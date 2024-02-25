<script setup lang="ts">
import { computed } from "vue";
import SquareContainer from "@/components/SquareContainer.vue";
import GameTile from "@/components/GameTile.vue";
import PlayerPiece from "@/components/PlayerPiece.vue";
import type {
  Board,
  GamePhase,
  GameStartSettings,
  Item,
  Player,
  PlayerId,
  Position,
  SideIndex,
} from "game-core/pkg";
import GameSettings from "@/components/GameSettings.vue";
import { NButton } from "naive-ui";
import { groupBy } from "@/array-utils";
import { PlayerColors } from "@/players";
import SideArrows from "@/components/GameBoard/SideArrows.vue";
import { useTilesMap } from "@/components/GameBoard/tiles-map";

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
  (e: "shift-tiles", side_index: SideIndex): void;
}>();

// Free tile + 1
const maxTileId = computed(() => (props.board?.tiles.length ?? 0) + 1);
const sideLength = computed(() => props.board?.side_length ?? 1);

const { tiles, animatedTiles, tileStyle } = useTilesMap(
  computed(() => props.board)
);

const tileSize = computed(() => 100 / sideLength.value + "cqw");

const positionToMapKey = (position: Position) =>
  `${position.x} - ${position.y}`;
const positionPlayersMap = computed(() =>
  groupBy([...props.players.values()], (player) =>
    positionToMapKey(player.position)
  )
);

const playerRenderOffsets = [
  { x: 0.5, y: -1 },
  { x: -1, y: -0.5 },
  { x: 1, y: -0.5 },
  { x: -0.5, y: -1 },
  //
  { x: -1, y: 0.5 },
  { x: 0.5, y: 1 },
  { x: -0.5, y: 1 },
  { x: 1, y: 0.5 },
];

function playerStyle(id: PlayerId) {
  const player = props.players.get(id) ?? null;
  if (player === null) {
    return {
      display: "none",
    };
  }

  const hasMultiplePlayers =
    (positionPlayersMap.value.get(positionToMapKey(player.position))?.length ??
      0) > 1;
  const transform = hasMultiplePlayers
    ? `scale(0.9) translate(${30 * (playerRenderOffsets[id]?.x ?? 0)}%, ${
        30 * (playerRenderOffsets[id]?.y ?? 0)
      }%)`
    : "";
  return {
    top: (player.position.y / sideLength.value) * 100 + "%",
    left: (player.position.x / sideLength.value) * 100 + "%",
    transform,
  };
}

const startPositionRenderOffsets = [
  { x: 0, y: -1 },
  { x: -1, y: 0 },
  { x: 1, y: 0 },
  { x: 0, y: -1 },
  //
  { x: -1, y: 0 },
  { x: 0, y: 1 },
  { x: 0, y: 1 },
  { x: 1, y: 0 },
];

function startCirclePosition(id: PlayerId) {
  const player = props.players.get(id) ?? null;
  if (player === null) {
    return {
      display: "none",
    };
  }

  const transform = `translate(${
    30 * (startPositionRenderOffsets[id]?.x ?? 0)
  }%, ${30 * (startPositionRenderOffsets[id]?.y ?? 0)}%)`;
  return {
    top: (player.start_position.y / sideLength.value) * 100 + "%",
    left: (player.start_position.x / sideLength.value) * 100 + "%",
    transform,
  };
}

function startCircleStyle(id: PlayerId) {
  const player = props.players.get(id) ?? null;
  if (player !== null && id === props.activePlayer) {
    return {
      backgroundColor: PlayerColors[id],
      border: "1px solid white",
    };
  } else {
    return {};
  }
}

function tryMovePlayer(tileId: number) {
  if (props.phase !== "MovePlayer") {
    return;
  }
  if (props.activePlayer === null) {
    return;
  }
  const tile = tiles.value.get(tileId);
  if (tile === undefined) {
    return;
  }
  emits("player-move", props.activePlayer, tile.x, tile.y);
}

function startGame() {
  emits("start-game", gameSettings.value);
}
</script>

<template>
  <SquareContainer>
    <div class="board-container">
      <div class="board">
        <div v-if="props.board === null" class="start-game">
          <n-button
            secondary
            round
            size="large"
            type="primary"
            :disabled="gameSettings.players.size < 2"
            @click="startGame"
          >
            <h1>Press Start</h1>
          </n-button>
          <GameSettings
            v-model:cards-per-player="gameSettings.items_per_player"
            v-model:side-length="gameSettings.side_length"
          ></GameSettings>
          <h2>
            <span>{{ gameSettings.players.size }}</span>
            <span>&nbsp;</span>
            <span v-if="gameSettings.players.size !== 1">Players</span
            ><span v-else>Player</span>
          </h2>
          <ul>
            <li
              v-for="playerId in [...gameSettings.players.keys()].toSorted(
                (a, b) => a - b
              )"
              :key="playerId"
            >
              Player {{ playerId }}
            </li>
          </ul>
        </div>
        <template v-else>
          <div class="tiles-wrapper">
            <!-- Vue.js v-for is freaking cursed and counts like Lua. But we're smart and use the index. -->
            <template v-for="(_, id) in maxTileId" :key="id">
              <div
                v-if="animatedTiles.has(id)"
                class="tile"
                :style="tileStyle(id)"
              >
                <GameTile
                  :tile="animatedTiles.get(id)?.tile ?? null"
                  :searching-for="props.activePlayerItem"
                  @click="() => tryMovePlayer(id)"
                />
              </div>
            </template>
          </div>
          <div class="tiles-wrapper">
            <div
              v-for="[id, player] of props.players.entries()"
              :key="id"
              class="start-circle"
              :style="startCirclePosition(id)"
            >
              <div :style="startCircleStyle(id)"></div>
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
            <SideArrows
              :board="props.board"
              :phase="props.phase"
              @shift-tiles="($event) => emits('shift-tiles', $event)"
            ></SideArrows>
          </div>
        </template>
      </div>
    </div>
  </SquareContainer>
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
  width: 30%;
  height: 30%;
  border-radius: 50%;
}

.board-container {
  margin: 1vmin;
  padding: 1vmin;
  background-color: #e0e0e0;
  height: 100%;
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
</style>
