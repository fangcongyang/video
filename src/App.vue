<template>
  <div id="main" :class="systemConf.theme">
    <Aside></Aside>
    <div class="main-body">
      <WinTool data-tauri-drag-region :minimizable="true" :maximizable="true" />
      <Movies v-show="view === 'Movies'" />
      <IPTV v-show="view === 'IPTV'" />
      <Play v-show="view === 'Play'" />
      <Star v-show="view === 'Star'" />
      <History v-show="view === 'History'" />
      <Setting v-show="view === 'Setting'" />
    </div>
    <transition name="slide">
      <Detail v-if="detail.show" />
    </transition>
  </div>
</template>

<script>
import { useCoreStore } from "@/store";
import { useMoviesStore } from "@/store/movies";
import { defineComponent, onMounted, onBeforeMount, watch } from "vue";
import Aside from "@/components/Aside.vue";
import { storeToRefs } from "pinia";
import WinTool from "@/components/WinTool.vue";
import Movies from "@/pages/Movies.vue";
import IPTV from "@/pages/IPTV.vue";
import Play from "@/pages/Play.vue";
import Star from "@/pages/Star.vue";
import History from "@/pages/History.vue";
import Detail from "@/pages/Detail.vue";
import Setting from "@/pages/Setting.vue";
import { useDark, useToggle } from "@vueuse/core";

export default defineComponent({
  components: {
    Aside,
    WinTool,
    Movies,
    IPTV,
    Play,
    History,
    Detail,
    Setting,
    Star,
  },
  setup() {
    const coreStore = useCoreStore();
    const { getSystemConf } = coreStore;
    const { view, shiftDown, systemConf } = storeToRefs(coreStore);

    const moviesStore = useMoviesStore();
    const { detail } = storeToRefs(moviesStore);

    const isDark = useDark();
    const toggleDark = useToggle(isDark);

    const initTheme = () => {
      if (systemConf.value.theme == "theme-dark") {
        isDark.value = false;
      } else {
        isDark.value = true;
      }
      toggleDark();
    }

    onBeforeMount(async () => {
      await getSystemConf();
      initTheme();
    });

    onMounted(() => {
      addEventListener("keydown", (event) => {
        if (event.key === 16) shiftDown.value = true;
      });
      addEventListener("keyup", (event) => {
        if (event.key === 16) shiftDown.value = false;
      });
    });

    watch(() => systemConf.value.theme, 
    () => {
      initTheme();
    });

    return {
      systemConf,
      view,
      detail,
    };
  },
});
</script>
<style lang="scss">
#main {
  overflow: hidden;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;

  .main-body {
    flex: 1;
    height: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
    flex-direction: column;
    padding: 0 20px 20px;
  }
}
</style>
