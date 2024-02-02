<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { PlayerColors } from "../players";
import { NModal, NButton, NSpace } from "naive-ui";
const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  id: number;
  playerMode: "local" | "online" | null;
}>();

const emit = defineEmits<{
  (e: "remove", id: number): void;
  (e: "join", id: number, mode: "local" | "online"): void;
}>();
const playerColor = computed(() => PlayerColors[props.id]);
const hasPlayer = computed(() => props.playerMode !== null);
const isTryingToRemove = ref(false);
watch(show, (v) => {
  if (v) {
    isTryingToRemove.value = false;
  }
});

function remove() {
  if (isTryingToRemove.value) {
    isTryingToRemove.value = false;
    emit("remove", props.id);
    close();
  } else {
    isTryingToRemove.value = true;
  }
}

function joinLocal() {
  emit("join", props.id, "local");
  close();
}
function joinOnline() {
  // TODO:
  //emit("join", props.id, "online");
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
      </n-button></n-space
    >
    <template #footer>
      <div v-if="hasPlayer">
        <n-button strong secondary round type="error" @click="remove()"
          ><p v-if="!isTryingToRemove">Remove Player</p>
          <p v-else>Are you sure?</p>
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style scoped></style>
