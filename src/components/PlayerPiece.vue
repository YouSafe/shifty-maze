<script setup lang="ts">
import { DungeonTiles } from "@/dungeon_tiles";
import { PlayerColors } from "@/players";
import type { Player } from "game-core/pkg/wasm";
import { computed } from "vue";

const props = defineProps<{
  player: Player;
  isActive: boolean;
}>();

const dungeonTile = computed(() => {
  return DungeonTiles.Players[props.player.id];
});
const playerColor = computed(() => PlayerColors[props.player.id]);
</script>

<template>
  <div class="tile" :class="{ active: props.isActive }">
    <div
      class="indicator-circle"
      :style="{
        backgroundColor: playerColor,
        opacity: props.isActive ? 0.8 : 0.4,
      }"
    ></div>
    <img :src="dungeonTile.img" alt="Player" />
  </div>
</template>

<style scoped>
.tile {
  width: 100%;
  height: 100%;
  display: flex;
  position: relative;
}
.tile:hover {
  cursor: pointer;
}
img {
  position: absolute;
  image-rendering: pixelated;
  width: auto;
  height: 50%;
  margin-left: auto;
  margin-right: auto;
  left: 0;
  right: 0;
  top: 10%;
}
.indicator-circle {
  position: absolute;
  width: 60%;
  height: 25%;
  border-radius: 50%;
  margin: auto;
  bottom: 30%;
  left: 0;
  right: 0;
}
.active .indicator-circle {
  border: 2px solid white;
  box-shadow: 0 0 10px 5px white;
}
</style>
