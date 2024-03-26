<script setup lang="ts">
import Title from "./Title.vue";
import Button from "./Button.vue";
const emit = defineEmits(["changeWindow"]);
</script>

<template>
  <Title>This is the main view</Title>
  <Button @click="() => emit('changeWindow', 'welcome')">Back</Button>
</template>
