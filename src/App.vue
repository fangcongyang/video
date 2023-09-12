<template>
  <div id="main" :class="systemConf.theme">
    <Aside></Aside>
    <div class="main-body">
      <WinTool data-tauri-drag-region :minimizable="true" :maximizable="true" />
      <Movie :class="view === 'Movie' ? 'active-page' : 'not-active-page'" />
      <IPTV v-show="view === 'IPTV'" />
      <Play v-show="view === 'Play'" />
      <Star :class="view === 'Star' ? 'active-page' : 'not-active-page'" />
      <History :class="view === 'History' ? 'active-page' : 'not-active-page'" />
      <Setting v-show="view === 'Setting'" />
      <EditSites v-show="view === 'EditSites'" />
    </div>
    <transition name="slide">
      <Detail v-if="detail.show" />
    </transition>
  </div>
</template>

<script>
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { defineComponent, onMounted, onBeforeMount, watch } from "vue";
import Aside from "@/components/Aside.vue";
import { storeToRefs } from "pinia";
import WinTool from "@/components/WinTool.vue";
import Movie from "@/pages/Movie.vue";
import IPTV from "@/pages/IPTV.vue";
import Play from "@/pages/Play.vue";
import Star from "@/pages/Star.vue";
import History from "@/pages/History.vue";
import Detail from "@/pages/Detail.vue";
import Setting from "@/pages/Setting.vue";
import EditSites from "@/pages/EditSites.vue";
import { useDark, useToggle } from "@vueuse/core";

export default defineComponent({
  components: {
    Aside,
    WinTool,
    Movie,
    IPTV,
    Play,
    History,
    Detail,
    Setting,
    Star,
    EditSites
  },
  setup() {
    const coreStore = useCoreStore();
    const { getSystemConf } = coreStore;
    const { view, shiftDown, systemConf } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { detail } = storeToRefs(movieStore);

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

  .active-page {
    transform: translate(0, 0);
  }

  .not-active-page{
    transform: translate(100vh, 100vh);
  }
}
</style>
