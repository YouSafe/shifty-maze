import { computed, ref } from "vue";
import init, {
  type Board,
  type Item,
  type Player,
  type PlayerId,
} from "../game-core/pkg";
import { useStoredUndo } from "./stored-undo";

await init();

class GameCoreCallbacks {
  constructor(
    public boardCallback: (board: Board) => void,
    public playersCallback: (players: Player[]) => void,
    public playersTurnCallback: (player_id: PlayerId) => void
  ) {}
  update_board(board: Board) {
    this.boardCallback(board);
  }
  update_players(players: Player[]) {
    this.playersCallback(players);
  }
  update_player_turn(player_id: PlayerId) {
    this.playersTurnCallback(player_id);
  }
}

export type PlayerMode = "local" | "online";

export interface GameStartSettings {
  board_side_length: number;
  cards_per_player: number;
}

export function useGame() {
  const hasStarted = ref(false);
  const playersMap = ref(new Map<PlayerId, Player>());
  const board = ref<Board | null>(null);
  const activePlayer = ref<PlayerId | null>(2);

  //const storedUndo = useStoredUndo<BinaryBoard>();

  const x = createDummyGame();
  board.value = x.board;
  playersMap.value = new Map(x.players.map((p) => [p.id, p]));

  const callbacks = new GameCoreCallbacks(
    (v) => {
      board.value = v;
    },
    (v) => {
      playersMap.value.clear();
      v.forEach((player) => {
        playersMap.value.set(player.id, player);
      });
    },
    (v) => {
      activePlayer.value = v;
    }
  );

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
  };
}

function createDummyGame(): {
  board: Board;
  players: Player[];
} {
  const size = 3;
  const items = 15;

  function createDummyPlayer(id: number): Player {
    const position = {
      x: Math.floor(Math.random() * size),
      y: Math.floor(Math.random() * size),
    };
    const start_position = {
      x: Math.floor(Math.random() * size),
      y: Math.floor(Math.random() * size),
    };

    const collected = new Array(Math.floor(Math.random() * items))
      .fill(0)
      .map(() => Math.floor(Math.random() * items) + 1);
    const to_collect = new Array(Math.floor(Math.random() * items))
      .fill(0)
      .map(() => Math.floor(Math.random() * items) + 1);

    return {
      id,
      position,
      start_position,
      collected,
      to_collect,
    };
  }

  const players = new Array(Math.floor(Math.random() * 4) + 4)
    .fill(0)
    .map((_, i) => createDummyPlayer(i));

  const tiles = new Array(size * size + 1).fill(0).map((_, i) => {
    const variant = (["LShape", "TShape", "IShape"] as const)[
      Math.floor(Math.random() * 3)
    ];
    const rotation = (["Zero", "Ninety", "OneEighty", "TwoSeventy"] as const)[
      Math.floor(Math.random() * 4)
    ];
    const item =
      Math.random() < 0.3 ? Math.floor(Math.random() * items + 1) : 0;
    return {
      id: i,
      variant,
      rotation,
      item,
    };
  });

  const free_tile = tiles.pop()!;
  const side_with_index:
    | ["Top" | "Right" | "Bottom" | "Left", number]
    | undefined =
    Math.random() < 0.3
      ? undefined
      : [
          (["Top", "Right", "Bottom", "Left"] as const)[
            Math.floor(Math.random() * 4)
          ],
          Math.floor(Math.random() * size),
        ];

  return {
    board: {
      tiles,
      side_length: size,
      free_tile: {
        tile: free_tile,
        side_with_index,
      },
    },
    players,
  };
}
