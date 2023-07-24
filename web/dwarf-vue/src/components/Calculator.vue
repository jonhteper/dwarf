<script setup>
import Results from "./Results.vue";
import CheckIcon from "./icons/CheckIcon.vue";
import CancelIcon from "./icons/CancelIcon.vue";
import DollarIcon from "./icons/DollarIcon.vue";
import { bill, reversed_bill } from "../utils/dwarf/dwarf_wasm";
import { ref } from "vue";
import Tabs from "./Tabs.vue";
import { BillOptions } from "../utils/bill-options";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";

const user_input = ref("0");
const option_selected = ref(BillOptions.Bill);
const bill_results = ref({});
const emits = defineEmits(["saved"]);

const update_option_selected = (option) => {
    option_selected.value = option;
};

const calc = () => {
    const n = parseFloat(user_input.value || 0);
    if (n <= 0) {
        toast("Ingresa un número mayor a 0", {
            type: "warning",
            position: "top-center",
            autoClose: 3000,
            transition: "slide",
        });
        return;
    }
    switch (option_selected.value) {
        case BillOptions.Bill:
            bill_results.value = bill(n);
            break;
        case BillOptions.ReverseBill:
            bill_results.value = reversed_bill(n);
            break;
        default:
            alert("Opción no válida");
    }
};

const clear = () => {
    user_input.value = "0";
    bill_results.value = {};
};

const emitSaved = () => {
    emits("saved");
};
</script>

<template>
    <div class="Calculator">
        <Tabs @option-selected="update_option_selected" />
        <div class="input-zone">
            <DollarIcon />
            <input
                type="number"
                class="principal-input"
                v-model="user_input"
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

        <Results :results="bill_results" @saved="emitSaved" />
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
    width: 700px;
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
