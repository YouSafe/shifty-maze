<script setup lang="ts">
import { Items } from "../items";
const props = defineProps<{
  orientation: "horizontal" | "vertical";
  count: number;
  item: number;
}>();
</script>

<template>
  <div
    :style="{
      flexDirection: orientation === 'horizontal' ? 'row' : 'column',
    }"
  >
    <div
      class="card"
      :class="orientation === 'horizontal' ? 'horizontal' : 'vertical'"
    >
      {{ props.count }}
    </div>

    <div
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
  --red: rgba(255, 84, 84, 0.411);
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
  width: calc(63vmin * 0.2);
  height: calc(88vmin * 0.2);
}
.vertical {
  width: calc(88vmin * 0.2);
  height: calc(63vmin * 0.2);
}
</style>
