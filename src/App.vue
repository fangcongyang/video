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
      <Download v-show="view === 'Download'" />
    </div>
    <transition name="slide">
      <Detail v-if="detail.show" />
    </transition>
  </div>
</template>

<script>
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useDownloadStore } from "@/store/download";
import { DownloadBus } from "@/business/download";
import { defineComponent, onMounted, onBeforeMount, watch, onBeforeUnmount, ref } from "vue";
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
import Download from "@/pages/Download.vue";
import { useDark, useToggle } from "@vueuse/core";
import { _ } from "lodash";

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
    EditSites,
    Download
  },
  setup() {
    const coreStore = useCoreStore();
    const { getSystemConf } = coreStore;
    const { view, shiftDown, systemConf } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { detail } = storeToRefs(movieStore);

    const isDark = useDark();
    const toggleDark = useToggle(isDark);

    const downloadWebsocketNum = ref(2);

    const downloadStore = useDownloadStore();
    const { downloadList } = storeToRefs(downloadStore);

    const initTheme = () => {
      if (systemConf.value.theme == "theme-dark") {
        isDark.value = false;
      } else {
        isDark.value = true;
      }
      toggleDark();
    }

    const initDownloadWebsocket = () => {
      for (var i = 0; i < downloadWebsocketNum.value; i++) {
        const downloadBus = new DownloadBus();
        downloadBus.updateDownloadInfoEvent = (download) => {
          let downloading = _.find(downloadList.value, { 'id': download.id});
          downloading.count = download.count;
          downloading.download_count = download.download_count;
          downloading.status = download.status;
          downloading.download_status = download.download_status;
        }
      }
    };

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
      
      initDownloadWebsocket();
    });

    onBeforeUnmount(() => {
    })

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
    padding: 0 20Px 20Px;
  }

  .active-page {
    transform: translate(0, 0);
  }

  .not-active-page{
    transform: translate(100vh, 100vh);
  }
}
</style>
