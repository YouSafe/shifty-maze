<script setup lang="ts">
import { h, ref } from "vue";
import GameBoard from "./components/GameBoard.vue";
import PlayerCards from "./components/PlayerCards.vue";
import PlayerDialog from "./components/PlayerDialog.vue";
import GameTile from "./components/GameTile.vue";
import { useGame, type PlayerMode } from "./game";
import type { Player, Side } from "game-core/pkg/wasm";
import { NButton } from "naive-ui";

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
      <div class="constrain-height container">
        <div class="top space-between">
          <OnePlayerCard :id="4"></OnePlayerCard>
          <OnePlayerCard :id="5"></OnePlayerCard>
        </div>
        <div class="middle">
          <div class="left space-between">
            <OnePlayerCard :id="3"></OnePlayerCard>
            <OnePlayerCard :id="2"></OnePlayerCard>
          </div>
          <GameBoard
            :board="game.hasStarted ? game.board.value : null"
            :players="game.playersMap.value"
            :active-player="game.activePlayer.value"
            :active-player-item="game.activePlayerItem.value"
            @start-game="(v) => game.updateGameSettings(v)"
            @player-move="(player, x, y) => game.movePlayer(player, x, y)"
          />
          <div class="right space-between">
            <OnePlayerCard :id="6"></OnePlayerCard>
            <OnePlayerCard :id="7"></OnePlayerCard>
          </div>
        </div>
        <div class="bottom space-between">
          <OnePlayerCard :id="1"></OnePlayerCard>
          <OnePlayerCard :id="0"></OnePlayerCard>
        </div>
        <GameTile
          v-if="game.board.value?.free_tile"
          :tile="game.board.value?.free_tile?.tile ?? null"
          :searching-for="game.activePlayerItem.value"
          class="free-tile"
        ></GameTile>
        <NButton
          :disabled="!game.hasStarted"
          round
          size="small"
          class="undo-button-small"
        >
          ⟲</NButton
        >
        <NButton :disabled="!game.hasStarted" round class="undo-button-large">
          ⟲ Undo</NButton
        >
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
.top,
.bottom {
  display: flex;
  padding: 0 calc(88vmin * var(--card-scale) + 15px);
  flex-direction: row;
}
.left,
.right {
  display: flex;
  flex: 1 0 auto;
  flex-direction: column;
}
.middle {
  display: flex;
  flex-direction: row;
}
.space-between {
  justify-content: space-between;
}
.container {
  position: relative;
  flex-direction: column;
}
.free-tile {
  position: absolute;
  top: calc(88vmin * var(--card-scale) * 0.5);
  left: calc(88vmin * var(--card-scale) * 0.5);
  width: calc(70vmin * var(--card-scale));
  height: calc(70vmin * var(--card-scale));
  transform: translate(-50%, -50%);
}
.undo-button-small,
.undo-button-large {
  position: absolute;
  top: 10px;
  right: 5px;
}
.undo-button-small {
  display: block;
}
.undo-button-large {
  display: none;
}
@media (min-width: 800px) {
  .undo-button-small {
    display: none;
  }
  .undo-button-large {
    display: block;
  }
}
</style>
