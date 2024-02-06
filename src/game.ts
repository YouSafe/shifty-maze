import { computed, ref } from "vue";
import init, {
  GameCore,
  type Board,
  type GameStartSettings,
  type Item,
  type Player,
  type PlayerId,
  type Rotation,
  type SideIndex,
  type GamePhase,
  type Players,
} from "../game-core/pkg";

await init();

class GameCoreCallbacks {
  constructor(
    public boardCallback: (board: Board) => void,
    public playersCallback: (players: Players) => void,
    public phaseCallback: (phase: GamePhase) => void,
  ) { }

  update_board(board: Board) {
    this.boardCallback(board);
  }

  update_players(players: Players) {
    this.playersCallback(players);
  }

  update_phase(phase: GamePhase) {
    this.phaseCallback(phase);
  }
}

export type PlayerMode = "local" | "online";

export function DefaultGameStartSettings(): GameStartSettings {
  return {
    items_per_player: 6,
    side_length: 7,
    players: [],
  };
}

export function useGame() {
  const players = ref<Players>({ players: new Map<PlayerId, Player>(), player_turn: 0 });
  const board = ref<Board | null>(null);
  const phase = ref<GamePhase>("MoveTiles");
  const hasStarted = computed(() => board.value !== null);

  function reset() {
    players.value.players.clear();
    players.value.player_turn = 0;
    board.value = null;
    phase.value = "MoveTiles";
  }

  const callbacks = new GameCoreCallbacks(
    (v) => {
      board.value = v;
    },
    (v) => {
      players.value = v;
    },
    (v) => {
      phase.value = v;
    }
  );

  const game = new GameCore(callbacks);

  function startGame(settings: GameStartSettings) {
    reset();
    game.start_game(settings);
  }

  function rotate_free_tile(rotation: Rotation) {
    game.rotate_free_tile(rotation);
  }

  function shiftTiles(side_index: SideIndex) {
    game.shift_tiles(side_index);
  }

  function removePlayer(id: PlayerId) {
    game.remove_player(id);
  }

  function movePlayer(id: PlayerId, x: number, y: number) {
    game.move_player(id, { x, y });
  }

  function finishGame() {
    reset();
  }

  const playerHelper = {
    hasPlayer: (id: PlayerId) => {
      return players.value.players.has(id);
    },
    itemCount: (id: PlayerId) => {
      const player = players.value.players.get(id);
      if (!player) return 0;
      return player.to_collect.length;
    },
    currentItem: (id: PlayerId): Item | null => {
      const player = players.value.players.get(id);
      if (!player) return 0;
      return player.to_collect.at(-1) ?? null;
    },
  };

  return {
    hasStarted: computed(() => hasStarted.value),
    playersMap: computed(() => players.value.players),
    playerHelper,
    activePlayer: computed(() => players.value.player_turn),
    activePlayerItem: computed(() => {
      if (players.value.player_turn) {
        return playerHelper.currentItem(players.value.player_turn);
      } else {
        return 0;
      }
    }),
    board: computed(() => board.value),
    phase: computed(() => phase.value),

    startGame,
    rotate_free_tile,
    shiftTiles,
    removePlayer,
    movePlayer,
    finishGame,
  };
}
