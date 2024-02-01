<script setup lang="ts"></script>

<template>
  <div class="max-size" ref="containerElement">
    <div class="constrain-width">
      <div class="constrain-height">a</div>
    </div>
  </div>
</template>

<style scoped>
.max-size {
  width: 100%;
  height: 100%;
}
.constrain-width {
  display: flex;
  flex-grow: 0;

  margin: auto;
  width: 100%;
  height: auto;
  aspect-ratio: 1/1;
}

.constrain-height {
  display: flex;
  flex-grow: 0;

  margin: auto;
  width: auto;
  height: 100%;
  aspect-ratio: 1/1;
  background-color: blue;
}
</style>
