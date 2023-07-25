<script setup>
import CalculatorWidget from "./components/CalculatorWidget.vue";
import ResultsStorage from "./components/ResultsStorage.vue";

</script>

<template>
    <h1 class="title">Calculadora Resico</h1>
    <CalculatorWidget />
    <ResultsStorage />
</template>

<style scoped>
.title {
    text-align: center;
    padding: 25px 0;
}
</style>
