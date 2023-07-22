<script setup>
import Row from "./Row.vue";

const props = defineProps(["results"]);
</script>

<template>
    <table class="ResultsTable">
        <Row title="Entrada" :value="props?.results?.input || '$ 0'" />
        <Row title="IVA" :value="props?.results?.iva || '$ 0'" />
        <Row title="ISR" :value="props?.results?.isr || '$ 0'" />
        <Row
            title="Libre de impuestos"
            :value="props?.results?.taxes_free || '$ 0'"
        />
        <Row title="Subtotal" :value="props?.results?.subtotal || '$ 0'" />
        <Row
            title="Total de la factura"
            :value="props?.results?.total || '$ 0'"
        />
    </table>
</template>

<style>
.ResultsTable tr th {
    text-align: left;
}

.ResultsTable tr td {
    text-align: right;
}
</style>
