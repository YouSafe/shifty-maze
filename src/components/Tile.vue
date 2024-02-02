<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  tile: {
    variant: any;
    rotation: any;
    item: any;
  };
}>();
</script>

<template>
  <div class="max-size">put tile here</div>
</template>

<style scoped></style>
