import { computed, ref } from "vue";
import init, {
  GameCore,
  type BinaryGame,
  type Board,
  type GameStartSettings,
  type Item,
  type Player,
  type PlayerId,
  type Position,
  type Rotation,
  type Side,
  type GamePhase,
} from "../game-core/pkg";
import { useStoredUndo } from "./stored-undo";
import { Items } from "./items";

await init();

class GameCoreCallbacks {
  constructor(
    public boardCallback: (board: Board) => void,
    public playersCallback: (players: Player[]) => void,
    public phaseCallback: (phase: GamePhase) => void,
    public playersTurnCallback: (player_id: PlayerId) => void
  ) {}
  update_board(board: Board) {
    this.boardCallback(board);
  }
  update_players(players: Player[]) {
    this.playersCallback(players);
  }
  update_phase(phase: GamePhase) {
    this.phaseCallback(phase);
  }
  update_player_turn(player_id: PlayerId) {
    this.playersTurnCallback(player_id);
  }
}

export type PlayerMode = "local" | "online";

export function DefaultGameStartSettings(): GameStartSettings {
  return {
    number_of_items: Items.length,
    items_per_player: 6,
    side_length: 7,
    players: [],
  };
}

export function useGame() {
  const playersMap = ref(new Map<PlayerId, Player>());
  const board = ref<Board | null>(null);
  const activePlayer = ref<PlayerId>(0);
  const phase = ref<GamePhase>("TurnMoveTiles");
  const hasStarted = computed(() => board.value !== null);

  const storedUndo = useStoredUndo<BinaryGame>();

  const callbacks = new GameCoreCallbacks(
    (v) => {
      board.value = v;
    },
    (v) => {
      playersMap.value.clear();
      v.forEach((player) => {
        playersMap.value.set(player.id, player);
      });
      if (playersMap.value.size === 0) {
        finishGame();
      }
    },
    (v) => {
      phase.value = v;
    },
    (v) => {
      if (v === activePlayer.value) return;
      activePlayer.value = v;
      // Prevent "recursive use of an object detected which would lead to unsafe aliasing in rust"
      setTimeout(() => {
        const bytes = getGameBytes();
        if (bytes) {
          storedUndo.add(bytes);
        }
      }, 0);
    }
  );

  const game = new GameCore(callbacks);

  function startGame(settings: GameStartSettings) {
    game.start_game(settings);
  }
  function shiftTiles(side: Side, index: number, insertRotation: Rotation) {
    game.shift_tiles(side, index, insertRotation);
  }
  function addPlayer(id: PlayerId, mode: PlayerMode) {
    game.add_player(id);
  }
  function removePlayer(id: PlayerId) {
    game.remove_player(id);
  }
  function movePlayer(id: PlayerId, x: number, y: number) {
    game.move_player(id, { x, y });
  }
  function getGameBytes(): BinaryGame | null {
    return game.get_game_bytes() ?? null;
  }
  function setGameBytes(v: BinaryGame) {
    game.set_game_bytes(v);
  }
  function undo() {
    const game = storedUndo.undo();
    if (game) {
      setGameBytes(game);
    }
  }
  function finishGame() {
    board.value = null;
    storedUndo.newGame();
  }

  const playerHelper = {
    hasPlayer: (id: PlayerId) => {
      return playersMap.value.has(id);
    },
    itemCount: (id: PlayerId) => {
      const player = playersMap.value.get(id);
      if (!player) return 0;
      return player.to_collect.length;
    },
    currentItem: (id: PlayerId): Item => {
      const player = playersMap.value.get(id);
      if (!player) return 0;
      if (player.to_collect.length > 0) {
        return player.to_collect[player.to_collect.length - 1];
      } else {
        return 0;
      }
    },
  };

  return {
    hasStarted: computed(() => hasStarted.value),
    playersMap: computed(() => playersMap.value),
    playerHelper,
    activePlayer: computed(() => activePlayer.value),
    activePlayerItem: computed(() => {
      if (activePlayer.value) {
        return playerHelper.currentItem(activePlayer.value);
      } else {
        return 0;
      }
    }),
    board: computed(() => board.value),

    startGame,
    shiftTiles,
    addPlayer,
    removePlayer,
    movePlayer,
    getGameBytes,
    setGameBytes,
    undo,
    finishGame,
  };
}
