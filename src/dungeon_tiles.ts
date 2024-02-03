import TileMapImageUrl from "./assets/dungeon_tiles.png?url";
const TileMapImage = new Image();
await new Promise((resolve) => {
  TileMapImage.onload = resolve;
  TileMapImage.src = TileMapImageUrl;
});
const canvas = document.createElement("canvas");
const makeCtx = (width: number, height: number) => {
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("Could not create 2d context");
  }
  ctx.imageSmoothingEnabled = false;
  return ctx;
};

export const DungeonTiles = {
  Width: 368,
  Height: 384,
  Players: [
    exactTile(16 * 16, 21 * 16, 21, 19),
    exactTile(20 * 16, 17 * 16, 17, 20),
    exactTile(16 * 16, 19 * 16, 21, 19),
    exactTile(20 * 16, 19 * 16, 17, 20),
    //
    exactTile(18 * 16, 21 * 16, 21, 19),
    exactTile(20 * 16, 21 * 16, 17, 20),
    exactTile(18 * 16, 17 * 16, 17, 20),
    exactTile(18 * 16, 19 * 16, 21, 19),
  ],
  Abyss: makeAbyssTiles(),
  Empty: exactTile(0, 0, 16, 16),
};

export interface TileImage {
  img: string;
  width: number;
  height: number;
}

function exactTile(
  x: number,
  y: number,
  width: number,
  height: number
): TileImage {
  const ctx = makeCtx(width, height);
  ctx.drawImage(TileMapImage, x, y, width, height, 0, 0, width, height);
  return {
    img: canvas.toDataURL(),
    width,
    height,
  };
}

function assembleTile(
  pieces: ({
    x: number;
    y: number;
    rotation?: 0 | 90 | 180 | 270;
  } | null)[]
) {
  if (pieces.length !== 9) {
    throw new Error("Invalid number of pieces");
  }
  const ctx = makeCtx(16 * 3, 16 * 3);
  pieces.forEach((piece, i) => {
    if (piece) {
      ctx.save();
      const x = (i % 3) * 16;
      const y = Math.floor(i / 3) * 16;
      ctx.translate(x, y);
      if (piece.rotation) {
        ctx.translate(8, 8);
        ctx.rotate((piece.rotation * Math.PI) / 180);
        ctx.translate(-8, -8);
      }
      ctx.drawImage(
        TileMapImage,
        piece.x * 16,
        piece.y * 16,
        16,
        16,
        0,
        0,
        16,
        16
      );
      ctx.restore();
    }
  });

  return {
    img: canvas.toDataURL(),
    width: 16 * 3,
    height: 16 * 3,
  };
}

function makeAbyssTiles() {
  let straight = {
    x: 8,
    y: 5,
  };
  let cliff = {
    x: 8,
    y: 6,
  };
  let corner = {
    x: 9,
    y: 5,
  };
  let tJunction = {
    x: 13,
    y: 15,
  };

  return {
    LShape: assembleTile([
      null,
      null,
      null,
      straight,
      corner,
      null,
      null,
      {
        ...straight,
        rotation: 90,
      },
      null,
    ]),
    TShape: assembleTile([
      null,
      null,
      null,
      straight,
      tJunction,
      straight,
      null,
      {
        ...straight,
        rotation: 90,
      },
      null,
    ]),
    IShape: assembleTile([
      null,
      null,
      null,
      straight,
      straight,
      straight,
      null,
      null,
      null,
    ]),
  };
}
