<script setup>
import ResultsTable from "./ResultsTable.vue";
import { useStore } from "vuex";
import { computed } from "vue";

const store = useStore();
const isOk = computed(() => store.getters.selectedBill?.total !== undefined);
const results = computed(() => store.getters.selectedBill);

const closeModal = () => {
    store.commit("clearSelectedBill");
};
</script>

<template>
    <div class="ModalShadow" v-show="isOk">
        <div class="ModalResult">
            <h2>Resultado</h2>
            <p>Creado:</p>
            <ResultsTable :results="results" />
            <button @click="closeModal">Cerrar</button>
        </div>
    </div>
</template>

<style scoped>
.ModalShadow {
    position: absolute;
    top: 0;
    left: 0;
    background-color: #1a1a1a88;
    backdrop-filter: blur(18px);
    width: 100vw;
    height: 100vh;
    display: grid;
    place-items: center;
    padding: 1%;
}

.ModalResult {
    background-color: black;
    border-radius: 0.5rem;
    width: 90%;
    max-width: 900px;
    padding: 2%;
}

.ModalResult .ResultsTable {
    width: 90%;
    margin: auto;
}
</style>
