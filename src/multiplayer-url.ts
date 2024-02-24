import { urlRef } from "@/url-ref";

export const ServerUrlRef = urlRef("server", "");
export const PlayerIdRef = urlRef("player", -1);

export function isClient() {
  return PlayerIdRef.value >= 0;
}

export function quitClient() {
  PlayerIdRef.value = -1;
}

export function getServerUrl(playerId: number) {
  const url = new URL(window.location.href);
  url.searchParams.set("server", ServerUrlRef.value);
  url.searchParams.set("player", playerId + "");
  return url.toString();
}
