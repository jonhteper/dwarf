<script setup>
import { ref, defineEmits } from 'vue';
import { BillOptions } from '../utils/bill-options'

const emits = defineEmits(['option-selected']);

const info_options = [
    'Ingresa cuánto dinero necesitas libre de impuestos.',
    'Ingresa el monto total de la factura para obtener los valores.'
];

const option_selected = ref(BillOptions.Bill);

const text_info = ref(info_options[option_selected.value]);

const change_option = (event) => {
    const option = parseInt(event.target.dataset.option);
    option_selected.value = option;
    text_info.value = info_options[option];
    emits('option-selected', option_selected.value);
}

</script>

<template>
    <div class="calc-options">
        <button @click="change_option" :data-option="BillOptions.Bill"
            :class="{ active: option_selected == BillOptions.Bill }">Factura a empresa</button>
        <button @click="change_option" :data-option="BillOptions.ReverseBill"
            :class="{ active: option_selected == BillOptions.ReverseBill }">Desde factura a empresa</button>
        <i class="help-info">{{ text_info }}</i>
    </div>
</template>

<style scoped>
.calc-options {
    display: inline-grid;
    grid-template-columns: 50% 50%;
    grid-template-rows: 50% 50%;
    grid-template-areas: "btn1 btn2" "info info";
    padding: 1%;
    gap: 7px;
}

.calc-options i {
    grid-area: info;
}
</style>