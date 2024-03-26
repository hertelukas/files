<script setup lang="ts">
import Title from "./Title.vue";
import Button from "./Button.vue";

const emit = defineEmits(["changeWindow"]);

function changeWindow(window) {
  emit("changeWindow", window);
}
</script>

<template>
  <Title>Welcome!</Title>
  <Button @click="() => {changeWindow('configuration')}">Configuration</Button>
</template>
