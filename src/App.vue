<script setup lang="ts">
import { h, ref } from "vue";
import GameBoard from "./components/GameBoard.vue";
import PlayerCards from "./components/PlayerCards.vue";
import PlayerDialog from "./components/PlayerDialog.vue";
import { useGame, type PlayerMode } from "./game";
import type { Player, Side } from "game-core/pkg/wasm";

const showDialog = ref(false);
const showDialogFor = ref({
  id: 0,
  mode: "local" as PlayerMode | null,
});

const game = useGame();

const playerSides: Side[] = [
  "Bottom",
  "Bottom",
  "Left",
  "Left",
  "Top",
  "Top",
  "Right",
  "Right",
];
// See also https://vuejs.org/guide/extras/render-function#typing-functional-components
function OnePlayerCard(props: { id: number }) {
  return h(PlayerCards, {
    side: playerSides[props.id] ?? "bottom",
    id: props.id,
    isActive: game.activePlayer.value === props.id,
    hasPlayer: game.playerHelper.hasPlayer(props.id),
    count: game.playerHelper.itemCount(props.id),
    item: game.playerHelper.currentItem(props.id),
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
          <GameBoard
            class="board"
            :board="game.board.value"
            :players="game.playersMap.value"
            :active-player="game.activePlayer.value"
          />
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
