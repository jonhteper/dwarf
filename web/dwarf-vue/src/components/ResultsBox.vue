<script setup>
import { computed, defineProps } from "vue";
import { saveInStorage } from "../utils/storage";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";
import ResultsTable from "./ResultsTable.vue";
import { useStore } from "vuex";

const props = defineProps(["results"]);
const store = useStore();

const isOk = computed(() => props?.results?.total !== undefined);

const save = () => {
    if (!isOk.value) {
        toast("Error al guardar. Vuelva a realizar el cálculo.", {
            type: "error",
            position: "top-center",
            autoClose: 3000,
            transition: "slide",
        });
        return;
    }
    saveInStorage(props.results);
    toast("Guardado con éxito", {
        type: "success",
        position: "top-center",
        autoClose: 3000,
        transition: "slide",
    });
    store.commit("updateStoredBills");
};
</script>

<template>
    <div class="Results" v-if="isOk">
        <h2>Resultados</h2>
        <ResultsTable :results="props.results" />
        <button @click="save" :disabled="!isOk">Guardar</button>
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
