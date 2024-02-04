<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { PlayerColors } from "../players";
import { NModal, NButton, NSpace } from "naive-ui";
import type { PlayerMode } from "@/game";
const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  hasGameStarted: boolean;
}>();
const emit = defineEmits<{
  (e: "undo"): void;
  (e: "quit-game"): void;
}>();
const isTryingToQuit = ref(false);
watch(show, (v) => {
  if (v) {
    isTryingToQuit.value = false;
  }
});
function undo() {
  emit("undo");
}

function quitGame() {
  if (isTryingToQuit.value) {
    isTryingToQuit.value = false;
    emit("quit-game");
    close();
  } else {
    isTryingToQuit.value = true;
  }
}

function close() {
  show.value = false;
}
</script>

<template open>
  <n-modal
    :show="show"
    @update:show="
      (v) => {
        if (!v) close();
        else show = v;
      }
    "
    preset="card"
    title="Game Settings"
    :bordered="false"
    size="huge"
    :segmented="{
      content: 'soft',
      footer: 'soft',
    }"
    :style="{
      width: '80%',
      maxWidth: '600px',
    }"
  >
    <p>Undo move</p>
    <NButton :disabled="!props.hasGameStarted" round @click="undo()">
      ⟲ Undo
    </NButton>
    <template #footer>
      <div v-if="hasGameStarted">
        <n-button strong secondary round type="error" @click="quitGame()"
          ><p v-if="!isTryingToQuit">Quit Game</p>
          <p v-else>Are you sure?</p>
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style scoped></style>
