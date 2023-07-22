<script setup>
import Calculator from "./components/Calculator.vue";
import Storage from "./components/Storage.vue";
</script>

<template>
    <h1 class="title">Calculadora Resico</h1>
    <Calculator />
    <Storage />
</template>

<style scoped>
.title {
    text-align: center;
    padding: 25px 0;
}
</style>
