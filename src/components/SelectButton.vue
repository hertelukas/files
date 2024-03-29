<script setup>
import { ref } from "vue";
const emit = defineEmits(["selectionChanged"]);
const selectedClass =
  "bg-blue text-mantle border rounded w-32 hover:bg-pink p-1 focus:!outline-none";
const unSelectedClass =
  "bg-mantle text-blue border border-blue rounded w-32 hover:bg-pink hover:text-mantle p-1 focus:!outline-none";

const buttonClass = ref(unSelectedClass);

let selected = false;

function handleClick() {
  selected = !selected;
  buttonClass.value = selected ? selectedClass : unSelectedClass;

  emit("selectionChanged", selected);
}
</script>
<template>
  <button @click="handleClick" :class="buttonClass">
    <slot>Button</slot>
  </button>
</template>
