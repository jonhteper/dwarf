<script setup>
import ResultsTable from './ResultsTable.vue';
import CheckIcon from './CheckIcon.vue';
import CancelIcon from './CancelIcon.vue';
import DollarIcon from './DollarIcon.vue';
import { bill, reversed_bill } from '../pkg/dwarf_wasm'
import { ref } from 'vue'

const user_input = ref('0');

const instructions = [
    'Ingresa cuánto dinero necesitas libre de impuestos',
    'Ingresa el monto total de la factura'
]


const calc = () => {
    console.warn('Calculando...')
    const n = user_input.value;
    console.warn(n);
    console.warn(bill(parseFloat(n)).total)
}

const clear = () => {
    user_input.value = '0';
    console.warn('borrando')
}
</script>

<template>
    <div class="Calculator">
        <div class="calc-options">
            <button>Factura</button>
            <button>Invertido</button>
        </div>
        <i class="help-info"></i>
        <div class="input-zone">
            <DollarIcon />
            <input type="number" class="principal-input" v-model="user_input"
                placeholder="Ingresa una cantidad">

            <button title="Calcular" @click="calc">
                <CheckIcon />
            </button>
            <button title="Borrar" @click="clear">
                <CancelIcon />
            </button>
        </div>

        <ResultsTable />
    </div>
</template>

<style scoped>
.Calculator {
    display: inline-grid;
    grid-template-rows: 60px 50px 60px auto;
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