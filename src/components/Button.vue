<script setup lang="ts">
const emit = defineEmits(["click"]);

function click() {
  emit("click");
}
</script>

<template>
  <button
    class="bg-blue text-mantle rounded w-32 hover:bg-pink p-1 disabled:bg-crust disabled:text-text focus:!outline-none"
    @click="click"
  >
    <slot>Button</slot>
  </button>
</template>
