<script setup>
import { computed, defineProps, ref } from "vue";
import Row from "./Row.vue";
import { save_in_storage } from "../utils/storage";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";

const props = defineProps(["results"]);

const is_ok = computed(() => props?.results?.total !== undefined);

const save = () => {
    if (!is_ok.value) {
        toast("Error al guardar. Vuelva a realizar el cálculo.", {
            type: "error",
            position: "top-center",
            autoClose: 3000,
            transition: "slide",
        });
        return;
    }
    save_in_storage(props.results);
    toast("Guardado con éxito", {
        type: "success",
        position: "top-center",
        autoClose: 3000,
        transition: "slide",
    });
};
</script>

<template>
    <div class="ResultsTable" v-if="is_ok">
        <h2>Resultados</h2>
        <table>
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
        <button @click="save" :disabled="!is_ok">Guardar</button>
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
