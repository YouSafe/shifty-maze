<template>
  <div class="container max-size">
    <div class="square">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.max-size {
  width: 100%;
  height: 100%;
}

.container {
  container-type: size;
}

.square {
  width: 100cqmin;
  height: 100cqmin;
}
</style>
