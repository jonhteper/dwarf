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
    total: {
        type: String,
        required: true,
    }
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

const showBill = (event) => {
    const id = event.target.dataset.id;
    store.commit("updateSelectedBill", id);
    console.warn(id);
};


</script>

<template>
    <article class="Card" :data-id="props.dataId">
        <p>{{ props.index }}</p>
        <p>{{ props.total }}</p>
        <p>{{ props.date }}</p>
        <div class="card-buttons">
            <button :data-id="props.dataId" @click="deleteCard">Borrar</button>
            <button :data-id="props.dataId" @click="showBill">Ver</button>
        </div>
    </article>
</template>

<style scoped>
.Card {
    border-radius: 0.5rem;
    width: 90%;
    padding: 1%;
    border: 2px solid rgb(94, 94, 94);
    margin: 1% auto;
    display: flex;
    gap: 2%;
    justify-content: space-between;
    align-items: center;
}

.card-buttons {
    display: flex;
    gap: 10px;
}
</style>