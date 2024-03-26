<script setup lang="ts">
import Title from "./Title.vue";
import Subtitle from "./Subtitle.vue";
import Button from "./Button.vue";
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { readDir } from "@tauri-apps/api/fs";

const emit = defineEmits(["changeWindow"])

const folder = ref("No folder selected");
const canConfirm = ref(false);

async function openDirectoryPicker() {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await documentDir(),
  });
  if (selected != null) {
    try {
      const files = await readDir(selected);
      // Folder has to be empty
      if (files.length === 0) {
        folder.value = selected;
        canConfirm.value = true;
      } else {
        folder.value = `${selected} is not empty`;
        canConfirm.value = false;
      }
    } catch {
      folder.value = "Cannot open folder";
      canConfirm.value = false;
    }
  }
}
function submitConfig() {
   // TODO this needs to be send to the backend, and then switched to main
   emit("changeWindow", "main")
}
</script>

<template>
  <div class="space-y-10">
    <Title>Configuration</Title>
    <form @submit.prevent="submitConfig" class="space-y-2">
      <Subtitle>General</Subtitle>
      <div>
        <Button type="button" @click="openDirectoryPicker">Directory</Button>
        <p>{{ folder }}</p>
      </div>
      <Subtitle>Structure</Subtitle>
      <div>
        <label class="flex items-center space-x-2">
          <input type="checkbox" class="accent-blue" />
          <span>Checkbox 1</span>
        </label>

        <!-- Checkbox with text -->
        <label class="flex items-center space-x-2">
          <input type="checkbox" class="accent-blue" />
          <span>Checkbox 2</span>
        </label>

        <!-- Checkbox with text -->
        <label class="flex items-center space-x-2">
          <input type="checkbox" class="accent-blue" />
          <span>Checkbox 3</span>
        </label>
      </div>
      <Button :disabled="!canConfirm" type="submit"> Confirm </Button>
    </form>
  </div>
</template>
