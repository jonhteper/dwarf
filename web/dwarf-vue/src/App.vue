<script setup>
import { ref } from "vue";
import Calculator from "./components/Calculator.vue";
import Storage from "./components/Storage.vue";

const updateStorage = ref(false);

const sendUpdateStorageSignal = () => {
    updateStorage.value = true;
};

const resetUpdateStoreSignal = () => {
    updateStorage.value = false;
};
</script>

<template>
    <h1 class="title">Calculadora Resico</h1>
    <Calculator @saved="sendUpdateStorageSignal" />
    <Storage :updateStorage="updateStorage" @updated="resetUpdateStoreSignal" />
</template>

<style scoped>
.title {
    text-align: center;
    padding: 25px 0;
}
</style>
