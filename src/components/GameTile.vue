<script setup lang="ts">
import { DungeonTiles } from "@/dungeon_tiles";
import { getItem } from "@/items";
import type { Item, Tile } from "../../game-core/pkg";
import { computed } from "vue";

const props = defineProps<{
  tile: Tile | null;
  searchingFor: Item | null;
}>();
const dungeonTile = computed(() => {
  if (props.tile === null) {
    console.warn("Tile is null");
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
  if (props.tile === null) {
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
const item = computed(() => getItem(props.tile?.item));
const isSearchingFor = computed(() => {
  if (props.searchingFor === null) {
    return false;
  }
  return props.searchingFor === props.tile?.item;
});
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
    <div
      v-if="item !== null"
      class="item"
      :class="{
        highlight: isSearchingFor,
      }"
    >
      {{ item }}
    </div>
  </div>
</template>

<style scoped>
img {
  image-rendering: pixelated;
}

.tile-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  --edge: 5px;
}

.tile {
  position: absolute;
  top: var(--edge);
  left: var(--edge);
  right: var(--edge);
  bottom: var(--edge);
  border-radius: 5px;
  background-color: rgb(6, 6, 17);
  display: flex;
}

.tile:hover {
  outline: 6px solid rgb(6, 6, 17);
}

.item {
  position: absolute;
  top: var(--edge);
  left: var(--edge);
  right: var(--edge);
  bottom: var(--edge);
  display: flex;
  container-type: size;
  font-size: 2.5vmin;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  text-shadow: 0 0 5px black;
  filter: sepia(25%);
  pointer-events: none;
  z-index: 1;
}

.highlight {
  font-size: 3vmin;
  filter: none;
  text-shadow: 0 0 5px black, 0 0 8px rgb(0, 23, 87);
}
</style>
