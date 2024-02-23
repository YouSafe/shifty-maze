<script setup lang="ts">
import { h, ref, watch } from "vue";
import SquareContainer from "@/components/SquareContainer.vue";
import GameBoard from "@/components/GameBoard.vue";
import PlayerCards from "@/components/PlayerCards.vue";
import PlayerDialog from "@/components/PlayerDialog.vue";
import SettingsDialog from "@/components/SettingsDialog.vue";
import WinnerDialog from "@/components/WinnerDialog.vue";
import PlayerJoinDialog from "@/components/PlayerJoinDialog.vue";
import GameTile from "@/components/GameTile.vue";
import { useGame, DefaultGameStartSettings } from "@/game";
import { NButton } from "naive-ui";
import { PlayerSides } from "@/players";
import { useClientGame, type PlayerMode, useServer } from "./multiplayer";
import {
  PlayerIdRef,
  isClient,
  quitClient as disconnectClient,
  ServerUrlRef,
} from "./multiplayer-url";
import type { PlayerId } from "game-core/pkg/wasm";

const showPlayerDialog = ref(false);
const showDialogFor = ref(0);

const showSettingsDialog = ref(false);
const showWinnerDialog = ref(false);
const showPlayerJoinDialog = ref(false);

const gameSettings = ref(DefaultGameStartSettings());
const game = isClient()
  ? useClientGame(ServerUrlRef.value, PlayerIdRef.value, () => {
      disconnectClient();
      window.location.reload();
    })
  : useGame();

const server = isClient() ? null : useServer(ServerUrlRef, game);

watch(game.winner, (v) => {
  showWinnerDialog.value = v !== null;
});

function join(id: number, mode: PlayerMode) {
  if (game.hasStarted.value === false) {
    if (gameSettings.value.players.includes(id)) {
      return;
    }
    gameSettings.value.players.push(id);
    if (mode === "online") {
      server?.startServer();
      showPlayerJoinDialog.value = true;
    } else {
      // Nothing
    }
  } else if (!game.playerHelper.hasPlayer(id)) {
    alert("Cannot add player to game that has already started.");
    return;
  } else {
    // We're switching player mode
    if (mode === "online") {
      if (server !== null) {
        server?.startServer();
        showPlayerJoinDialog.value = true;
      } else {
        alert("Cannot switch to online mode in client mode.");
      }
    } else {
      game.removePlayer(id);
    }
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

function getPlayerMode(id: PlayerId): PlayerMode | null {
  if (server !== null && server.isOnlinePlayer(id)) {
    return "online";
  }
  if (game.playerHelper.hasPlayer(id)) {
    return "local";
  }
  return null;
}

// See also https://vuejs.org/guide/extras/render-function#typing-functional-components
function OnePlayerCard(props: { id: number }) {
  return h(PlayerCards, {
    side: PlayerSides[props.id] ?? "Top",
    id: props.id,
    isActive: game.activePlayer.value === props.id,
    hasPlayer: game.playerHelper.hasPlayer(props.id),
    count: game.playerHelper.itemCount(props.id),
    item: game.playerHelper.currentItem(props.id),
    onClick: () => {
      if (isClient()) {
        showDialogFor.value = PlayerIdRef.value;
      } else {
        showDialogFor.value = props.id;
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
    <SquareContainer>
      <div class="container">
        <div class="top space-between">
          <OnePlayerCard :id="1"></OnePlayerCard>
          <OnePlayerCard :id="2"></OnePlayerCard>
        </div>
        <div class="middle">
          <div class="left space-between">
            <OnePlayerCard :id="0"></OnePlayerCard>
            <OnePlayerCard :id="7"></OnePlayerCard>
          </div>
          <GameBoard
            :board="game.board.value"
            :players="game.playersMap.value"
            :active-player="game.activePlayer.value"
            :active-player-item="game.activePlayerItem.value"
            :phase="game.phase.value"
            v-model:start-settings="gameSettings"
            @start-game="(v) => game.startGame(v)"
            @player-move="(player, x, y) => game.movePlayer(player, x, y)"
            @shift-tiles="(side_index) => game.shiftTiles(side_index)"
          />
          <div class="right space-between">
            <OnePlayerCard :id="3"></OnePlayerCard>
            <OnePlayerCard :id="4"></OnePlayerCard>
          </div>
        </div>
        <div class="bottom space-between">
          <OnePlayerCard :id="6"></OnePlayerCard>
          <OnePlayerCard :id="5"></OnePlayerCard>
        </div>
        <div
          v-if="game.board.value?.free_tile"
          class="free-tile"
          @click="game.rotateFreeTile()"
        >
          <GameTile
            :tile="game.board.value?.free_tile?.tile ?? null"
            :searching-for="game.activePlayerItem.value"
          ></GameTile>
          <div v-if="game.phase.value === 'MoveTiles'">Rotate&nbsp;⟳</div>
        </div>
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
    </SquareContainer>
    <PlayerDialog
      v-model:show="showPlayerDialog"
      :id="showDialogFor"
      :player-mode="getPlayerMode(showDialogFor)"
      @join="(id, mode) => join(id, mode)"
      @remove="(v) => removePlayer(v)"
    ></PlayerDialog>
    <PlayerJoinDialog
      v-model:show="showPlayerJoinDialog"
      :player-id="showDialogFor"
    ></PlayerJoinDialog>
    <WinnerDialog
      v-if="game.winner.value !== null"
      :id="game.winner.value"
      v-model:show="showWinnerDialog"
      @new-game="game.finishGame()"
    ></WinnerDialog>
    <SettingsDialog
      v-model:show="showSettingsDialog"
      :has-game-started="game.hasStarted.value"
      @quit-game="game.finishGame()"
      @undo="game.undoMove()"
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
