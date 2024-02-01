<script setup lang="ts">
import { ref } from "vue";
import Board from "./components/Board.vue";
import PlayerCards from "./components/PlayerCards.vue";
</script>

<template>
  <div class="fullsize">
    <div class="top space-between">
      <PlayerCards orientation="horizontal" :count="3" :item="0" />
      <PlayerCards orientation="horizontal" :count="3" :item="0" />
    </div>
    <div class="middle">
      <div class="space-between">
        <PlayerCards orientation="vertical" :count="3" :item="1" />
        <PlayerCards orientation="vertical" :count="3" :item="1" />
      </div>
      <Board class="board" />
      <div class="space-between">
        <PlayerCards orientation="vertical" :count="3" :item="2" />
        <PlayerCards orientation="vertical" :count="3" :item="2" />
      </div>
    </div>
    <div class="bottom space-between">
      <PlayerCards orientation="horizontal" :count="3" :item="5" />
      <PlayerCards orientation="horizontal" :count="3" :item="5" />
    </div>
  </div>
</template>

<style scoped>
.top {
  padding: 0 calc(88vmin * 0.2 + 15px);
  flex-direction: row;
  justify-content: space-between;
}
.middle {
  display: flex;
  flex-direction: row;
  flex: 1;
}
.board {
  flex: 1;
}
.bottom {
  padding: 0 calc(88vmin * 0.2 + 15px);
  flex-direction: row;
}
.space-between {
  justify-content: space-between;
}

.fullsize {
  width: 100%;
  height: 100%;
}
</style>
