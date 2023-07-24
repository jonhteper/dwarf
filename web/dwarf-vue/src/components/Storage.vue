<script setup>
import { ref, defineProps, watch, defineEmits } from "vue";
import ResultCard from "./ResultCard.vue";
import { read_storage } from "../utils/storage";

const props = defineProps({
    updateStorage: {
        type: Boolean,
        required: false,
    },
});
const emits = defineEmits(["updated"]);

const stored_bills = ref(read_storage());

watch(
    () => props.updateStorage,
    () => {
        stored_bills.value = read_storage();
        console.warn("actualizando...");
        emits("updated");
    }
);
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
