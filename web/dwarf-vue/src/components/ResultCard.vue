<script setup>
import { defineProps } from "vue";
import { useStore } from "vuex";
import { deleteBill } from "../utils/storage";
import { toast } from "vue3-toastify";

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
const store = useStore();

const deleteCard = (event) => {
    const id = event.target.dataset.id;
    deleteBill(id);
    store.commit("updateStoredBills");
    toast("Eliminado correctamente.", {
        type: "success",
        position: "top-center",
        autoClose: 3000,
        transition: "slide",
    });
};

const showBill = () => {};
</script>

<template>
    <article class="Card" :data-id="props.dataId">
        <p>{{ props.index }}. Creada el: {{ props.date }}</p>
        <button :data-id="props.dataId" @click="deleteCard">Borrar</button>
        <button :data-id="props.dataId" @click="showBill">Ver</button>
    </article>
</template>
