<script setup lang="ts">
import { getServerUrl } from "@/multiplayer-url";
import { Message } from "@/notification";
import { PlayerColors } from "@/players";
import { NModal, NQrCode } from "naive-ui";
import { computed } from "vue";

const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  playerId: number;
}>();
const playerColor = computed(() => PlayerColors[props.playerId]);
const serverUrl = computed(() => getServerUrl(props.playerId));

function copyUrl() {
  navigator.clipboard.writeText(serverUrl.value);
  Message.success("Copied to clipboard");
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
    :title="`Player ${props.playerId} - Join Game`"
    :bordered="false"
    size="huge"
    :style="{
      width: '80%',
      maxWidth: '600px',
      border: `2px solid ${playerColor}`,
    }"
  >
    <n-qr-code :value="serverUrl" :size="300" level="Q" />
    <br />
    <span class="link" @click="copyUrl">{{ serverUrl }}</span>
  </n-modal>
</template>

<style scoped>
.link {
  user-select: all;
}
</style>
