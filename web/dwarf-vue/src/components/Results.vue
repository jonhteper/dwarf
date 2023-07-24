<script setup>
import { computed, defineProps, defineEmits } from "vue";
import { save_in_storage } from "../utils/storage";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";
import ResultsTable from "./ResultsTable.vue";

const props = defineProps(["results"]);
const emits = defineEmits(["saved"]);

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
    emits("saved");
};
</script>

<template>
    <div class="Results" v-if="is_ok">
        <h2>Resultados</h2>
        <ResultsTable :results="props.results" />
        <button @click="save" :disabled="!is_ok">Guardar</button>
    </div>
</template>

<style>
.Results {
    display: inline-grid;
    text-align: center;
}

.Results .ResultsTable {
    width: 90%;
    margin: auto;
}

.Results button {
    width: 90%;
    margin: auto;
}
</style>
