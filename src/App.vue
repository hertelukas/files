<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Configuration from "./components/Configuration.vue";
import Welcome from "./components/Welcome.vue";
import Main from "./components/Main.vue";
import { onMounted, ref } from "vue";

const currentWindow = ref("welcome");

onMounted(() => {
  document.body.classList.add("bg-base");
});
</script>

<template>
  <div class="text-text p-6">
    <Configuration
      @changeWindow="(window) => (currentWindow = window)"
      v-if="currentWindow === 'configuration'"
    />
    <Welcome
      @changeWindow="(window) => (currentWindow = window)"
      v-if="currentWindow === 'welcome'"
    />
    <Main
      @changeWindow="(window) => (currentWindow = window)"
      v-if="currentWindow === 'main'"
    />
  </div>
</template>

<style scoped></style>
