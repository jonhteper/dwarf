<script setup>
import TableRow from "./TableRow.vue";

const props = defineProps(["results"]);
</script>

<template>
    <table class="ResultsTable">
        <TableRow title="Entrada" :value="props?.results?.input || '$ 0'" />
        <TableRow title="IVA" :value="props?.results?.iva || '$ 0'" />
        <TableRow title="ISR" :value="props?.results?.isr || '$ 0'" />
        <TableRow
            title="Libre de impuestos"
            :value="props?.results?.taxesFree || '$ 0'"
        />
        <TableRow title="Subtotal" :value="props?.results?.subtotal || '$ 0'" />
        <TableRow
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
