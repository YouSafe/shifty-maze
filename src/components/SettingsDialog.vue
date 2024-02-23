<script setup lang="ts">
import { ref, watch } from "vue";
import { NModal, NButton, NPopconfirm } from "naive-ui";
const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  hasGameStarted: boolean;
}>();
const emit = defineEmits<{
  (e: "undo"): void;
  (e: "quit-game"): void;
}>();

function undo() {
  emit("undo");
}

function quitGame() {
  emit("quit-game");
  show.value = false;
}
</script>

<template open>
  <n-modal
    :show="show"
    @update:show="
      (v) => {
        show = v;
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
        <n-popconfirm @positive-click="quitGame()">
          <template #trigger>
            <n-button strong secondary round type="error"> Quit Game </n-button>
          </template>
          Are you sure?
        </n-popconfirm>
      </div>
    </template>
  </n-modal>
</template>

<style scoped></style>
