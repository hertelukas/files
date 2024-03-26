<script setup lang="ts">
const emit = defineEmits(["click"]);

function click() {
  emit("click");
}
</script>

<template>
  <button
    class="bg-mantle rounded hover:bg-crust p-2 disabled:bg-base"
    @click="click"
  >
    <slot>Button</slot>
  </button>
</template>
