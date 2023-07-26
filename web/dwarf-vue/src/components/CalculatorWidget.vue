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

            <button title="Calcular" @click="calc">
                <CheckIcon />
            </button>
            <button title="Borrar" @click="clear">
                <CancelIcon />
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
    box-shadow: -5px -1px 30px 3px rgba(0, 0, 0, 0.39);
    border-radius: 1rem;
    max-width: 700px;
    width: 99%;
    margin: auto;
}

.calc-options {
    display: inline-grid;
    grid-template-columns: 50% 50%;
    padding: 1%;
    gap: 7px;
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

.principal-input {
    border-radius: 0.5rem;
    border: none;
    box-shadow: -5px -1px 30px 3px rgba(0, 0, 0, 0.39);
    height: 35px;
    padding: 1%;
    color: rgb(175, 175, 175);
    font-size: 20px;
}
</style>
