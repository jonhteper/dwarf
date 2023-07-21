<script setup>
import { defineProps } from 'vue';
import Row from './Row.vue';

const props = defineProps(['results']);
</script>

<template>
    <div class="ResultsTable">
        <h2>Resultados</h2>
        <table>
            <Row title="Entrada" :value="props?.results?.input || '0'"/>
            <Row title="IVA" :value="props?.results?.iva || '0'"/>
            <Row title="ISR" :value="props?.results?.isr || '0'"/>
            <Row title="Libre de impuestos" :value="props?.results?.taxes_free || '0'"/>
            <Row title="Subtotal" :value="props?.results?.subtotal || '0'"/>
            <Row title="Total de la factura" :value="props?.results?.total || '0'"/>
        </table>
        <button>Guardar</button>
    </div>
</template>

<style>
    .ResultsTable {
        display: inline-grid;
        text-align: center;
    }

    .ResultsTable table {
        margin: auto;
        width: 90%;
        padding: 1%;
    }
    .ResultsTable table th {
        text-align: left;
    }

    .ResultsTable table td {
        text-align: right;
    }

    .ResultsTable button {
        width: 90%;
        margin: auto;
    }
</style>