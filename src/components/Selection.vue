<script setup lang="ts">
import { ref } from "vue";
const emit = defineEmits(["valueChanged"]);
const props = defineProps({
  values: Array,
});

function handleChange(event) {
  if (event.target.selectedIndex !== 0) {
    emit("valueChanged", event.target.value);
  } else {
    emit("valueChanged", null);
  }
}
</script>

<template>
  <select
    @change="handleChange"
    class="appearance-none bg-blue border text-mantle w-32 p-1 focus:!outline-none"
  >
    <option value="">-</option>
    <option v-for="val in values">{{ val }}</option>
  </select>
</template>
