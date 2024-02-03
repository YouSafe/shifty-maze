<script setup lang="ts">
import { computed } from "vue";
import GameTile from "./GameTile.vue";
import PlayerPiece from "./PlayerPiece.vue";
import type { Board, Player, PlayerId, Tile } from "game-core/pkg/wasm";

const props = defineProps<{
  board: Board | null;
  players: Map<PlayerId, Player>;
  activePlayer: PlayerId | null;
}>();

const tileCount = computed(() => props.board?.tiles.length ?? 0);

interface TileData {
  tile: Tile;
  x: number;
  y: number;
}

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
function tileStyle(id: number) {
  const board = props.board;
  if (board === null) {
    return {};
  }
  const tile = tilesMap.value.get(id);
  if (tile === undefined) {
    return {};
  }
  return {
    width: 100 / board.side_length + "%",
    height: 100 / board.side_length + "%",
    top: (tile.y / board.side_length) * 100 + "%",
    left: (tile.x / board.side_length) * 100 + "%",
  };
}
function playerStyle(id: PlayerId) {
  const board = props.board;
  if (board === null) {
    return {
      display: "none",
    };
  }
  const player = props.players.get(id);
  if (player === undefined) {
    return {
      display: "none",
    };
  }
  return {
    width: 100 / board.side_length + "%",
    height: 100 / board.side_length + "%",
    top: (player.position.y / board.side_length) * 100 + "%",
    left: (player.position.x / board.side_length) * 100 + "%",
  };
}

// TODO: Deal with stacked players
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height board">
        <div v-if="props.board === null" class="start-game">Start</div>
        <template v-else>
          <div class="tiles-wrapper">
            <div
              v-for="id in tileCount"
              :key="id"
              class="tile"
              :style="tileStyle(id - 1)"
            >
              <GameTile :tile="tilesMap.get(id - 1)?.tile ?? null" />
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
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
div {
  display: flex;
  min-width: 0;
  min-height: 0;
  flex-direction: column;
}
.tile {
  position: absolute;
  transition: top 1s ease-in-out, left 1s ease-in-out;
}
.player {
  position: absolute;
  transition: top 1s ease-in-out, left 1s ease-in-out;
}
.tiles-wrapper {
  padding: 10px;
}
.board {
  background-color: #e0e0e0;
  position: relative;
}
.start-game {
  font-size: 10vmin;
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
