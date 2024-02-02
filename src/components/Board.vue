<script setup lang="ts">
import { computed } from "vue";
import Tile from "./Tile.vue";

const props = defineProps<{
  board: Board | null;
}>();

interface Board {
  tiles: any[];
  side_length: number;
}
const tileCount = computed(() => props.board?.tiles.length ?? 0);

const tilesMap = computed(() => {
  const board = props.board;
  if (board === null) {
    return new Map();
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
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height blue">
        <div v-if="props.board === null" class="start-game">Start</div>
        <div class="tiles-wrapper" v-else>
          <div
            v-for="id in tileCount"
            :key="id"
            class="tile"
            :style="tileStyle(id - 1)"
          >
            <Tile :tile="tilesMap.get(id - 1).tile" />
          </div>
        </div>
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
.tiles-wrapper {
  padding: 10px;
}
.blue {
  background-color: rgb(9, 9, 107);
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
