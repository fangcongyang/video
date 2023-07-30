<template>
  <div class="frame">
    <span
      :title="data.isAlwaysOnTop ? '取消置顶' : '置顶'"
      class="top"
      @click="handleAlwaysTop"
    >
      <SvgIcon name="wintool-ontop" :style-var="{ width: '8px', height: '14px' }" :color="data.isAlwaysOnTop ? '#555555' : '#ffffff'"></SvgIcon>
    </span>
    <span v-if="minimizable" class="min" @click="handleWinMin" title="最小化">
      <SvgIcon name="wintool-min" :style-var="{ width: '8px', height: '14px' }" color="#ffffff"></SvgIcon>
    </span>
    <span
      v-if="maximizable && data.isResizable"
      class="max"
      @click="handleWinMax2Min"
      :title="data.isMaximized ? '还原' : '最大化'"
    >
      <SvgIcon name="wintool-max" :style-var="{ width: '8px', height: '14px' }" color="#ffffff"></SvgIcon>
    </span>
    <span v-if="closable" class="close" @click="handleWinClose" title="关闭">
      <SvgIcon name="wintool-close" :style-var="{ width: '8px', height: '14px' }" color="#ffffff"></SvgIcon>
    </span>
  </div>
</template>
<script setup>
import { onMounted, reactive } from "vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useCoreStore } from "@/store";
import { storeToRefs } from "pinia";
import SvgIcon from '@/components/SvgIcon.vue';

const coreStore = useCoreStore();
const { systemConf } = storeToRefs(coreStore);

const data = reactive({
  isMaximized: false,
  isResizable: true,
  isAlwaysOnTop: false,
  fullscreen: false,
});

const props = defineProps({
  minimizable: {
    type: Boolean,
    required: true,
  },
  maximizable: {
    type: Boolean,
    required: true,
  },
  closable: {
    type: Boolean,
    required: false,
    default: true,
  },
});

onMounted(async () => {
  data.isMaximized = await appWindow.isMaximized();
  data.isResizable = await appWindow.isResizable();
  listen("tauri://resize", async () => {
    data.isMaximized = await appWindow.isMaximized();
  });
});

// 最小化
const handleAlwaysTop = async () => {
  data.isAlwaysOnTop = !data.isAlwaysOnTop;
  await appWindow.setAlwaysOnTop(data.isAlwaysOnTop);
};

// 最小化
const handleWinMin = async () => {
  await appWindow.minimize();
};
// 最大化/还原
const handleWinMax2Min = async () => {
  const resizable = await appWindow.isResizable()
  if(!resizable) return
  await appWindow.setFullscreen(!data.fullscreen);
  if (data.fullscreen) {
    if (await appWindow.innerSize().width != systemConf.value.mainWidth) {
      await appWindow.setSize(new LogicalSize(systemConf.value.mainWidth, systemConf.value.mainHeight))
      await appWindow.center();
    }
    await appWindow.unmaximize();
  }

  data.fullscreen = !data.fullscreen;
};
// 关闭
const handleWinClose = async () => {
  await invoke("exist_app", {});
};
</script>
<style lang="scss" scoped>
.frame {
  width: 100%;
  height: 40px;
  display: flex;
  user-select: none;
  align-items: center;
  justify-content: flex-end;
  -webkit-app-region: drag;
  span {
    width: 14px;
    height: 14px;
    cursor: pointer;
    margin-left: 10px;
    border-radius: 50%;
    text-align: center;
    line-height: 14px;
    display: inline-block;
    -webkit-app-region: no-drag;
  }
}
</style>
