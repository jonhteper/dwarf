<template>
    <div class="ResultsTable">
        <h2>Resultados</h2>
        <table>
            <tr>
                <th>Entrada</th>
                <td>$ 0</td>
            </tr>
            <tr>
                <th>IVA</th>
                <td>$ 0</td>
            </tr>
            <tr>
                <th>ISR</th>
                <td>$ 0</td>
            </tr>
            <tr>
                <th>Libre de impuestos</th>
                <td>$ 0</td>
            </tr>
            <tr>
                <th>Subtotal</th>
                <td>$ 0</td>
            </tr>
            <tr>
                <th>Total de la factura</th>
                <td>$ 0</td>
            </tr>
        </table>
        <button>Guardar</button>
    </div>
</template>

<style scoped>
    .ResultsTable {
        display: inline-grid;
        text-align: center;
    }

    .ResultsTable table {
        margin: auto;
        width: 90%;
        padding: 1%;
    }
    .ResultsTable table th {
        text-align: left;
    }

    .ResultsTable table td {
        text-align: right;
    }

    .ResultsTable button {
        width: 90%;
        margin: auto;
    }
</style>