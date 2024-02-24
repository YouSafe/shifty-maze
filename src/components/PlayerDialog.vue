<script setup lang="ts">
import { computed } from "vue";
import { PlayerColors } from "@/players";
import { NModal, NButton, NSpace, NPopconfirm } from "naive-ui";
import type { PlayerMode } from "@/multiplayer";

const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  id: number;
  playerMode: PlayerMode | null;
}>();

const emit = defineEmits<{
  (e: "remove", id: number): void;
  (e: "join", id: number, mode: PlayerMode): void;
}>();
const playerColor = computed(() => PlayerColors[props.id]);
const hasPlayer = computed(() => props.playerMode !== null);

function remove() {
  emit("remove", props.id);
  show.value = false;
}

function joinLocal() {
  emit("join", props.id, "local");
  show.value = false;
}
function joinOnline() {
  emit("join", props.id, "online");
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
    :title="`Player ${props.id} - ${props.playerMode ?? 'Choose a mode'}`"
    :bordered="false"
    size="huge"
    :segmented="{
      content: 'soft',
      footer: 'soft',
    }"
    :style="{
      width: '80%',
      maxWidth: '600px',
      border: `2px solid ${playerColor}`,
    }"
  >
    <p v-if="!hasPlayer">Choose a player mode</p>
    <p v-else>Switch to</p>
    <n-space>
      <n-button
        strong
        round
        type="primary"
        :disabled="props.playerMode === 'local'"
        @click="joinLocal"
      >
        Local
      </n-button>
      <n-button
        strong
        round
        :disabled="props.playerMode === 'online'"
        @click="joinOnline"
      >
        Online
      </n-button>
    </n-space>
    <template #footer>
      <div v-if="hasPlayer">
        <n-popconfirm @positive-click="remove()">
          <template #trigger>
            <n-button strong secondary round type="error">
              Remove Player
            </n-button>
          </template>
          Are you sure?
        </n-popconfirm>
      </div>
    </template>
  </n-modal>
</template>

<style scoped></style>
