<script setup lang="ts">
import { h, ref } from "vue";
import Board from "./components/Board.vue";
import PlayerCards from "./components/PlayerCards.vue";

interface Player {
  id: number;
  collected: any[];
  to_collect: any[];
}

const playersMap = ref(new Map<number, Player>());
function playerItemCount(id: number) {
  const player = playersMap.value.get(id);
  if (!player) return 0;
  return player.to_collect.length;
}
function playerItemId(id: number) {
  const player = playersMap.value.get(id);
  if (!player) return 0;
  return player.to_collect[0];
}
function playerOrientation(id: number) {
  return id === 0 || id === 1 || id === 4 || id === 5
    ? "horizontal"
    : "vertical";
}
// TODO: Respond to click (add & remove player - do a popup!)

// See also https://vuejs.org/guide/extras/render-function#typing-functional-components
function OnePlayerCard(props: { id: number }) {
  return h(PlayerCards, {
    orientation: playerOrientation(props.id),
    id: props.id,
    isActive: true,
    hasPlayer: playersMap.value.has(props.id),
    count: playerItemCount(props.id),
    item: playerItemId(props.id),
  });
}
OnePlayerCard.props = {
  id: Number,
};
</script>

<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height">
        <div class="top space-between">
          <OnePlayerCard :id="4"></OnePlayerCard>
          <OnePlayerCard :id="5"></OnePlayerCard>
        </div>
        <div class="middle">
          <div class="space-between">
            <OnePlayerCard :id="3"></OnePlayerCard>
            <OnePlayerCard :id="2"></OnePlayerCard>
          </div>
          <Board class="board" :board="null" />
          <div class="space-between">
            <OnePlayerCard :id="6"></OnePlayerCard>
            <OnePlayerCard :id="7"></OnePlayerCard>
          </div>
        </div>
        <div class="bottom space-between">
          <OnePlayerCard :id="1"></OnePlayerCard>
          <OnePlayerCard :id="0"></OnePlayerCard>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.top {
  padding: 0 calc(88vmin * var(--card-scale) + 15px);
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
  padding: 0 calc(88vmin * var(--card-scale) + 15px);
  flex-direction: row;
}
.space-between {
  justify-content: space-between;
}
</style>
