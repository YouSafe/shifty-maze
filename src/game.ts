import { computed, ref, watch } from "vue";
import init, {
  GameCore,
  type GameStartSettings,
  type Item,
  type PlayerId,
  type Rotation,
  type SideIndex,
  type Game,
  type Result,
  type Position,
} from "../game-core/pkg";
import { useLocalStorage } from "@/local-storage";

await init();

export function DefaultGameStartSettings(): GameStartSettings {
  return {
    players: new Map<PlayerId, undefined>(),
    side_length: 7,
    items_per_player: 6,
  };
}

type StringPosition = string;
function positionToString(p: Position): StringPosition {
  return `${p.x},${p.y}`;
}

export function useGame(errorHandler: (error: string) => void) {
  const game = ref<Game | null>(null);
  const reachable = ref<Set<StringPosition>>(new Set());
  const storage = useLocalStorage<Game>();

  function reset() {
    game.value = null;
    storage.newGame();
  }

  const core = new GameCore(10);

  storage.load().map((state) => setGame(state));

  watch(
    () => game.value,
    (game) => {
      reachable.value = new Set();
      if (game === null) return;
      if (game.phase !== "MovePlayer") return;

      const v: Result<Position[], string> = core.currently_reachable();
      if (v.type === "Ok") {
        reachable.value = new Set(v.value.map(positionToString));
      } else {
        errorHandler(v.value);
      }
    },
    { immediate: true, deep: true }
  );

  function handleResult(result: Result<Game, string>) {
    if (result.type === "Ok") {
      game.value = result.value;
      saveState();
    } else {
      errorHandler(result.value);
    }
  }

  function startGame(settings: GameStartSettings) {
    reset();
    handleResult(core.start_game(settings));
  }

  function rotateFreeTile() {
    if (game.value !== null) {
      const rotation = nextRotation(
        game.value.board.free_tile.tile.rotation ?? "OneEighty"
      );
      handleResult(core.rotate_free_tile(rotation));
    }
  }

  function shiftTiles(side_index: SideIndex) {
    handleResult(core.shift_tiles(side_index));
  }

  function removePlayer(id: PlayerId) {
    handleResult(core.remove_player(id));
  }

  function movePlayer(id: PlayerId, x: number, y: number) {
    handleResult(core.move_player(id, { x, y }));
  }

  function setGame(g: Game) {
    game.value = g;
    core.set_game(g);
  }

  function saveState() {
    if (game.value) {
      storage.save(game.value);
    }
  }

  function undoMove() {
    handleResult(core.undo_move());
  }

  function finishGame() {
    reset();
    storage.newGame();
  }

  function isReachable(position: Position) {
    if (game.value === null) return true;
    if (game.value.phase !== "MovePlayer") return true;
    return reachable.value.has(positionToString(position));
  }

  const playerHelper = {
    hasPlayer: (id: PlayerId) => {
      return game.value?.players.players.has(id) ?? false;
    },
    itemCount: (id: PlayerId) => {
      const player = game.value?.players.players.get(id);
      if (!player) return 0;
      return player.to_collect.length;
    },
    currentItem: (id: PlayerId): Item | null => {
      const player = game.value?.players.players.get(id);
      if (!player) return null;
      return player.to_collect.at(-1) ?? null;
    },
  };

  return {
    game: computed(() => game.value),
    hasStarted: computed(() => game.value !== null),
    playersMap: computed(() => game.value?.players.players ?? new Map()),
    playerHelper,
    activePlayer: computed(() => game.value?.players.player_turn ?? null),
    activePlayerItem: computed(() => {
      let turn = game.value?.players.player_turn;
      if (turn !== undefined) {
        return playerHelper.currentItem(turn);
      } else {
        return 0;
      }
    }),
    board: computed(() => game.value?.board ?? null),
    phase: computed(() => game.value?.phase ?? "MoveTiles"),
    winner: computed(() => game.value?.winner ?? null),

    setGame,
    startGame,
    rotateFreeTile,
    shiftTiles,
    removePlayer,
    movePlayer,
    undoMove,
    finishGame,
    isReachable,
  };
}

function nextRotation(rotation: Rotation): Rotation {
  if (rotation === "Zero") return "Ninety";
  if (rotation === "Ninety") return "OneEighty";
  if (rotation === "OneEighty") return "TwoSeventy";
  return "Zero";
}
