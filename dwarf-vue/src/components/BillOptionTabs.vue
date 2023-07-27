<script setup>
import { ref, defineEmits } from "vue";
import { BillOptions } from "../utils/bill-options";

const emits = defineEmits(["option-selected"]);

const infoOptions = [
    "Ingresa la cantidad que quieres libre de impuestos.",
    "Ingresa el monto total de la factura para obtener los valores.",
];

const optionSelected = ref(BillOptions.Bill);

const textInfo = ref(infoOptions[optionSelected.value]);

const changeOption = (event) => {
    const option = parseInt(event.target.dataset.option);
    optionSelected.value = option;
    textInfo.value = infoOptions[option];
    emits("option-selected", optionSelected.value);
};
</script>

<template>
    <div class="calc-options">
        <button
            id="billButton"
            @click="changeOption"
            :data-option="BillOptions.Bill"
            :class="{ active: optionSelected == BillOptions.Bill }"
        >
            Factura a empresa
        </button>
        <button
            id="reverseBillButton"
            @click="changeOption"
            :data-option="BillOptions.ReverseBill"
            :class="{ active: optionSelected == BillOptions.ReverseBill }"
        >
            Desde factura a empresa
        </button>
        <i class="help-info">{{ textInfo }}</i>
    </div>
</template>

<style scoped>
.calc-options {
    display: inline-grid;
    grid-template-columns: auto auto;
    grid-template-rows: 50% 50%;
    grid-template-areas: "btn1 btn2" "info info";
    padding: 1%;
    gap: 7px;
}

.calc-options i {
    grid-area: info;
}
</style>
