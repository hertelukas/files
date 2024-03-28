<script setup lang="ts">
import Title from "./Title.vue";
import Subtitle from "./Subtitle.vue";
import Button from "./Button.vue";
import TextInput from "./TextInput.vue";
import { reactive, ref, computed } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { documentDir } from "@tauri-apps/api/path";
import { readDir } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";

const emit = defineEmits(["changeWindow"]);

const newTag = ref("");
// newVals[i] is the input field of category i
const newVals = ref([]);
const newCategory = ref("");
const config = reactive({
  cfg: {
    folder: "Loading...",
    save_date: true,
    categories: [],
    tags: [],
  },
});

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
        config.cfg.folder = selected;
      } else {
        config.cfg.folder = null;
      }
    } catch {
      config.cfg.folder = null;
    }
  }
}

const categories = computed(() => {
  const res = [];
  config.cfg.categories.forEach((cat) => {
    res.push(cat.name);
  });
  return res;
});

function addTag() {
  if (newTag.value.trim() === "") {
    return;
  }
  if (config.cfg.tags.includes(newTag.value.trim())) {
    return;
  }
  config.cfg.tags.push(newTag.value.trim());
  newTag.value = "";
}

function addValue(i) {
  const valToAdd = newVals.value[i].trim();
  if (valToAdd === "") {
    return;
  }
  if (config.cfg.categories[i].values.includes(valToAdd)) {
    return;
  }
  config.cfg.categories[i].values.push(valToAdd);
  newVals.value[i] = "";
}

function addCategory() {
  const catToAdd = newCategory.value.trim();
  if (catToAdd === "") {
    return;
  }
  if (categories.value.includes(catToAdd)) {
    return;
  }
  config.cfg.categories.push({ name: catToAdd, values: [] });
  newCategory.value = "";
}

async function submitConfig() {
  invoke("store_config", { config: config.cfg })
    .then(() => emit("changeWindow", "main"))
    // TODO handle error
    .catch((err) => console.error(err));
}

// Load the config from the backend
invoke("load_config")
  .then((cfg) => {
    config.cfg = cfg;
  })
  // TODO handle this case, as this is the default when no config exists
  .catch((err) => console.error(err));
</script>

<template>
  <div class="space-y-10">
    <Title>Configuration</Title>
    <form @submit.prevent="submitConfig" class="space-y-6">
      <div class="space-y-2">
        <Subtitle>General</Subtitle>
        <div>
          <Button type="button" @click="openDirectoryPicker">Directory</Button>
          <p>{{ config.cfg.folder || "No folder selected" }}</p>
        </div>
        <label class="flex items-center space-x-2">
          <input
            type="checkbox"
            class="accent-blue"
            v-model="config.cfg.save_date"
          />
          <span>Track Date</span>
        </label>
      </div>

      <div class="space-y-4">
        <Subtitle>Categories</Subtitle>
        <div
          v-for="(category, i) in config.cfg.categories"
          :key="i"
          class="space-y-2"
        >
          <TextInput
            v-model="config.cfg.categories[i].name"
            placeholder="Category"
          />
          <div class="space-y-2">
            <div
              v-for="(value, j) in config.cfg.categories[i].values"
              class="ml-8"
            >
              <TextInput
                :key="i * j"
                v-model="config.cfg.categories[i].values[j]"
              />
            </div>
            <TextInput
              v-model="newVals[i]"
              placeholder="New value..."
              class="ml-8"
            />
          </div>
          <Button @click="addValue(i)" type="button" class="ml-8"
            >Add Value</Button
          >
        </div>
        <div class="space-y-2">
          <div>
            <TextInput
              v-model="newCategory"
              placeholder="New category..."
            />
          </div>
          <Button @click="addCategory" type="button">Add Category</Button>
        </div>
      </div>

      <div class="space-y-2">
        <Subtitle>Tags</Subtitle>
        <div class="grid grid-cols-6 gap-4">
          <TextInput
            v-for="(tag, index) in config.cfg.tags"
            :key="index"
            v-model="config.cfg.tags[index]"
            placeholder="Loading..."
          />
          <TextInput
            v-model="newTag"
            placeholder="New tag..."
          />
        </div>
        <Button @click="addTag" type="button">Add Tag</Button>
      </div>

      <Button :disabled="!config.cfg.folder" type="submit"> Confirm </Button>
    </form>
  </div>
</template>
