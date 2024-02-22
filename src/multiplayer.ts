import Peer, { type DataConnection } from "peerjs";
import { useGame } from "./game";
import type {
  Game,
  GameStartSettings,
  PlayerId,
  Result,
  SideIndex,
} from "game-core/pkg/wasm";
import { ref, type ComputedRef, watch, type Ref } from "vue";
import { JsonSerializer } from "json-safe-stringify";

const serializer = new JsonSerializer();
export type PlayerMode = "local" | "online";

interface OnlinePlayer {
  connection: DataConnection;
}

interface ServerGame {
  game: ComputedRef<Game | null>;
  rotateFreeTile: () => void;
  shiftTiles: (side_index: SideIndex) => void;
  movePlayer: (id: PlayerId, x: number, y: number) => void;
}

const onlinePlayers = ref(new Map<PlayerId, OnlinePlayer>());
export function useServer(id: Ref<string>, game: ServerGame) {
  let peer: Peer | null = null;

  function isOnlinePlayer(id: PlayerId): boolean {
    return onlinePlayers.value.has(id);
  }

  function startServer() {
    if (peer !== null) {
      return;
    }
    id.value = crypto.randomUUID();

    peer = new Peer(id.value, {
      debug: 2,
    });
    peer.on("open", (id) => {
      console.log("Server open", id);
    });
    peer.on("error", (err) => {
      console.error("Peer error", err);
    });
    peer.on("close", () => {
      console.log("Server close");
    });
    peer.on("connection", (connection) => {
      console.log("Connection", connection);
      onlinePlayers.value.set(connection.metadata as PlayerId, {
        connection,
      });
      connection.on("close", () => {
        onlinePlayers.value.delete(connection.metadata as PlayerId);
      });
      connection.on("error", (err) => {
        console.error("Connection error", err);
      });
      connection.on("data", (v) => {
        const data: RPCFunction = serializer.parse(v as any);
        if (data.name === "rotateFreeTile") {
          game.rotateFreeTile();
        } else if (data.name === "shiftTiles") {
          game.shiftTiles(data.side_index);
        } else if (data.name === "movePlayer") {
          game.movePlayer(data.id, data.x, data.y);
        } else if (data.name === "requestGame") {
          updateGame();
        } else {
          console.error("Unknown RPC", data);
        }
      });
    });

    watch(
      game.game,
      (v) => {
        updateGame();
      },
      {
        deep: true,
      }
    );
  }

  if (id.value !== "") {
    startServer();
  }

  function updateGame() {
    const g = game.game.value;
    if (g === null) {
      return;
    }
    for (const player of onlinePlayers.value.values()) {
      player.connection.send(
        serializer.stringify({ type: "Ok", value: g } as Result<Game, string>)
      );
    }
  }

  return {
    isOnlinePlayer,
    startServer,
  };
}

type RPCFunction =
  | {
      name: "rotateFreeTile";
    }
  | {
      name: "shiftTiles";
      side_index: SideIndex;
    }
  | {
      name: "movePlayer";
      id: PlayerId;
      x: number;
      y: number;
    }
  | {
      name: "requestGame";
    };

function useLocalChangeTracker(requestGame: () => void) {
  let localChangeTimeout: number | null = null;
  function clearHasLocalChange() {
    if (localChangeTimeout !== null) {
      globalThis.clearTimeout(localChangeTimeout);
    }
  }
  function setHasLocalChange() {
    localChangeTimeout = globalThis.setTimeout(() => {
      requestGame();
    }, 1000);
  }

  return {
    clearHasLocalChange,
    setHasLocalChange,
  };
}

export function useClientGame(
  serverId: string,
  playerId: number,
  disconnect: () => void
): ReturnType<typeof useGame> {
  const game = useGame();

  const peer = new Peer();
  peer.on("error", (err) => {
    alert(err);
    console.error("Peer error", err);
  });
  let connection: DataConnection | null = null;

  peer.on("open", () => {
    connection = peer.connect(serverId, {
      metadata: playerId,
      reliable: true,
    });
    connection.on("open", () => {
      connection!.on("data", (v) => {
        console.log("Data", v);
        const data: Result<Game, string> = serializer.parse(v as any);
        if (data.type === "Ok") {
          game.setGame(data.value);
          clearHasLocalChange();
        } else {
          alert(data.value);
          console.error("Server error", data.value);
        }
      });
      send({
        name: "requestGame",
      });
      console.log("Connection open");
    });
    connection.on("close", () => {
      alert("Connection closed");
    });
    connection.on("error", (err) => {
      alert(err);
      console.error("Connection error", err);
    });
  });

  const { clearHasLocalChange, setHasLocalChange } = useLocalChangeTracker(
    () => {
      send({
        name: "requestGame",
      });
    }
  );

  function send(data: RPCFunction) {
    connection!.send(serializer.stringify(data));
  }

  function setGame(game: Game) {
    alert("Client cannot set game");
  }
  function startGame(settings: GameStartSettings) {
    alert("Client cannot start game");
  }
  function rotateFreeTile() {
    if (game.activePlayer.value !== playerId) {
      return;
    }
    game.rotateFreeTile();
    setHasLocalChange();
    send({
      name: "rotateFreeTile",
    });
  }
  function shiftTiles(side_index: SideIndex) {
    if (game.activePlayer.value !== playerId) {
      return;
    }
    game.shiftTiles(side_index);
    setHasLocalChange();
    send({
      name: "shiftTiles",
      side_index,
    });
  }
  function removePlayer(id: PlayerId) {
    if (id === playerId) {
      disconnect();
    } else {
      alert("Client cannot remove player");
    }
  }
  function movePlayer(id: PlayerId, x: number, y: number) {
    if (game.activePlayer.value !== playerId) {
      return;
    }
    if (id !== playerId) {
      return;
    }
    game.movePlayer(id, x, y);
    setHasLocalChange();
    send({
      name: "movePlayer",
      id,
      x,
      y,
    });
  }
  function undoMove() {
    alert("Client cannot undo move");
  }
  function finishGame() {
    disconnect();
  }

  return {
    game: game.game,
    hasStarted: game.hasStarted,
    playersMap: game.playersMap,
    playerHelper: game.playerHelper,
    activePlayer: game.activePlayer,
    activePlayerItem: game.activePlayerItem,
    board: game.board,
    phase: game.phase,
    winner: game.winner,
    setGame,
    startGame,
    rotateFreeTile,
    shiftTiles,
    removePlayer,
    movePlayer,
    undoMove,
    finishGame,
  };
}
