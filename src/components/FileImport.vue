<script setup lang="ts">
import Subtitle from "./Subtitle.vue";
import Subsubtitle from "./Subsubtitle.vue";
import Selection from "./Selection.vue";
import Button from "./Button.vue";
import SelectButton from "./SelectButton.vue";
import { computed, reactive, ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
const emit = defineEmits(["close"]);

const tags = [];
const categories = {};
const file = ref("");

async function openFilePicker() {
  const selected = await open({
    multiple: false,
    defaultPath: await documentDir(),
  });
  if (selected && selected.length > 0) {
    file.value = selected;
  }
}
const config = reactive({
  cfg: {
    folder: "Loading...",
    save_date: true,
    categories: [],
    tags: [],
  },
});

function toggleTag(selected, tag) {
  if (selected && !tags.includes(tag)) {
    tags.push(tag);
  } else if (!selected && tags.includes(tag)) {
    const index = tags.indexOf(tag);
    if (index > -1) {
      tags.splice(index, 1);
    }
  }
}

const validImport = computed(() => {
  return file.value.length > 0;
});

function toggleCat(category, value) {
  categories[category] = value;
}

invoke("load_config")
  .then((cfg) => {
    config.cfg = cfg;
    for (const cat of config.cfg.categories) {
      categories[cat.name] = null;
    }
  })
  // TODO handle
  .catch((err) => console.error(err));
</script>
<template>
  <div class="flex-none bg-mantle p-6 space-y-6 w-80">
    <Subtitle>Import File</Subtitle>
    <div>
      <p>{{ file }}</p>
      <Button @click="openFilePicker">Select</Button>
    </div>

    <div class="space-y-2">
      <Subsubtitle>Categories</Subsubtitle>
      <div v-for="cat in config.cfg.categories">
        <label class="block pb-1" :for="cat.name + '.cat'">{{
          cat.name
        }}</label>
        <Selection
          @valueChanged="(value) => toggleCat(cat.name, value)"
          :id="cat.name + '.cat'"
          :values="cat.values"
        ></Selection>
      </div>
    </div>

    <div class="space-y-2">
      <Subsubtitle>Tags</Subsubtitle>
      <div class="grid grid-cols-2 gap-2">
        <div v-for="(tag, index) in config.cfg.tags" :key="index">
          <SelectButton
            @selectionChanged="(selected) => toggleTag(selected, tag)"
            >{{ tag }}</SelectButton
          >
        </div>
      </div>
    </div>
    <div class="space-x-3">
      <Button :disabled="!validImport" @click="">Import</Button>
      <Button @click="() => emit('close')">Close</Button>
    </div>
  </div>
</template>
