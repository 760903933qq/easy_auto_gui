<script setup>
import { defineProps } from 'vue';

const props = defineProps({
    selectedScript: Object,
});
</script>
<template>
    main
    <div v-if="selectedScript">
        <p>当前选中的脚本是：{{ selectedScript.name }}</p>
    </div>
</template>