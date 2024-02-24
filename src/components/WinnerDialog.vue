<script setup lang="ts">
import { computed } from "vue";
import { NModal, NButton } from "naive-ui";
import { PlayerColors } from "@/players";

const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  id: number;
}>();
const emit = defineEmits<{
  (e: "new-game"): void;
}>();

function newGame() {
  emit("new-game");
  show.value = false;
}

const playerColor = computed(() => PlayerColors[props.id]);
</script>

<template open>
  <n-modal
    :show="show"
    @update:show="(v) => (show = v)"
    preset="card"
    :title="`Player ${props.id} has won!`"
    :bordered="false"
    size="huge"
    :style="{
      width: '80%',
      maxWidth: '600px',
      border: `2px solid ${playerColor}`,
    }"
  >
    Congratulations! You have won the game!
    <br />
    <br />
    <n-button strong secondary round type="primary" @click="newGame()">
      New Game?
    </n-button>
  </n-modal>
</template>

<style scoped></style>
