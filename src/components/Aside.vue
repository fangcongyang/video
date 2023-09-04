<template>
  <div class="aside">
    <span
      :class="[view === 'Movie' ? 'active ' : ''] + 'dh-svg'"
      @click="changeView('Movie')"
    >
      <SvgIcon name="movie"></SvgIcon>
    </span>
    <!-- <span :class="[view === 'Recommendation' ? 'active ': ''] + 'dh-svg'" @click="changeView('Recommendation')">
      <svg role="img" xmlns="http://www.w3.org/2000/svg" width="48px" height="48px" viewBox="0 0 24 24" aria-labelledby="thumbUpIconTitle" stroke="#2329D6" stroke-width="1" stroke-linecap="square" stroke-linejoin="miter" fill="none" color="#2329D6">
      <title id="thumbUpIconTitle">影视推荐</title>
      <path d="M8,8.73984815 C8,8.26242561 8.17078432,7.80075162 8.4814868,7.43826541 L13.2723931,1.84887469 C13.7000127,1.34998522 14.4122932,1.20614658 15,1.5 C15.5737957,1.78689785 15.849314,2.45205792 15.6464466,3.06066017 L14,8 L18.6035746,8 C18.7235578,8 18.8432976,8.01079693 18.9613454,8.03226018 C20.0480981,8.22985158 20.7689058,9.27101818 20.5713144,10.3577709 L19.2985871,17.3577709 C19.1256814,18.3087523 18.2974196,19 17.3308473,19 L10,19 C8.8954305,19 8,18.1045695 8,17 L8,8.73984815 Z"/>
      <path d="M4,18 L4,9"/>
      </svg>
    </span> -->
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

.dh-svg {
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
