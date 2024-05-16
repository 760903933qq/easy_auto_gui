<script setup>
import { onMounted, onUnmounted, defineProps, ref } from 'vue';

const props = defineProps({
    selectedScript: Object,
});
const handleKeydown = (event) => {
    if (event.key == 'F11') {
        event.preventDefault();
        startRecord();
    }
};

async function startRecord() {
    console.log('开始录制');
}


onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
});


onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
});

// 事件卡片(模拟两个)
const eventCardList = ref([
    {
        id: 1,
        type: 'click',
        image: 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANIAAAATCAYAAADlLVNTAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAALPSURBVGhD7Zu7jtpAFIb/5EWIgly4MK+AFIS0DS6hpkgTiYImDZRU21Ag0ayEa1rTIBkheAUailUi/CTOHHvGDLYxNydmo/NJ9s7tnBnDnItndz+tVqug0WiAYZj7+Sx/MgzzAGxIDFMAbEgMUwBsSAxTAGxIDFMAz2FIvoNOrQPHl/VMthheHJNHnvy9ukmuhlp83bs+0jMUd1XW9eh9zDNA33WSpzAkf72EYRtYru+2khIx0V/ssNuJa2Jg3Lpn09cx2o3EPYu8PqYsksb0BIbkY7000Bw1YSzXoqajefyhJ9sIH07nGAk6sfuW3twZxn3DcFdTew8u9mKj6+OJc33a3NdGmnoTtizmyftOR7Yno1DWWlRfOFBwTq8cpz376XMyRUOfcQz9QrZUDrOgPdiExc3ACmQxhOrt2UFVAstqB6p6ZBMMrIG4q7IVWEpJKKP3ZckT6b7U3O1ZkBY9lTvM2vG48/L6ehW6nuRaTuv5es89O1MkFn3O2kWUHpG2b2MYzShxqTdtuJ5KjLbwXBMv3ypR9cTbC7bK85IHf8fv2PmKVOu7TIRCGb3vWjLm3i+RnXlG0YPW0hobmMy7qOTKf0HVdNGLQuWNXFpXEc/O3AKl9ETJhkQbA3B7MlXpUcUTrRcgI5pWsaD3kt3k1MD+Odo70lXvMhV05zssqlPxzHyI8JFRRkSUa0hbD649kZswuia2iygokefeHw8gaGxUijC+ii0pSLaLCKFkfGcK13yBcuDXU0fTTsx9k57L8pXuK/rmrRHj0XUxRaEbEVGqIW1FOLJlWqc4pnfCc7/2IfKmKFp5OEYeSlvcXro9xITx66dMtYD+K6VaRLQJ04cNRLqvPprAUHP33jU913FeXh0WtDA2fqCbUpq3zsfXxTxO0oiI/+yvv2mTTlFdzDM2KMP8PZ7g+JthPj78/0gMUwAckRimANiQGOZhgD+sQ0J1Ryb0PwAAAABJRU5ErkJggg==',
        params:
        {
            name: '坐标',
            value: '100, 200',
        },
    },

]);

const radio1 = ref('1')



</script>
<template>
    <el-button @click="startRecord">开始录制（F11）</el-button>
    <div v-if="selectedScript">
        <p>当前选中的脚本是：{{ selectedScript.name }}</p>
    </div>
    <div class="eventCards">
        <div v-for="eventCard in eventCardList" :key="eventCard.id" class="eventCard">
            <el-col>
                <el-row :span="12">
                    <img :src="eventCard.image" alt="" class="button_img" />
                </el-row>
                <el-row :span="12">
                    <el-col :span="12">

                        <el-button>修改</el-button>

                    </el-col>
                    <el-col :span="12">

                        <el-col :span="12">
                            {{ eventCard.params.value }}
                        </el-col>
                        <el-col :span="12">
                            <el-radio-group v-model="radio1" class="ml-4">
                                <el-radio value="1" size="large">使用图片</el-radio>
                                <el-radio value="2" size="large">使用坐标</el-radio>
                            </el-radio-group>
                        </el-col>
                    </el-col>
                </el-row>
            </el-col>
        </div>
    </div>

</template>
<style>
.eventCard {
    margin-top: 20px;
    padding: 20px;
    background-color: #f2f2f2;
    border-radius: 4px;
    border: 1px solid #e9e9e9;
    height: 100px;
    width: 100px;
}

.eventCards {
    display: flex;
    flex-wrap: wrap;
    gap: 20px;
}

.button_img {
    width: 100%;
    height: 100%;
    object-fit: contain;
}
</style>