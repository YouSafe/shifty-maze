<script setup lang="ts">
import { h, ref } from "vue";
import GameBoard from "./components/GameBoard.vue";
import PlayerCards from "./components/PlayerCards.vue";
import PlayerDialog from "./components/PlayerDialog.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import GameTile from "./components/GameTile.vue";
import { useGame, type PlayerMode, DefaultGameStartSettings } from "./game";
import { NButton } from "naive-ui";
import { PlayerSides } from "./players";

const showPlayerDialog = ref(false);
const showDialogFor = ref({
  id: 0,
  mode: "local" as PlayerMode | null,
});

const showSettingsDialog = ref(false);

const gameSettings = ref(DefaultGameStartSettings());
const game = useGame();

function addPlayer(id: number, mode: PlayerMode) {
  if (game.hasStarted.value === false) {
    if (gameSettings.value.players.includes(id)) {
      return;
    }
    gameSettings.value.players.push(id);
    // TODO: Deal with online players
  }
}

function removePlayer(id: number) {
  if (game.hasStarted.value === false) {
    const index = gameSettings.value.players.indexOf(id);
    if (index !== -1) {
      gameSettings.value.players.splice(index, 1);
    }
  } else {
    game.removePlayer(id);
  }
}

// See also https://vuejs.org/guide/extras/render-function#typing-functional-components
function OnePlayerCard(props: { id: number }) {
  return h(PlayerCards, {
    side: Object.keys(PlayerSides[props.id])[0] ?? "Bottom",
    id: props.id,
    isActive: game.activePlayer.value === props.id,
    hasPlayer: game.playerHelper.hasPlayer(props.id),
    count: game.playerHelper.itemCount(props.id),
    item: game.playerHelper.currentItem(props.id),
    onClick: () => {
      if (game.hasStarted.value === false) {
        showDialogFor.value = { id: props.id, mode: null };
      } else {
        showDialogFor.value = { id: props.id, mode: "local" };
      }
      showPlayerDialog.value = true;
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
            :phase="game.phase.value"
            v-model:start-settings="gameSettings"
            @start-game="(v) => game.startGame(v)"
            @player-move="(player, x, y) => game.movePlayer(player, x, y)"
            @shift-tiles="(side_index, rotation) => game.shiftTiles(side_index)"
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
          round
          size="small"
          class="settings-button-small"
          @click="showSettingsDialog = true"
        >
          ⚙️</NButton
        >
        <NButton
          round
          class="settings-button-large"
          @click="showSettingsDialog = true"
        >
          Settings</NButton
        >
      </div>
    </div>
    <PlayerDialog
      v-model:show="showPlayerDialog"
      :id="showDialogFor.id"
      :player-mode="showDialogFor.mode"
      @join="(id, mode) => addPlayer(id, mode)"
      @remove="(v) => removePlayer(v)"
    ></PlayerDialog>
    <SettingsDialog
      v-model:show="showSettingsDialog"
      :has-game-started="game.hasStarted.value"
      @quit-game="game.finishGame()"
    ></SettingsDialog>
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
  min-height: 0;
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

.settings-button-small,
.settings-button-large {
  position: absolute;
  top: 10px;
  right: 5px;
}

.settings-button-small {
  display: block;
}

.settings-button-large {
  display: none;
}

@media (min-width: 800px) {
  .settings-button-small {
    display: none;
  }

  .settings-button-large {
    display: block;
  }
}
</style>
