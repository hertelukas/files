<script setup lang="ts">
const emit = defineEmits(["click"]);

function click() {
  emit("click");
}
</script>

<template>
  <button
    class="bg-mantle rounded w-32 hover:bg-crust p-1.5 disabled:bg-base"
    @click="click"
  >
    <slot>Button</slot>
  </button>
</template>
