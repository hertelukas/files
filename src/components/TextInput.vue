<script setup lang="ts">
const model = defineModel();
</script>

<template>
  <input
    type="text"
    v-model="model"
    class="rounded bg-surface1 p-1 focus:!outline-none focus:border focus:border-blue"
  />
</template>
