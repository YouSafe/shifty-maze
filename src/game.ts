import { computed, ref } from "vue";
import init, {
  GameCore,
  type GameStartSettings,
  type Item,
  type PlayerId,
  type Rotation,
  type SideIndex,
  type Game,
  type Result as ActionResult,
} from "game-core/pkg";
import { useLocalStorage } from "@/local-storage";
import { Result } from "@/result";

await init();

export type PlayerMode = "local" | "online";

export function DefaultGameStartSettings(): GameStartSettings {
  return {
    players: [],
    side_length: 7,
    items_per_player: 6,
  };
}

export function useGame() {
  const game = ref<Game | null>(null);
  const storage = useLocalStorage<Game>();

  function reset() {
    game.value = null;
    storage.newGame();
  }

  const core = new GameCore(10);

  storage.load().map(state => setGame(state));

  function handleResult(result: ActionResult<Game, string>) {
    Result.fromSplitResult(result).map(g => {
      game.value = g;
      saveState();
    }).mapErr(alert);
  }

  function startGame(settings: GameStartSettings) {
    reset();
    handleResult(core.start_game(settings));
  }

  function rotate_free_tile(rotation: Rotation) {
    handleResult(core.rotate_free_tile(rotation));
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
      if (!player) return 0;
      return player.to_collect.at(-1) ?? null;
    },
  };

  return {
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

    startGame,
    rotate_free_tile,
    shiftTiles,
    removePlayer,
    movePlayer,
    undoMove,
    finishGame,
  };
}
