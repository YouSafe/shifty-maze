<script setup lang="ts">
import { computed } from "vue";
import type { Board, Side, SideIndex, GamePhase } from "game-core/pkg";

const props = defineProps<{
  board: Board | null;
  phase: GamePhase;
}>();

const emits = defineEmits<{
  (e: "shift-tiles", side_index: SideIndex): void;
}>();

interface SideArrow {
  id: string;
  side_index: SideIndex;
  top?: string;
  left?: string;
  right?: string;
  bottom?: string;
}

const sideArrows = computed(() => {
  const board = props.board;
  if (board === null) {
    return [];
  }
  const sideLength = board.side_length;
  return (
    [
      (index: number, percent: string) => ({
        side_index: { side: "Top" as Side, index },
        top: "0",
        left: percent,
      }),
      (index: number, percent: string) => ({
        side_index: { side: "Bottom" as Side, index },
        bottom: "0",
        left: percent,
      }),
      (index: number, percent: string) => ({
        side_index: { side: "Left" as Side, index },
        left: "0",
        top: percent,
      }),
      (index: number, percent: string) => ({
        side_index: { side: "Right" as Side, index },
        right: "0",
        top: percent,
      }),
    ] as const
  ).flatMap((position, mainIndex) => {
    const arrows: SideArrow[] = [];
    for (let i = 1; i < sideLength; i += 2) {
      arrows.push({
        id: `${mainIndex}-${i}`,
        ...position(i, (i / sideLength) * 100 + "%"),
      });
    }
    return arrows;
  });
});

function isTileArrowDisabled(side_index: SideIndex) {
  if (props.board === null) {
    return true;
  }
  let freeTile = props.board.free_tile;
  if (
    (freeTile.side_with_index ?? null) !== null &&
    freeTile.side_with_index?.side === side_index.side &&
    freeTile.side_with_index?.index === side_index.index
  ) {
    return true;
  }
  return false;
}
function startShiftTiles(side_index: SideIndex) {
  emits("shift-tiles", side_index);
}
</script>

<template>
  <div
    v-for="arrow in sideArrows"
    :key="arrow.id"
    class="arrow-wrapper"
    :class="{
      [arrow.side_index.side]: true,
      'is-active': props.phase === 'MoveTiles',
      disabled: isTileArrowDisabled(arrow.side_index),
    }"
    :style="{
      top: arrow.top,
      left: arrow.left,
      right: arrow.right,
      bottom: arrow.bottom,
    }"
    @click="() => startShiftTiles(arrow.side_index)"
  >
    <div
      class="arrow"
      :class="{
        [arrow.side_index.side]: true,
      }"
    ></div>
  </div>
</template>

<style scoped>
.arrow-wrapper {
  position: absolute;
  height: calc(var(--tile-size));
  width: calc(var(--tile-size));
  transform-origin: 50% 50%;
  pointer-events: none;
}

.arrow-wrapper.Top {
  transform: translateY(-2vmin);
}

.arrow-wrapper.Bottom {
  transform: rotate(180deg) translateY(-2vmin);
}

.arrow-wrapper.Right {
  transform: rotate(90deg) translateY(-2vmin);
}

.arrow-wrapper.Left {
  transform: rotate(-90deg) translateY(-2vmin);
}

/* TODO: Make this a disabled icon */
.arrow-wrapper.disabled {
  display: none;
}

.arrow-wrapper.is-active {
  pointer-events: auto;
}

.arrow {
  width: 100%;
  height: 2vmin;
  margin-top: 0vmin;
  background-color: #7c7c7c79;
  clip-path: polygon(50% 100%, 100% 0%, 0 0%);
}
.arrow-wrapper.is-active .arrow {
  background-color: #7c7c7c;
  animation: pulse 2s infinite;
}

.arrow-wrapper:hover .arrow {
  background-color: #4c4c4c;
}

/* animation for drop shadow */
@keyframes pulse {
  0% {
    background-color: #7c7c7c;
  }

  70% {
    background-color: #5f5f5f;
  }

  100% {
    background-color: #7c7c7c;
  }
}
</style>
