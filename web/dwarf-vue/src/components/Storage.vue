<script setup>
import { ref } from "vue";
import ResultCard from "./ResultCard.vue";
import { read_storage } from "../utils/storage";

const stored_bills = ref(read_storage());
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
