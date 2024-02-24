<template>
  <div class="max-size">
    <div class="constrain-width">
      <div class="constrain-height">
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.max-size {
  display: flex;
  width: 100%;
  height: 100%;
  min-width: 0;
  min-height: 0;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}

.constrain-width {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: auto;
  aspect-ratio: 1/1;
  min-width: 0;
  min-height: 0;
  flex-direction: column;
}

.constrain-height {
  display: flex;
  width: auto;
  height: 100%;
  aspect-ratio: 1/1;
  min-width: 0;
  min-height: 0;
  flex-direction: column;
}
</style>
