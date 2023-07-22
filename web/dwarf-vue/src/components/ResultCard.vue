<script setup>
import { defineProps } from "vue";

const props = defineProps({
    dataId: {
        type: String,
        required: true,
    },
    date: {
        type: String,
        required: true,
    },
    index: {
        type: Number,
        required: true,
    },
});
</script>

<template>
    <article class="Card" :data-id="props.dataId">
        <p>{{ props.index }}. Creada el: {{ props.date }}</p>
        <button :data-id="props.dataId">Ver</button>
    </article>
</template>
