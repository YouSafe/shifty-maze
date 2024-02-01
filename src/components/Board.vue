<script setup lang="ts">
const props = defineProps<{
  board: any | null;
}>();
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height blue">
        <div v-if="board === null" class="start-game">Start</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
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
