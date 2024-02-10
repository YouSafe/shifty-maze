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
  type Game,
} from "../game-core/pkg";
import { useLocalStorage } from "./local-storage";

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
  const storage = useLocalStorage<Game>();

  function reset() {
    players.value.players.clear();
    players.value.player_turn = 0;
    board.value = null;
    phase.value = "MoveTiles";
    storage.newGame();
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

  const game = new GameCore(callbacks, 10);

  {
    const state = storage.load();
    if (state) {
      setGame(state);
    }
  }

  function startGame(settings: GameStartSettings) {
    reset();
    game.start_game(settings);
  }

  function rotate_free_tile(rotation: Rotation) {
    game.rotate_free_tile(rotation);
  }

  function shiftTiles(side_index: SideIndex) {
    game.shift_tiles(side_index);
    saveState();
  }

  function removePlayer(id: PlayerId) {
    game.remove_player(id);
    saveState();
  }

  function movePlayer(id: PlayerId, x: number, y: number) {
    game.move_player(id, { x, y });
    saveState();
  }

  function getCurrentGame(): Game | null {
    return game.get_current_game() ?? null;
  }

  function setGame(g: Game) {
    game.set_game(g);
  }

  function saveState() {
    let current = getCurrentGame();
    if (current) {
      storage.save(current);
    }
  }

  function undo() {
    game.undo_move();
    saveState();
  }

  function finishGame() {
    reset();
    storage.newGame();
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
