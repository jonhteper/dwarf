<script setup>
import { ref, defineProps, watch, defineEmits, computed } from "vue";
import ResultCard from "./ResultCard.vue";
import { read_storage } from "../utils/storage";
import { useStore } from "vuex";

const store = useStore();

const stored_bills = computed(() => store.getters.storedBills);


</script>
<template>
    <div class="Storage">
        <h2>Resultados guardados</h2>
        <div class="storage-list">
            <ResultCard
                v-for="([key, value], index) in stored_bills"
                :key="key"
                :dataId="key"
                :date="value.createdAt"
                :index="index + 1"
            />
        </div>
    </div>
</template>
