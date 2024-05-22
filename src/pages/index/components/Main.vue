<script setup>
import { onMounted, onUnmounted, ref, computed, reactive } from 'vue';
import VuePictureCropper, { cropper } from 'vue-picture-cropper'

import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { tr } from 'element-plus/es/locales.mjs';
const props = defineProps({
    selectedScript: Object,
});
const handleKeydown = (event) => {
    if (event.key == 'F11') {
        event.preventDefault();
        startRecord();
    }
};

const isRecording = ref(false);
async function startRecord() {
    console.log('开始录制');
    isRecording.value = true;
    await invoke('start_recording');
}

async function stopRecord() {
    console.log('结束录制');
    isRecording.value = false;
    await invoke('stop_recording');
}

let nextId = 1;
onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
    listen('button-press', (event) => {
        const newItem = {
            ...event.payload,
            id: nextId,
            nextStep: 'step' + (nextId + 1),
            type: 'sequence'
        }
        // eventCardList.value.push(newItem);
        eventCardDict.value['step' + nextId] = newItem;
        nextId++;
        console.log('收到按钮事件', eventCardDict.value);
    });
});



const eventCardDict = ref({});
const displayedSteps = computed(() => {
    let steps = [];
    let nextStepKey = 'step1';
    let stopLoop = false;
    while (!stopLoop) {
        if (eventCardDict.value[nextStepKey] === undefined) {
            stopLoop = true;
        } else {
            if (eventCardDict.value[nextStepKey].type === 'sequence') {
                steps.push(eventCardDict.value[nextStepKey]);
                nextStepKey = eventCardDict.value[nextStepKey].nextStep;
            }
        }
    }
    console.log('显示的步骤', steps);
    return steps;
});


onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
});


const conditionCardList = ref([
    {
        id: 1,
        image: 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANEAAAAUCAYAAAATH9joAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAK7SURBVHhe7Zu7juIwFIb/3RdhNaMUFMkrRFop0jSkDDXFlhQ020BJNQ1FJJqRSD1taJAiIfIKNBSjWZEnyeYkdmIgXKNZL+h8ksGX2D4k/n1sA9/SDDAMczPfxTvDMDfCImKYhrCIGKYhLCKGaQiLiGEaoldESYCu1UWQiHQtMUZnrznFqfoN2o5HsCxLBNlGU1uZ/xl61nVoFVGyXMBwDSyWdzbqSPz9Dwzma6zXFN7Ra4ky5qGpE5JGESVYLgw4YwfGYpmlVGhGF7P8KBJ5RIKgK2d/C91yyhceIKi8wyiW+X2E2GDSUa8njpUpfZ/0KgaejglnWdlRtXut7RLFnt0CRhP0LHagL1u1sJ2l3nCVR1dDMxXRHEp7s61MpKbppTJZsUqH5jB7lXEzNWUjeR21rK4+cVh20Lc3S+uqbmde1seu3UV7x+xQudR2Sso+tunMO/Y5mH8BPW81SLR5ovhtAsOx87jtuAgjOcvGiMI2Xn6Kad524BaxgnIvQl7kA39KT9HG4FfRXlFHLbuUmr43C9StNlu992wZ5wP9zJZuoHjSE3ZcbTvZA4TUh9XBZLPB5za/itEMLeMlmkSkDo4s9CkRZblnoEE4fcY834f4u+LSgo3xeo4BJng7Z/zNtmcCK/dea4yF1hh9qAIi9IgojhC6fjkwKPhuiMIZ/cBze1MdNtC1RazAeELuJ/bzs72NrJMEU4TtF0iHcjk2HHev75vaOcLVtu/dC0Y7+wIitIgoztyQK5ZykmpJ10LvdYBst194qQjVrE1LnbB/mJ/ThvH5Oy/rTIDBa68YsEIYhwcLxGGZPfZhyL7pBK5sRyE/mhdelJZZhn/eQ9xk+969sEbnvTXzZdQJiHiQX3HTCdYUz/N7PGq+Z9sZQuMRN8M8Bvx/IoZpCHsihmkIi4hhGsIiYphGAH8Bow45DQAun74AAAAASUVORK5CYII='
    },

    {
        id: 2,
        image: 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANEAAAAUCAYAAAATH9joAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAK7SURBVHhe7Zu7juIwFIb/3RdhNaMUFMkrRFop0jSkDDXFlhQ020BJNQ1FJJqRSD1taJAiIfIKNBSjWZEnyeYkdmIgXKNZL+h8ksGX2D4k/n1sA9/SDDAMczPfxTvDMDfCImKYhrCIGKYhLCKGaQiLiGEaoldESYCu1UWQiHQtMUZnrznFqfoN2o5HsCxLBNlGU1uZ/xl61nVoFVGyXMBwDSyWdzbqSPz9Dwzma6zXFN7Ra4ky5qGpE5JGESVYLgw4YwfGYpmlVGhGF7P8KBJ5RIKgK2d/C91yyhceIKi8wyiW+X2E2GDSUa8njpUpfZ/0KgaejglnWdlRtXut7RLFnt0CRhP0LHagL1u1sJ2l3nCVR1dDMxXRHEp7s61MpKbppTJZsUqH5jB7lXEzNWUjeR21rK4+cVh20Lc3S+uqbmde1seu3UV7x+xQudR2Sso+tunMO/Y5mH8BPW81SLR5ovhtAsOx87jtuAgjOcvGiMI2Xn6Kad524BaxgnIvQl7kA39KT9HG4FfRXlFHLbuUmr43C9StNlu992wZ5wP9zJZuoHjSE3ZcbTvZA4TUh9XBZLPB5za/itEMLeMlmkSkDo4s9CkRZblnoEE4fcY834f4u+LSgo3xeo4BJng7Z/zNtmcCK/dea4yF1hh9qAIi9IgojhC6fjkwKPhuiMIZ/cBze1MdNtC1RazAeELuJ/bzs72NrJMEU4TtF0iHcjk2HHev75vaOcLVtu/dC0Y7+wIitIgoztyQK5ZykmpJ10LvdYBst194qQjVrE1LnbB/mJ/ThvH5Oy/rTIDBa68YsEIYhwcLxGGZPfZhyL7pBK5sRyE/mhdelJZZhn/eQ9xk+969sEbnvTXzZdQJiHiQX3HTCdYUz/N7PGq+Z9sZQuMRN8M8Bvx/IoZpCHsihmkIi4hhGsIiYphGAH8Bow45DQAun74AAAAASUVORK5CYII='
    },
]);

const radio1 = ref('1')
const radioCondition = ref('1')

// 裁剪
const isShowModal = ref(false);

const handlingImage = ref('');
const handlingImageID = ref(0);

const modifyImage = (eventCard) => {
    console.log('修改图片', eventCard);
    handlingImage.value = eventCard.image;
    handlingImageID.value = eventCard.id;
    isShowModal.value = true;
    console.log(handlingImage.value);
};

const isShowConditionModal = ref(false);
const conditionControl = (eventCard) => {
    console.log('条件控制', eventCard);
    isShowConditionModal.value = true;
};

const conditionImageSelect = (conditionCard) => {
    console.log('确认图片', conditionCard);
    isShowConditionModal.value = false;
};


const clearCropper = () => {
    cropper.clear();
};

const resetCropper = () => {
    cropper.reset();
};

const getCropperDataURL = () => {
    const data = cropper.getDataURL();
    eventCardList.value = eventCardList.value.map((item) => {
        if (item.id === handlingImageID.value) {
            item.image = data;
        }
        return item;
    });
    console.log(data);
    isShowModal.value = false;
};



</script>
<template>
    <div v-if="selectedScript">
        <el-button v-if="isRecording" @click="stopRecord">结束录制（F11）</el-button>
        <el-button v-else @click="startRecord">开始录制（F11）</el-button>
        <div v-if="selectedScript">
            <p>当前选中的脚本是：{{ selectedScript.name }}</p>
        </div>
        <div class="eventCards">
            <div v-for="eventCard in displayedSteps" class="eventCard">
                <el-col>
                    <el-row :span="12" class="eventImage">
                        <img :src="eventCard.image" alt="" class="button_img" />
                    </el-row>
                    <el-row :span="12">
                        <el-col :span="12">
                            <el-row :span="12">
                                <el-button @click="modifyImage(eventCard)" size="small">修改</el-button>
                            </el-row>
                            <el-row :span="12">
                                <el-button @click="conditionControl(eventCard)" size="small">条件控制</el-button>
                            </el-row>
                        </el-col>
                        <el-col :span="12">
                            <el-row :span="12" class="coordinate">
                                {{ eventCard.coordinates[0] }} - {{ eventCard.coordinates[1] }}
                            </el-row>
                            <el-row :span="12">
                                <el-radio-group v-model="radio1" class="ml-4">
                                    <el-radio value="1" size="small">使用图片</el-radio>
                                    <el-radio value="2" size="small">使用坐标</el-radio>
                                </el-radio-group>
                            </el-row>
                        </el-col>
                    </el-row>
                </el-col>
            </div>
        </div>


        <el-dialog v-model="isShowModal" title="图片裁剪" width="500" :before-close="handleClose">
            <el-row :span="5">
                <el-col :span="5">
                    <el-button @click="isShowModal = false">取消</el-button>
                </el-col>
                <el-col :span="5">
                    <el-button @click="clearCropper">清除</el-button>
                </el-col>
                <el-col :span="5">
                    <el-button @click="resetCropper">重置</el-button>
                </el-col>
                <el-col :span="5">
                    <el-button type="primary" @click="getCropperDataURL">裁剪</el-button>
                </el-col>
            </el-row>
            <el-row>
                <VuePictureCropper :boxStyle="{
                    width: '100%',
                    height: '100%',
                    backgroundColor: '#f8f8f8',
                    margin: 'auto',
                }" :img="handlingImage" :options="{
                    viewMode: 1,
                    dragMode: 'crop',
                }" @ready="ready" />
            </el-row>
        </el-dialog>

        <el-dialog v-model="isShowConditionModal" title="条件控制" width="500" :before-close="handleClose">
            <el-radio-group v-model="radioCondition" class="ml-4">
                <el-radio value="1" size="large">顺序执行</el-radio>
                <el-radio value="2" size="large">条件判断</el-radio>
                <el-radio value="3" size="large">循环</el-radio>
            </el-radio-group>
            <div class="conditionCards">
                <div v-for="conditionCard in conditionCardList" :key="conditionCard.id" class="conditionCard">
                    <div class="conditionImageDiv">
                        <img :src="conditionCard.image" alt="" class="conditionImage" />
                    </div>
                    <button @click="conditionImageSelect(conditionCard)">确认图片</button>
                </div>
            </div>
        </el-dialog>
    </div>

</template>
<style>
.eventCards,
.conditionCards {
    display: flex;
    flex-shrink: 0;
    overflow-x: auto;
    padding: 0;
}

.eventCard,
.conditionCard {
    margin: 20px;
    background-color: #f2f2f2;
    border-radius: 4px;
    border: 1px solid #e9e9e9;
    height: 200px;
    width: 200px;
    flex: 0 0 auto;
}


.conditionImageDiv {
    height: 200px;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}


.eventImage {
    height: 100px;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border-bottom: 2px dashed #372199;

}

.button_img {
    object-fit: contain;
    object-position: center;
    ;

}

.coordinate {
    text-align: center;
    font-size: 12px;
    color: #666;
    display: flex;
    flex-direction: row;
    width: 100%;
}
</style>