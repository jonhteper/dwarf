<script setup>
import CalculatorWidget from "./components/CalculatorWidget.vue";
import ResultsStorage from "./components/ResultsStorage.vue";
</script>

<template>
    <div class="App">
        <h1 class="title">Calculadora Resico</h1>
        <CalculatorWidget />
        <ResultsStorage />
    </div>
</template>

<style scoped>
.App {
    display: inline-grid;
    grid-template-rows: 100px auto 500px;
    width: 100%;
    padding: 1%;
    text-align: center;
}
.title {
    text-align: center;
    padding: 25px 0;
}
</style>
