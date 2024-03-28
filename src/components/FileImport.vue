<script setup lang="ts">
import Subtitle from "./Subtitle.vue";
import Button from "./Button.vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
const emit = defineEmits(["close"]);

async function openFilePicker() {
  const selected = await open({
    multiple: false,
    defaultPath: await documentDir(),
  });
  if (selected != null) {
    console.log(selected);
  }
}
</script>
<template>
  <div class="flex-none bg-mantle p-6 space-y-6 w-80">
    <Subtitle>Import File</Subtitle>
    <div>
      <Button @click="openFilePicker">Select</Button>
    </div>
    <div>
      <Button @click="() => emit('close')">Close</Button>
    </div>
  </div>
</template>
