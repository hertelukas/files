<script setup lang="ts">
import { ref } from "vue";
import Button from "./Button.vue";
import FileImport from "./FileImport.vue";
const emit = defineEmits(["changeWindow"]);
const showingImport = ref(false);

function toggleImport() {
  showingImport.value = !showingImport.value;
}

function closeImport() {
  showingImport.value = false;
}
</script>

<template>
  <div class="flex h-screen">
    <FileImport v-if="showingImport" @close="closeImport" />
    <div class="flex-auto p-6">
      <div class="flex flex-col space-y-4">
        <div class="flex flex-row space-x-4">
          <Button @click="() => emit('changeWindow', 'configuration')"
            >Configuration</Button
          >
          <Button @click="toggleImport">Import File</Button>
        </div>
        <div class="flex flex-row space-x-4"></div>
      </div>
    </div>
  </div>
</template>
