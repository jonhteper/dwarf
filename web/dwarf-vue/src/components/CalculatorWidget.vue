<script setup>
import ResultsBox from "./ResultsBox.vue";
import CheckIcon from "./icons/CheckIcon.vue";
import CancelIcon from "./icons/CancelIcon.vue";
import DollarIcon from "./icons/DollarIcon.vue";
import { bill, reversedBill } from "../utils/dwarf/dwarf_wasm";
import { ref } from "vue";
import BillOptionTabs from "./BillOptionTabs.vue";
import { BillOptions } from "../utils/bill-options";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";

const userInput = ref("0");
const optionSelected = ref(BillOptions.Bill);
const billResults = ref({});

const updateOptionSelected = (option) => {
    optionSelected.value = option;
};

const calc = () => {
    const n = parseFloat(userInput.value || 0);
    if (n <= 0) {
        userInput.value = "0";
        toast("Ingresa un número mayor a 0", {
            type: "warning",
            position: "top-center",
            autoClose: 3000,
            transition: "slide",
        });
        return;
    }
    switch (optionSelected.value) {
        case BillOptions.Bill:
            billResults.value = bill(n);
            break;
        case BillOptions.ReverseBill:
            billResults.value = reversedBill(n);
            break;
        default:
            alert("Opción no válida");
    }
};

const clear = () => {
    userInput.value = "0";
    billResults.value = {};
};
</script>

<template>
    <div class="Calculator">
        <BillOptionTabs @option-selected="updateOptionSelected" />
        <div class="input-zone">
            <DollarIcon />
            <input
                type="number"
                class="principal-input"
                v-model="userInput"
                placeholder="Ingresa una cantidad"
                min="0"
            />

            <button class="calc-btn" title="Calcular" @click="calc">
                <CheckIcon />
                <p class="btn-text">Calcular</p>
            </button>
            <button class="clear-btn" title="Limpiar" @click="clear">
                <CancelIcon />
                <p class="btn-text">Limpiar</p>
            </button>
        </div>

        <ResultsBox :results="billResults" />
    </div>
</template>

<style scoped>
.Calculator {
    display: inline-grid;
    grid-template-rows: 110px 60px auto;
    padding: 1%;
    text-align: center;
    box-shadow: var(--box-shadow);
    border-radius: 1rem;
    max-width: 700px;
    width: 99%;
    margin: auto;
}


.input-zone {
    display: inline-grid;
    grid-template-columns: 30px auto 40px 40px;
    gap: 5px;
    align-content: center;
    width: 90%;
    margin: auto;
}

.input-zone .DolarIcon {
    height: 30px;
}

.input-zone button {
    display: inline-grid;
    border-radius: 50%;
    height: 35px;
    width: 35px;
    text-align: center;
    place-items: center;
    padding: auto;
    padding: 0;
}

button svg {
    width: 20px;
    height: 20px;
}

button .btn-text {
    display: none;
}

.principal-input {
    border-radius: 0.5rem;
    border: none;
    box-shadow: var(--box-shadow);
    height: 35px;
    padding: 1%;
    color: rgb(175, 175, 175);
    font-size: 20px;
}

@media screen and (max-width: 600px) {
    .Calculator {
        grid-template-rows: 140px 100px auto;
    }
    .input-zone {
        grid-template-columns: 30px 1fr 1fr;
        grid-template-areas: "dollar input input" ". clear calc";
        height: auto;
    }
    .principal-input {
        grid-area: input;
    }

    .input-zone button {
        width: 110px;
        height: 40px;
        border-radius: 0.5rem;
    }

    button svg {
        display: none;
    }

    button .btn-text {
        display: inline;
    }

    .calc-btn {
        grid-area: calc;
    }
    .clear-btn {
        grid-area: clear;
    }
}
@media (prefers-color-scheme: light) {
    .principal-input {
        color: rgb(126, 126, 126);
    }
}
</style>
