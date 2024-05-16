<script setup>
import { ref } from 'vue';
import Aside from "./components/Aside.vue";
import Main from "./components/Main.vue";

const selectedScript = ref(null);

const handleScriptSelected = (script) => {
  selectedScript.value = script;
};
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header>EasyAutoGUI</el-header>
      <el-container>
        <el-aside width="100px">
          <Aside @scriptSelected="handleScriptSelected" />
        </el-aside>
        <el-main>
          <Main :selectedScript="selectedScript" />
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style>
.common-layout {
  height: 100vh;
}
</style>
