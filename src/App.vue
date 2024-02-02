<script setup lang="ts">
import { h, ref } from "vue";
import Board from "./components/Board.vue";
import PlayerCards from "./components/PlayerCards.vue";
import PlayerDialog from "./components/PlayerDialog.vue";

const showDialog = ref(false);
const showDialogFor = ref({
  id: 0,
  mode: "local" as "local" | "online" | null,
});

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
const playerSideMap = new Map<number, "bottom" | "left" | "right" | "top">([
  [0, "bottom"],
  [1, "bottom"],
  [2, "left"],
  [3, "left"],
  [4, "top"],
  [5, "top"],
  [6, "right"],
  [7, "right"],
]);
// See also https://vuejs.org/guide/extras/render-function#typing-functional-components
function OnePlayerCard(props: { id: number }) {
  return h(PlayerCards, {
    side: playerSideMap.get(props.id) ?? "bottom",
    id: props.id,
    isActive: true,
    hasPlayer: playersMap.value.has(props.id),
    count: playerItemCount(props.id),
    item: playerItemId(props.id),
    onClick: () => {
      showDialogFor.value = { id: props.id, mode: "local" };
      showDialog.value = true;
    },
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
    <PlayerDialog
      v-model:show="showDialog"
      :id="showDialogFor.id"
      :player-mode="showDialogFor.mode"
    ></PlayerDialog>
  </div>
</template>

<style scoped>
div {
  display: flex;
  min-width: 0;
  min-height: 0;
  flex-direction: column;
}

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
