<script setup lang="ts">
import { reactive } from "vue";
import { NInputNumber, NSpace, NSelect } from "naive-ui";

const sideLength = defineModel("sideLength", { type: Number, required: true });
const cardsPerPlayer = defineModel("cardsPerPlayer", {
  type: Number,
  required: true,
});

const sideLengthOptions = reactive([
  { label: "5", value: 5 },
  { label: "7", value: 7 },
  { label: "9", value: 9 },
  { label: "11", value: 11 },
  { label: "13", value: 13 },
  { label: "15", value: 15 },
]);
</script>
<template>
  <n-space vertical>
    <label>
      Board side length
      <n-select v-model:value="sideLength" :options="sideLengthOptions" />
    </label>
    <label>
      Cards per player
      <n-input-number v-model:value="cardsPerPlayer" :min="1" />
    </label>
  </n-space>
</template>

<style scoped></style>
