<script setup>
import { computed } from "vue";
import ResultCard from "./ResultCard.vue";
import { useStore } from "vuex";
import ResultModal from "./ResultModal.vue";

const store = useStore();

const storedBills = computed(() => store.getters.storedBills);
</script>
<template>
    <div class="Storage">
        <h2>Resultados guardados</h2>
        <div class="storage-list">
            <ResultCard
                v-for="([key, value], index) in storedBills"
                :key="key"
                :dataId="key"
                :date="value.createdAt"
                :index="index + 1"
                :total="value.total"
            />
        </div>
        <ResultModal />
    </div>
</template>

<style scoped>
.Storage {
    /* max-height: 500px; */
    padding: 1%;
    width: 99%;
    max-width: 700px;
    margin: auto;
    box-shadow: -5px -1px 30px 3px rgba(0, 0, 0, 0.39);
    border-radius: 0.5rem;
}

.storage-list {
    overflow-y: auto;
    max-height: 300px;
    padding: 2%;
    /* outline: 1px solid red; */
}
</style>
