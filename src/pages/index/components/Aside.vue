<script setup>
import { ref, defineProps, defineEmits } from 'vue';

const scriptsList = ref([]);

const props = defineProps();
const emit = defineEmits(['scriptSelected']);
const handleClickScript = (script) => {
    console.log('点击脚本', script);
    emit('scriptSelected', script);
};

const handleDeleteScript = (script) => {
    console.log('删除脚本', script);
    scriptsList.value = scriptsList.value.filter((item) => item.id !== script.id);
};


const createScript = () => {
    console.log('创建脚本');
    scriptsList.value.push({
        id: scriptsList.value.length + 1,
        name: '脚本' + (scriptsList.value.length + 1),
    });
};


</script>
<template>
    <el-card style="max-width: 480px">
        <template #header>
            <div class="card-header">
                <el-button type="text" @click="createScript">新建脚本</el-button>
            </div>
        </template>
        <p v-if="scriptsList.length === 0" class="text">暂无脚本</p>
        <div v-for="script in scriptsList" :key="script.id" class="script-item">
            <button class="text item" @click="handleClickScript(script)">
                {{ script.name }}
            </button>
            <button class="delete item" @click="handleDeleteScript(script)">
                删除
            </button>
        </div>
    </el-card>
</template>


<style scoped>
/* 容器样式，使用 Flexbox 实现并排布局 */
.script-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    /* 如果你希望按钮之间有间距 */
}

/* 按钮样式，根据需要调整 */
.button {
    margin: 0 5px;
    /* 为按钮添加左右间距 */
}
</style>