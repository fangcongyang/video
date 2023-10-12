<template>
  <div class="aside">
    <span
      :class="[view === 'Movie' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('Movie')"
    >
      <SvgIcon name="movie"></SvgIcon>
    </span>
    <span
      :class="[view === 'IPTV' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('IPTV')"
    >
      <SvgIcon name="iptv"></SvgIcon>
    </span>
    <span
      :class="[view === 'Play' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('Play')"
    >
      <SvgIcon name="play"></SvgIcon>
    </span>
    <span
      :class="[view === 'Star' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('Star')"
    >
      <SvgIcon name="star"></SvgIcon>
    </span>
    <span
      :class="[view === 'History' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('History')"
    >
      <SvgIcon name="history"></SvgIcon>
    </span>
    <span
      :class="[view === 'Setting' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('Setting')"
    >
      <SvgIcon name="setting"></SvgIcon>
    </span>
  </div>
</template>
<script>
import { useCoreStore } from "../store";
import { defineComponent, onMounted } from "vue";
import { storeToRefs } from "pinia";
import SvgIcon from "@/components/SvgIcon.vue";

export default defineComponent({
  name: "Aside",
  components: {
    SvgIcon,
  },
  setup() {
    const coreStore = useCoreStore();
    const { view } = storeToRefs(coreStore);

    const changeView = (viewName) => {
      view.value = viewName;
    };

    onMounted(() => {});

    return {
      changeView,
      view,
    };
  },
});
</script>
<style lang="scss" scoped>
.aside {
  width: 60px;
  height: 100%;
  display: flex;
  user-select: none;
  align-items: center;
  flex-direction: column;
  justify-content: center;
  -webkit-app-region: drag;
  span {
    width: 60px;
    height: 60px;
    cursor: pointer;
    -webkit-app-region: no-drag;
  }
}

.dh-svg  {
  display: flex;
  justify-content: center;
  align-items: center;
  svg {
    width: 24px;
    height: 24px;
    stroke-width: 1;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
}

</style>
