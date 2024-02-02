<script setup lang="ts">
import { DungeonTiles } from "@/dungeon_tiles";
import type { Tile } from "game-core/pkg/wasm";
import { computed } from "vue";

const props = defineProps<{
  tile: Tile | null;
}>();
const dungeonTile = computed(() => {
  if (!props.tile) {
    return DungeonTiles.Empty;
  } else if (props.tile.variant === "IShape") {
    return DungeonTiles.Abyss.IShape;
  } else if (props.tile.variant === "LShape") {
    return DungeonTiles.Abyss.LShape;
  } else if (props.tile.variant === "TShape") {
    return DungeonTiles.Abyss.TShape;
  } else {
    console.warn("Unknown tile variant", props.tile.variant);
    return DungeonTiles.Empty;
  }
});
const rotation = computed(() => {
  if (!props.tile) {
    return 0;
  }
  if (props.tile.rotation === "Zero") {
    return 0;
  } else if (props.tile.rotation === "Ninety") {
    return 90;
  } else if (props.tile.rotation === "OneEighty") {
    return 180;
  } else if (props.tile.rotation === "TwoSeventy") {
    return 270;
  } else {
    console.warn("Unknown tile rotation", props.tile.rotation);
    return 0;
  }
});
// TODO: Render item on top of tile
</script>

<template>
  <div class="tile-container">
    <div class="tile">
      <img
        :src="dungeonTile.img"
        alt="Tile"
        :style="{
          transform: `rotate(${rotation ?? 0}deg)`,
        }"
      />
    </div>
  </div>
</template>

<style scoped>
.tile-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.tile {
  width: calc(100% - 5px);
  height: calc(100% - 5px);
  display: flex;
  border-radius: 5px;
  background-color: rgb(6, 6, 17);
}
</style>
