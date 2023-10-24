<template>
  <div class="detail">
    <div class="detail-content">
      <div class="detail-header">
        <span class="detail-title">详情</span>
        <span class="detail-close zy-svg" @click="close">
          <SvgIcon name="close"></SvgIcon>
        </span>
      </div>
      <div class="detail-body" v-show="!detailInfo.loading">
        <div class="info">
          <div class="info-left">
            <img :src="info.pic" alt="">
          </div>
          <div class="info-right">
            <div class="name">{{info.name}}</div>
            <div class="director" v-show="info.director">导演: {{info.director}}</div>
            <div class="actor" v-show="info.actor">主演: {{info.actor}}</div>
            <div class="type" v-show="info.type">类型: {{info.type}}</div>
            <div class="area" v-show="info.area">地区: {{info.area}}</div>
            <div class="lang" v-show="info.lang">语言: {{info.lang}}</div>
            <div class="year" v-show="info.year">上映: {{info.year}}</div>
            <div class="last" v-show="info.last">更新: {{info.last}}</div>
            <div class="note" v-show="info.note">备注: {{info.note}}</div>
            <div class="rate" v-show="info.rate">豆瓣评分: {{info.rate}}</div>
          </div>
        </div>
        <div class="operate">
          <span @click="playEvent(detailInfo.selectedEpisode)">播放</span>
          <span @click="starEvent(info)">收藏</span>
          <span @click="downloadEvent">下载</span>
          <span @click="doubanLinkEvent">豆瓣</span>
        </div>
        <div class="desc" v-show="info.des">{{info.des}}</div>
        <div class="m3u8" v-if="detailInfo.videoFullList.length > 1">
          <div class="box">
            <span v-bind:class="{ selected: i.flag === detailInfo.videoFlag }" v-for="(i, j) in detailInfo.videoFullList" :key="j" 
              @click="updateVideoList(i)">
              {{i.flag}}
            </span>
          </div>
        </div>
        <div class="m3u8">
          <div class="box">
            <span v-bind:class="{ selected: (j === detailInfo.selectedEpisode || j === detailInfo.moveOn) }" v-for="(i, j) in detailInfo.videoList" :key="j" 
              @click="playEvent(j)" @mouseenter="() => { detailInfo.moveOn = j }">
              {{ ftName(i, j) }}
            </span>
          </div>
        </div>
        <div class="m3u8">
          <div class="show-picture" v-show="info.recommendations && info.recommendations.length > 0">
            <span>喜欢这部电影的人也喜欢 · · · · · ·</span>
            <Waterfall :list="info.recommendations" :gutter="20" :width="240"
              :breakpoints="{
              1200: { //当屏幕宽度小于等于1200
                rowPerView: 4,
              },
              800: { //当屏幕宽度小于等于800
                rowPerView: 3,
              },
              500: { //当屏幕宽度小于等于500
                rowPerView: 2,
              }
            }"
            animationEffect="fadeIn"
            backgroundColor="rgba(0, 0, 0, 0)">
              <template #item="{item}">
                <div class="card">
                  <div class="img">
                    <ImageLazy style="width: 100%" :url="item.pic" @click="detailEvent(item)"/>
                    <div class="operate">
                      <div class="operate-wrap">
                        <span class="o-play" @click="playRecommendationEvent(item)">播放</span>
                        <span class="o-star" @click="starEvent(item)">收藏</span>
                      </div>
                    </div>
                  </div>
                  <div class="name">{{item.name}}</div>
                  <div class="info">
                    <span>{{item.area}}</span>
                    <span>{{item.year}}</span>
                    <span>{{item.note}}</span>
                    <span>{{item.type}}</span>
                  </div>
                </div>
              </template>
            </Waterfall>
          </div>
        </div>
      </div>
      <div class="detail-mask zy-loading" v-show="detailInfo.loading">
        <div class="loader"></div>
      </div>
    </div>
  </div>
</template>
<script>
import { defineComponent, reactive, onMounted, watch } from 'vue';
import { Waterfall } from 'vue-waterfall-plugin-next';
import 'vue-waterfall-plugin-next/dist/style.css';
import ImageLazy from '@/components/ImageLazy.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useDownloadStore } from "@/store/download";
import { storeToRefs } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import moviesApi from '@/api/movies';
import doubanApi from '@/api/douban';
import { _ } from 'lodash';

export default defineComponent({
  name: 'detail',
  components: {
      Waterfall,
      ImageLazy,
      SvgIcon
  },
  setup() {
    const coreStore = useCoreStore();
    const { view, playInfo } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { getSiteByKey, getMoviesDetailCacheByKey } = movieStore;
    const { movieDetailCache, detail } = storeToRefs(movieStore);

    const downloadStore = useDownloadStore();
    const { downloadMovie } = downloadStore; 

    const detailInfo = reactive({
      loading: false,
      videoFlag: "",
      selectedEpisode: 0,
      videoList: [],
      videoFullList: [],
      moveOn: 0,
    })

    const info = reactive({});

    const getDetailInfo = async () => {
      detailInfo.loading = true;
      const id = detail.value.ids.toString();
      const cacheKey = detail.value.siteKey + '@' + id
      let historyStr = await invoke("get_history_by_uq", { siteKeyStr: detail.value.siteKey, idsStr: id })
      if (historyStr) {
        const history = JSON.parse(historyStr);
        detailInfo.videoFlag = history.videoFlag;
        detailInfo.selectedEpisode = history.index;
      } else {
        detailInfo.selectedEpisode = 0;
      }
      getMoviesDetailCacheByKey(detail.value.siteKey, id).then((res) => {
        handlerDetailData(res, cacheKey);
      });
    }

    const handlerDetailData = async (res, cacheKey) => {
      if (res) {
        _.assign(info, res);
        info.rate = res.rate || '';
        info.recommendations = res.recommendations || [];
        detailInfo.videoFlag = detailInfo.videoFlag || res.fullList[0].flag
        detailInfo.videoList = res.fullList[0].list
        detailInfo.videoFullList = res.fullList
        detailInfo.loading = false
        if (!info.rate) {
          await getDoubanRate()
          movieDetailCache.value[cacheKey] = info
        }
      }
    }
    
    const getDoubanRate = async () => {
      const name = info.name.trim()
      const year = info.year
      info.rate = await doubanApi.doubanRate(name, year);
      const recommendations = await doubanApi.doubanRecommendations(name, year);
      const siteInfo = getSiteByKey(detail.value.siteKey);
      if (recommendations) {
        info.recommendations = []
        recommendations.forEach(element => {
          moviesApi.searchFirstDetail(siteInfo, element).then(detailRes => {
            if (detailRes) {
              info.recommendations.push(detailRes)
            }
          })
        })
      }
    }
    
    const close = () => {
      detail.value.show = false
    }

    const ftName = (e, n) => {
      const num = e.split('$')
      if (num.length > 1) {
        return e.split('$')[0]
      } else {
        return `第${(n + 1)}集`
      }
    }
    
    const detailEvent = (info) => {
      detail.value = {
        show: true,
        siteKey: detail.value.siteKey,
        ids: info.id.toString(),
      }
      getDetailInfo();
    }
    
    const updateVideoList = async (e) => {
      detailInfo.videoFlag = e.flag
      detailInfo.videoList = e.list
      
      let historyStr = await invoke("get_history_by_uq", { siteKeyStr: detail.value.siteKey, idsStr: detail.value.ids.toString() })
      if (historyStr) {
        let history = JSON.parse(historyStr);
        history.videoFlag = e.flag;
        await invoke("save_history", {history: history})
      }
    }
    
    const playEvent = async (n) => {
      playInfo.value.playType = "onlineMovie";
      playInfo.value.name = info.name;
      playInfo.value.movie.siteKey = detail.value.siteKey;
      playInfo.value.movie.ids = info.id;
      playInfo.value.movie.index = n;
      playInfo.value.movie.videoFlag = detailInfo.videoFlag;
      playInfo.value.movie.playMode = "local";
      detail.value.show = false;
      view.value = "Play";
    }
    
    const playRecommendationEvent = async (e) => {
      let historyStr = await invoke("get_history_by_uq", { siteKey: detail.value.siteKey, ids: e.id });
      video.value.playType = "movies";
      if (historyStr) {
        const history = JSON.parse(historyStr);
        info.value = {
          siteKey: detail.value.siteKey,
          ids: history.ids,
          name: e.name,
          index: history.index,
          videoFlag: history.videoFlag,
        }
      } else {
        info.value = {
          siteKey: detail.value.siteKey,
          ids: e.ids,
          name: e.name,
          index: 0,
          videoFlag: "",
        }
      }
      detail.value.show = false;
      view.value = "Play";
    }
    
    const starEvent = async (info) => {
      const starStr = await invoke("get_star_by_uq", { starSiteKey: detail.value.siteKey, starIds: info.id });
      if (starStr) {
        let star = JSON.parse(starStr);
        star.hasUpdate = star.lastUpdateTime != info.detail.last ? '1' : '0';
        star.lastUpdateTime = info.detail.last;
        star.doubanRate = info.rate;
        await invoke("save_star", { star: star });
        ElMessage.success('收藏更新成功');
      } else {
        const star = {
          id: null,
          name: info.name,
          ids: info.id.toString(),
          siteKey: detail.value.siteKey,
          movieType: info.type,
          year: info.year + "年",
          note: info.note,
          doubanRate: info.rate,
          hasUpdate: "0",
          lastUpdateTime: info.last,
          position: 0.0,
          pic: info.pic,
          area: info.area
        }
        await invoke("save_star", { star: star });
        ElMessage.success('收藏成功');
      }
    }

    const downloadEvent = () => {
      downloadMovie(getSiteByKey(detail.value.siteKey), detail.value.ids);
    };

    onMounted(() => {
      getDetailInfo();
    })

    return {
      detailInfo,
      info,
      close,
      ftName,
      detailEvent,
      updateVideoList,
      playRecommendationEvent,
      playEvent,
      starEvent,
      downloadEvent,
    }
  }
});
// export default {
//   methods: {
//     doubanLinkEvent () {
//       const name = this.info.name.trim()
//       const year = this.info.year
//       zy.doubanLink(name, year).then(link => {
//         const open = require('open')
//         open(link)
//       })
//     },
// }
</script>
<style lang="scss" scoped>

.detail {
  position: absolute;
  top: 38px;
  left: 80px;
  right: 20px;
  bottom: 0;
  border-radius: 10px;
  width: calc(100% - 100px);
  height: calc(100% - 58px);
  z-index: 888;
  .detail-content {
    height: 100%;
    padding: 0 30px;
    position: relative;
    .detail-header {
      width: 100%;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .detail-title {
        margin-left: 10px;
        font-size: 16px;
      }
      .detail-close {
        cursor: pointer;
      }
    }
  }
  .detail-body {
    width: 100%;
    height: calc(100% - 80px);
    overflow-y: auto;
    &::-webkit-scrollbar {
      display: none; /* Chrome Safari */
      // 或者 width: 0;
    }
    .info {
      padding: 10px;
      display: flex;
      flex-wrap: wrap;
      align-items: flex-start;
      justify-content: flex-start;
      border: 1px solid;
      height: auto;
      border-radius: 5px;
      margin-bottom: 10px;
      .info-left {
        width: 200px;
        height: 100%;
        img {
          width: 100%;
          height: auto;
        }
      }
      .info-right {
        flex: 1;
        margin-left: 20px;
        .name {
          font-size: 20px;
          margin-bottom: 10px;
          font-weight: bold;
        }
        .director,
        .actor,
        .type,
        .area,
        .lang,
        .year,
        .last,
        .note {
          font-size: 14px;
          line-height: 26px;
        }
        .rate {
          font-size: 16px;
          line-height: 26px;
          font-weight: bolder;
        }
      }
    }
    .operate {
      border: 1px solid;
      padding: 10px;
      border-radius: 5px;
      margin-bottom: 10px;
      span {
        margin-right: 20px;
        font-size: 14px;
        cursor: pointer;
        user-select: none;
      }
    }
    .desc {
      border: 1px solid;
      padding: 10px;
      margin: 0 10px 10px;
      border-radius: 2px;
      font-size: 14px;
      line-height: 20px;
    }
    .m3u8 {
      display: flex;
      border: 1px solid;
      border-radius: 5px;
      align-content: center;
      border-radius: 5px;
      margin-bottom: 10px;
      .box {
        width: 100%;
        display: flex;
        flex-wrap: wrap;
        padding: 0 10px 10px 10px;
        span {
          display: inline-block;
          font-size: 12px;
          border: 1px solid;
          border-radius: 2px;
          cursor: pointer;
          height: 20px;
          padding: 8px 24px;
          margin: 10px 10px 0 0;
        }
        .selected {
          display: inline-block;
          font-size: 12px;
          border: 1px solid;
          border-radius: 2px;
          cursor: pointer;
          padding: 8px 24px;
          margin: 10px 10px 0 0;
        }
      }
    }
  }
  .detail-mask {
    position: absolute;
    top: 50px;
    left: 0;
    height: calc(100% - 50px);
    display: flex;
    justify-content: center;
    align-items: center;
    .loader {
      font-size: 8px;
      width: 1em;
      height: 1em;
      border-radius: 50%;
      position: relative;
      text-indent: -9999em;
      animation: load4 1.3s infinite linear;
      transform: translateZ(0);
    }
    @keyframes load4 {
      0%,
      100% {
        box-shadow: 0 -3em 0 0.2em, 2em -2em 0 0em, 3em 0 0 -1em, 2em 2em 0 -1em,
          0 3em 0 -1em, -2em 2em 0 -1em, -3em 0 0 -1em, -2em -2em 0 0;
      }
      12.5% {
        box-shadow: 0 -3em 0 0, 2em -2em 0 0.2em, 3em 0 0 0, 2em 2em 0 -1em,
          0 3em 0 -1em, -2em 2em 0 -1em, -3em 0 0 -1em, -2em -2em 0 -1em;
      }
      25% {
        box-shadow: 0 -3em 0 -0.5em, 2em -2em 0 0, 3em 0 0 0.2em, 2em 2em 0 0,
          0 3em 0 -1em, -2em 2em 0 -1em, -3em 0 0 -1em, -2em -2em 0 -1em;
      }
      37.5% {
        box-shadow: 0 -3em 0 -1em, 2em -2em 0 -1em, 3em 0em 0 0, 2em 2em 0 0.2em,
          0 3em 0 0em, -2em 2em 0 -1em, -3em 0em 0 -1em, -2em -2em 0 -1em;
      }
      50% {
        box-shadow: 0 -3em 0 -1em, 2em -2em 0 -1em, 3em 0 0 -1em, 2em 2em 0 0em,
          0 3em 0 0.2em, -2em 2em 0 0, -3em 0em 0 -1em, -2em -2em 0 -1em;
      }
      62.5% {
        box-shadow: 0 -3em 0 -1em, 2em -2em 0 -1em, 3em 0 0 -1em, 2em 2em 0 -1em,
          0 3em 0 0, -2em 2em 0 0.2em, -3em 0 0 0, -2em -2em 0 -1em;
      }
      75% {
        box-shadow: 0em -3em 0 -1em, 2em -2em 0 -1em, 3em 0em 0 -1em,
          2em 2em 0 -1em, 0 3em 0 -1em, -2em 2em 0 0, -3em 0em 0 0.2em,
          -2em -2em 0 0;
      }
      87.5% {
        box-shadow: 0em -3em 0 0, 2em -2em 0 -1em, 3em 0 0 -1em, 2em 2em 0 -1em,
          0 3em 0 -1em, -2em 2em 0 0, -3em 0em 0 0, -2em -2em 0 0.2em;
      }
    }
  }
}
</style>
