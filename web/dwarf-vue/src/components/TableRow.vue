<script setup>
import { defineProps } from "vue";
defineProps(["title", "value"]);
</script>

<template>
    <tr>
        <th>{{ title }}</th>
        <td>{{ value }}</td>
    </tr>
</template>
