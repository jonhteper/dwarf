<script setup>
import ResultsTable from './ResultsTable.vue';
import CheckIcon from './CheckIcon.vue';
import CancelIcon from './CancelIcon.vue';
import DollarIcon from './DollarIcon.vue';

</script>

<template>
    <div class="Calculator">
        <div class="calc-options">
            <button>Factura</button>
            <button>Invertido</button>
        </div>
        <i class="help-info">Ingresa cuánto dinero necesitas libre de impuestos</i>
        <div class="input-zone">
            <DollarIcon/> <input class="principal-input" type="number" value="0" min="0">

            <button title="Calcular">
                <CheckIcon />
            </button>
            <button title="Borrar">
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