<script setup>
import { defineProps, ref } from "vue";
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
    },
});
const store = useStore();
const show = ref(true);

const deleteCard = (event) => {
    const id = event.target.dataset.id;
    let aborted = false;
    show.value = false;

    toast("Eliminado. <a>Haga click para cancelar.</a>", {
        type: "warning",
        dangerouslyHTMLString: true,
        position: "top-center",
        autoClose: 4000,
        transition: "slide",
        onClick: () => {
            aborted = true;
            show.value = true;
        },
        onClose: () => {
            if (aborted) {
                return;
            }
            deleteBill(id);
            store.commit("updateStoredBills");
        },
    });
};

const showBill = (event) => {
    const id = event.target.dataset.id;
    store.commit("updateSelectedBill", id);
};

</script>

<template>
    <article class="Card" :data-id="props.dataId" v-show="show">
        <p class="number">{{ props.index }}</p>
        <p class="total">{{ props.total }}</p>
        <p class="date">{{ props.date }}</p>
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
    border: 2px solid rgb(94, 94, 94);
    margin: 1% auto;
    gap: 2%;
    display: grid;
    text-align: left;
    grid-template-columns: 1fr 3fr 3fr 3fr;
    align-items: center;
}

.number {
    background-color: rgb(94, 94, 94);
    height: 100%;
    border-radius: 0.2rem 0 0 0.2rem;
    display: inline-grid;
    place-items: center;
}

.card-buttons {
    display: inline-grid;
    gap: 10px;
    padding: 7px;
    grid-template-columns: 1fr 1fr;
}

@media screen and (max-width: 700px) {
    .Card {
        grid-template-columns: 15px auto auto;
        grid-template-areas: "number total btns" "number date btns";
        text-align: center;
    }

    .number {
        grid-area: number;
    }

    .total {
        grid-area: total;
    }

    .date {
        grid-area: date;
    }

    .card-buttons {
        grid-area: btns;
        grid-template-columns: 1fr;
        grid-template-rows: 1fr 1fr;
    }

    .Card button {
        width: 80%;
        margin: auto;
    }
}

@media (prefers-color-scheme: light) {
    .Card {
        border-color: var(--ligth-secondary-bg-color);
    }

    .number {
        background-color: var(--ligth-secondary-bg-color);
    }
}
</style>
