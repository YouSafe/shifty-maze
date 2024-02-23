<script setup lang="ts">
import { computed } from "vue";
import { getItem } from "@/items";
import { PlayerColors } from "@/players";
import type { Item, PlayerId, Side } from "../../game-core/pkg";

const props = defineProps<{
  side: Side;
  id: PlayerId;
  isActive: boolean;
  hasPlayer: boolean;
  count: number;
  item: Item | null;
}>();

const playerColor = computed(() => PlayerColors[props.id]);
const orientation = computed(() =>
  props.side === "Top" || props.side === "Bottom" ? "vertical" : "horizontal"
);
const item = computed(() => getItem(props.item));
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
        <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
          <text
            x="50%"
            y="50%"
            dominant-baseline="central"
            text-anchor="middle"
          >
            {{ hasPlayer ? props.count : "" }}
          </text>
        </svg>
      </div>
    </div>

    <div v-if="hasPlayer" class="card item-card">
      <div class="card-inner">
        {{ item }}
      </div>
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
  font-size: 3vmin;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

.card-inner svg text {
  stroke-linejoin: round;
  text-anchor: middle;
  fill: black;
  stroke: white;
  stroke-width: 6px;
  paint-order: stroke;
}

.card.item-card .card-inner {
  font-size: 5vmin;
  background-image: none;
  color: white;
  font-weight: bold;
  filter: sepia(25%);
}

.is-active > * {
  box-shadow: 0 0 8px 4px var(--red);
}
</style>
