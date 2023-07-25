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
            />
        </div>
        <ResultModal />
    </div>
</template>
