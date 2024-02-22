<script setup lang="ts">
import { getServerUrl } from "@/multiplayer-url";
import { PlayerColors } from "@/players";
import { NModal, NButton, NSpace } from "naive-ui";
import QrcodeVue from "qrcode.vue";
import { computed } from "vue";
const show = defineModel("show", { type: Boolean, required: true });
const props = defineProps<{
  playerId: number;
}>();
const playerColor = computed(() => PlayerColors[props.playerId]);
const serverUrl = computed(() => getServerUrl(props.playerId));
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
    <qrcode-vue :value="serverUrl" :size="300" level="H" />
    <br />
    <span class="link">{{ serverUrl }}</span>
  </n-modal>
</template>

<style scoped>
.link {
  user-select: all;
}
</style>
