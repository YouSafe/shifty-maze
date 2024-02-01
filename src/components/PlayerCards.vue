<script setup lang="ts">
import { computed } from "vue";
import { Items } from "../items";
import { PlayerColors } from "../players";
const props = defineProps<{
  orientation: "horizontal" | "vertical";
  id: number;
  isActive: boolean;
  hasPlayer: boolean;
  count: number;
  item: number;
}>();

const playerColor = computed(() => PlayerColors[props.id]);
</script>

<template>
  <div
    :class="{
      'is-active': props.isActive,
    }"
    :style="{
      flexDirection: orientation === 'horizontal' ? 'row' : 'column',
    }"
  >
    <div
      class="card"
      :class="orientation === 'horizontal' ? 'horizontal' : 'vertical'"
    >
      {{ hasPlayer ? props.count : "" }}
    </div>

    <div
      v-if="hasPlayer"
      class="card item-card"
      :class="orientation === 'horizontal' ? 'horizontal' : 'vertical'"
    >
      {{ Items[item] }}
    </div>
  </div>
</template>

<style scoped>
.card {
  margin: 5px;
  border: 2px solid black;
  border-radius: 8px;
  /** red cross hatched background (playing card) */
  --red: v-bind(playerColor);
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
  font-weight: bold;
  font-size: 2em;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}
.card.item-card {
  font-size: 4em;
  background-image: none;
}
.card:hover {
  cursor: pointer;
  outline: 2px solid black;
}
.horizontal {
  width: calc(63vmin * var(--card-scale));
  height: calc(88vmin * var(--card-scale));
}
.vertical {
  width: calc(88vmin * var(--card-scale));
  height: calc(63vmin * var(--card-scale));
}

/** css animation */
@keyframes pulse {
  0% {
    box-shadow: 0 0 10px 1px rgba(12, 113, 160, 0.822);
  }
  50% {
    box-shadow: 0 0 10px 3px rgba(12, 113, 160, 0.822);
  }
  100% {
    box-shadow: 0 0 10px 1px rgba(12, 113, 160, 0.822);
  }
}
.is-active > * {
  animation: pulse 1s infinite;
}
</style>
