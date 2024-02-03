<script setup lang="ts">
import { computed } from "vue";
import { Items, getItemNonZeroU8 } from "../items";
import { PlayerColors } from "../players";
import type { Item, PlayerId, Side } from "game-core/pkg/wasm";
const props = defineProps<{
  side: Side;
  id: PlayerId;
  isActive: boolean;
  hasPlayer: boolean;
  count: number;
  item: Item;
}>();

const playerColor = computed(() => PlayerColors[props.id]);
const orientation = computed(() =>
  props.side === "Top" || props.side === "Bottom" ? "vertical" : "horizontal"
);
const item = computed(() => getItemNonZeroU8(props.item));
</script>

<template>
  <div
    class="player-cards"
    :class="{
      'is-active': props.isActive,
      horizontal: orientation === 'horizontal',
    }"
  >
    <div class="card">
      <div class="card-inner">
        {{ hasPlayer ? props.count : "" }}
      </div>
    </div>

    <div v-if="hasPlayer" class="card item-card">
      <div class="card-inner">{{ item }}</div>
    </div>
  </div>
</template>

<style scoped>
.player-cards {
  display: flex;
  flex-direction: row;
}
.player-cards.horizontal {
  flex-direction: column;
}
.card {
  margin: 5px;
  border: 2px solid black;
  border-radius: 8px;

  --red: v-bind(playerColor);
  width: calc(63vmin * var(--card-scale));
  height: calc(88vmin * var(--card-scale));
  display: flex;
  justify-content: center;
  align-items: center;
}
.card:hover {
  cursor: pointer;
  outline: 2px solid black;
}
.horizontal .card {
  width: calc(88vmin * var(--card-scale));
  height: calc(63vmin * var(--card-scale));
}
.card-inner {
  border: 1px solid black;
  border-radius: 5px;
  width: calc(100% - 8px);
  height: calc(100% - 8px);

  --blue: rgba(255, 255, 255, 0);
  background-image: repeating-linear-gradient(
      45deg,
      transparent,
      transparent 12px,
      var(--red) 12px,
      var(--red) 14px
    ),
    repeating-linear-gradient(
      -45deg,
      transparent,
      transparent 12px,
      var(--red) 12px,
      var(--red) 14px
    ),
    linear-gradient(0deg, var(--blue), var(--blue) 100%);
}
.card-inner {
  font-weight: bold;
  font-size: 2em;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

.card.item-card .card-inner {
  font-size: 4em;
  background-image: none;
  color: white;
  font-weight: bold;
  filter: sepia(25%);
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 8px 2px var(--red);
  }
  50% {
    box-shadow: 0 0 8px 4px var(--red);
  }
  100% {
    box-shadow: 0 0 8px 2px var(--red);
  }
}
.is-active > * {
  animation: pulse 1s infinite;
}
</style>
