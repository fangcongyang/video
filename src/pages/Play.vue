<template>
  <div class="play">
    <div class="box">
      <div class="title">
        <span v-if="moviesInfo.moviesList.length > 1"
          >『第 {{ playInfo.movie.index + 1 }} 集』</span
        >{{ playInfo.name }}
        <span class="right" @click="closePlayerAndInit">
          <svg-icon name="close"></svg-icon>
        </span>
      </div>
      <div class="player" v-show="playInfo.movie.playMode === 'local' || playInfo.movie.playMode === 'localFile'">
        <div id="dpplayer"></div>
      </div>
      <div
        class="iframePlayer"
        v-if="playInfo.movie.playMode === 'online'"
        style="width: 100%; height: 100%"
      >
        <iframe
          id="iframePlayer"
          v-bind:src="playInfo.movie.onlineUrl"
          width="100%"
          height="100%"
          frameborder="0"
          scrolling="no"
          allow="autoplay;fullscreen"
        >
        </iframe>
      </div>

      <div
        class="more"
        v-if="playInfo.playType === 'iptv'"
        :key="playInfo.playType"
      >
        <span
          class="zy-svg"
          @click="
            iptvInfo.showChannelGroupList = !iptvInfo.showChannelGroupList
          "
          :class="iptvInfo.showChannelGroupList ? 'active' : ''"
        >
          <svg-icon name="play-play_list" size="22" title="频道列表"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="otherEvent"
          :class="playerInfo.right.type === 'sources' ? 'active' : ''"
        >
          <svg-icon name="play-website" size="22" title="换源"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="showShortcutEvent"
          :class="playerInfo.right.type === 'shortcut' ? 'active' : ''"
        >
          <svg-icon name="play-guide" size="22" title="快捷键指南"></svg-icon>
        </span>
      </div>
      <div
        class="more"
        v-if="playInfo.movie.playMode === 'localFile'"
        :key="playInfo.playType"
        >
        <span
          class="zy-svg"
          @click="localMoviesListEvent"
          :class="playerInfo.right.type === 'localMovieList' ? 'active' : ''"
          v-show="downloadList.length > 0"
        >
          <svg-icon name="play-play_list" size="22" title="播放列表"></svg-icon>
        </span>
      </div>
      <div
        class="more"
        v-if="playInfo.playType == 'onlineMovie'"
        :key="playInfo.playType"
      >
        <span
          class="zy-svg"
          @click="otherEvent"
          v-show="playInfo.name != ''"
          :class="playerInfo.right.type === 'other' ? 'active' : ''"
        >
          <svg-icon name="play-website" size="22" title="换源"></svg-icon>
        </span>
        <span
          class="zy-svg"
          v-if="movieParseUrlInfo.vipPlay"
          @click="() => {
            playerInfo.right.type = 'movieParseSources';
            playerInfo.right.show = true;
          }"
          :class="playerInfo.right.type === 'movieParseSources' ? 'active' : ''"
        >
          <svg-icon name="play-line" size="22" title="切换线路"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="moviesListEvent"
          :class="playerInfo.right.type === 'movieList' ? 'active' : ''"
          v-show="moviesInfo.moviesList.length > 0"
        >
          <svg-icon name="play-play_list" size="22" title="播放列表"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="historyEvent"
          :class="playerInfo.right.type === 'history' ? 'active' : ''"
        >
          <svg-icon name="play-history" size="22" title="历史记录"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="starEvent"
          :class="moviesInfo.isStar ? 'active' : ''"
          v-show="moviesInfo.moviesList.length > 0 || moviesInfo.isStar"
        >
          <svg-icon name="play-star" size="22" title="收藏"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="detailEvent"
          v-show="moviesInfo.moviesList.length > 0"
        >
          <svg-icon name="play-detail" size="22" title="详情"></svg-icon>
        </span>
        <span
          class="zy-svg"
          @click="showShortcutEvent"
          :class="playerInfo.right.type === 'shortcut' ? 'active' : ''"
          v-show="
            playInfo.movie.playMode === 'local' &&
            moviesInfo.moviesList.length > 0
          "
        >
          <svg-icon name="play-guide" size="22" title="快捷键指南"></svg-icon>
        </span>
        <span
          class="timespanSwitch"
          v-if="
            moviesInfo.moviesList.length > 1 &&
            playInfo.movie.playMode === 'local'
          "
          title="跳过片头片尾，建议优先通过快捷键设置，更便捷更精准"
        >
          <el-switch
            v-model="moviesInfo.showTimespanSetting"
            active-text="手动跳略时长"
          ></el-switch>
        </span>
        <span class="timespan" v-if="moviesInfo.showTimespanSetting">
          <label>片头：</label>
          <input
            type="number"
            v-model="moviesInfo.startPosition.min"
            style="width: 45px"
            min="00"
            max="59"
            placeholder="00"
            required
          />
          <label>:</label>
          <input
            type="number"
            v-model="moviesInfo.startPosition.sec"
            style="width: 45px"
            min="00"
            max="59"
            placeholder="00"
            required
          />
          <span></span>
          <label>片尾：</label>
          <input
            type="number"
            v-model="moviesInfo.endPosition.min"
            style="width: 45px"
            min="00"
            max="59"
            placeholder="00"
            required
          />
          <label>:</label>
          <input
            type="number"
            v-model="moviesInfo.endPosition.sec"
            style="width: 45px"
            min="00"
            max="59"
            placeholder="00"
            required
          />
          <span></span>
          <input type="button" value="确定" @click="setProgressDotByInput" />
          <span></span>
          <input
            type="button"
            value="重置"
            @click="
              () => {
                moviesInfo.startPosition.min =
                  moviesInfo.startPosition.sec =
                  moviesInfo.endPosition.min =
                  moviesInfo.endPosition.sec =
                    '00';
                this.clearPosition();
              }
            "
          />
        </span>
        <span
          class="last-tip"
          v-if="
            !playerInfo.isPlaying && historyList.length > 0 && historyList[0]
          "
          @click="historyItemEvent(historyList[0])"
        >
          <span
            >上次播放到:【{{ historyList[0].siteKey }}】{{
              historyList[0].name
            }}
            第{{ historyList[0].index + 1 }}集
          </span>
          <span v-if="historyList[0].time && historyList[0].duration">
            {{ fmtMSS(historyList[0].time.toFixed(0)) }}/{{
              fmtMSS(historyList[0].duration.toFixed(0))
            }}
          </span>
          <span v-if="historyList[0].onlinePlay">在线解析</span>
        </span>
      </div>
    </div>

    <transition name="slideX">
      <div v-if="playerInfo.right.show" class="list">
        <div class="list-top">
          <span
            class="list-top-title"
            v-if="playerInfo.right.type === 'movieList'"
            >播放列表</span
          >
          <span
            class="list-top-title"
            v-if="playerInfo.right.type === 'history'"
            >历史记录</span
          >
          <span
            class="list-top-title"
            v-if="playerInfo.right.type === 'shortcut'"
          >
            快捷键指南{{
              playInfo.playType === "iptv" ? "(直播时部分功能不可用)" : ""
            }}
          </span>
          <span class="list-top-title" v-if="playerInfo.right.type === 'other'"
            >同组其他源的视频</span
          >
          <span
            class="list-top-title"
            v-if="playerInfo.right.type === 'sources'"
            >频道可用线路</span
          >
          <span
            class="list-top-title"
            v-if="playerInfo.right.type === 'movieParseSources'"
            >可用线路</span
          >
          <span class="list-top-close zy-svg" @click="closeListEvent">
            <svg-icon name="close" title="关闭"></svg-icon>
          </span>
        </div>
        <div
          class="list-body vop-scroll"
          :style="{
            overflowY: moviesInfo.scroll ? 'auto' : 'hidden',
            paddingRight: moviesInfo.scroll ? '0' : '5px',
          }"
          @mouseenter="moviesInfo.scroll = true"
          @mouseleave="moviesInfo.scroll = false"
        >
          <ul
            v-if="playerInfo.right.type === 'movieList'"
            class="list-item"
            v-ClickOutside="closeListEvent"
          >
            <div
              style="height: 20px"
              v-if="moviesInfo.currentMoviesFullList.length > 1"
            >
              <el-carousel
                height="auto"
                :autoplay="false"
                indicator-position="none"
                :initial-index="moviesInfo.moviesFullListIndex"
                @change="moviesFullListChange"
              >
                <el-carousel-item
                  style="height: 20px; text-align: center"
                  v-for="item in moviesInfo.currentMoviesFullList"
                >
                  {{ item.flag }}
                </el-carousel-item>
              </el-carousel>
            </div>
            <li v-if="moviesInfo.exportablePlaylist" @click="exportM3u8">
              导出
            </li>
            <li v-if="moviesInfo.moviesList.length === 0">无数据</li>
            <li
              @click="listItemEvent(j)"
              :class="playInfo.movie.index === j ? 'active' : ''"
              v-for="(i, j) in moviesInfo.moviesList"
              :key="j"
            >
              {{ ftName(i) }}
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'localMovieList'"
            class="list-item"
            v-ClickOutside="closeListEvent"
          >
            <li v-if="downloadList.length === 0">无数据</li>
            <li
              @click="listItemEvent(j)"
              :class="playInfo.name === i.movie_name ? 'active' : ''"
              v-for="(i, j) in downloadList"
              :key="i.id"
            >
            <span style="display :inline-block ;width: 100%;overflow: hidden;textOverflow: ellipsis;whiteSpace: nowrap" 
              :title="i.movie_name" v-text="i.movie_name">
            </span>
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'history'"
            class="list-history"
            v-ClickOutside="closeListEvent"
          >
            <li v-if="historyList.length > 0" @click="clearAllHistory">清空</li>
            <li v-if="historyList.length === 0">无数据</li>
            <li
              @click="historyItemEvent(m)"
              :class="playInfo.movie.ids === m.ids ? 'active' : ''"
              v-for="(m, n) in historyList"
              :key="n"
            >
              <span
                class="title"
                :title="
                  '【' +
                  getSiteNameByKey(m.siteKey) +
                  '】' +
                  m.name +
                  ' 第' +
                  (m.index + 1) +
                  '集'
                "
              >
                【{{ getSiteNameByKey(m.siteKey) }}】{{ m.name }} 第{{
                  m.index + 1
                }}集
              </span>
              <span @click.stop="removeHistoryItem(m)" class="detail-delete"
                >删除</span
              >
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'shortcut'"
            class="list-shortcut"
            v-ClickOutside="closeListEvent"
          >
            <li v-for="(m, n) in shortcutList" :key="n">
              <span class="title">{{ m.desc }} -- [ {{ m.key }} ]</span>
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'other'"
            class="list-other"
            v-ClickOutside="closeListEvent"
          >
            <li v-if="moviesInfo.otherSiteMoviesSources.length === 0">
              无数据
            </li>
            <li
              @click="otherItemEvent(m)"
              v-for="(m, n) in moviesInfo.otherSiteMoviesSources"
              :key="n"
            >
              <span class="title">{{ m.name }} - [{{ m.site.name }}]</span>
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'sources'"
            class="list-channels"
            v-ClickOutside="closeListEvent"
          >
            <li v-if="currentChannel.channels.length === 0">当前频道已关闭</li>
            <li v-for="(channel, index) in currentChannel.channels" :key="index">
              <span @click="playInfo.iptv.channelActive = channel.id" class="title" :style="channel.status == '可用' ? {} : {color: 'red'}">{{
                playInfo.iptv.channelActive === channel.id
                  ? channel.label + "[当前]"
                  : channel.label
              }}</span>
            </li>
          </ul>
          <ul
            v-if="playerInfo.right.type === 'movieParseSources'"
            class="list-channels"
            v-ClickOutside="closeListEvent"
          >
            <li v-if="movieParseUrlList.length === 0">当前频道已关闭</li>
            <li v-for="(movieParseUrl, index) in movieParseUrlList" :key="index">
              <span @click="movieParseUrlInfo.activeMovieParseId = movieParseUrl.id" class="title">{{
                movieParseUrlInfo.activeMovieParseId === movieParseUrl.id
                  ? "线路" + (index + 1) + "[当前]"
                  : "线路" + (index + 1)
              }}</span>
            </li>
          </ul>
        </div>
      </div>
    </transition>

    <transition name="slideX">
      <div
        v-if="
          iptvInfo.showChannelGroupList &&
          channelGroupList &&
          channelGroupList.length > 0
        "
        class="list"
        v-ClickOutside="closeListEvent"
      >
        <div class="list-top">
          <span class="list-top-title">频道列表</span>
          <span
            class="list-top-close zy-svg"
            @click="iptvInfo.showChannelGroupList = false"
          >
            <svg-icon name="close"></svg-icon>
          </span>
        </div>
        <div
          class="list-body vop-scroll"
          :style="{
            overflowY: moviesInfo.scroll ? 'auto' : 'hidden',
            paddingRight: moviesInfo.scroll ? '0' : '5px',
          }"
          @mouseenter="moviesInfo.scroll = true"
          @mouseleave="moviesInfo.scroll = false"
        >
          <el-input
            clearable
            size="small"
            title="支持按拼音首字母搜索"
            v-model.trim="playerInfo.searchTxt"
            @change="searchTxtChange"
            placeholder="搜索"
          >
            <i slot="prefix" class="el-input__icon el-icon-search"></i>
          </el-input>
          <el-tree
            ref="channelTreeRef"
            :data="channelGroupTree"
            :props="iptvInfo.defaultProps"
            accordion
            :filter-node-method="filterNode"
            @node-click="handleNodeClick"
          >
          </el-tree>
        </div>
      </div>
    </transition>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  ref,
  onMounted,
  onBeforeMount,
  onBeforeUnmount,
  watch,
  toRefs,
  nextTick
} from "vue";
import { useCoreStore } from "@/store";
import { useIptvStore } from "@/store/iptv";
import { useMovieStore } from "@/store/movie";
import { useHistoryStore } from "@/store/history";
import { useDownloadStore } from "@/store/download";
import moviesApi from "@/api/movies";
import { ElMessage, ClickOutside } from "element-plus";
import mt from "mousetrap";
import PinyinMatch from "pinyin-match";
import { _ } from "lodash";
import { storeToRefs } from "pinia";
import { MoviesPlayer, getPlayerType, getIsVipMovies } from "@/business/play";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import date from "@/util/date";
import SvgIcon from "@/components/SvgIcon.vue";
import { convertFileSrc } from '@tauri-apps/api/tauri';

export default defineComponent({
  name: "Play",
  components: {
    SvgIcon,
  },
  setup() {
    const playerInfo = reactive({
      searchTxt: "",
      skipendStatus: false,
      right: {
        show: false,
        type: "",
      },
      isLive: false,
      isPlaying: false,
    });
    const iptvInfo = reactive({
      showChannelGroupList: false,
      changingIPTV: false,
      defaultProps: {
        label: "label",
        children: "children",
      },
    });

    const moviesInfo = reactive({
      showTimespanSetting: false,
      moviesList: [],
      otherSiteMoviesSources: [],
      startPosition: { min: "00", sec: "00" }, // 对应调略输入框
      endPosition: { min: "00", sec: "00" },
      exportablePlaylist: false,
      currentTime: 0,
      isStar: false,
      scroll: false,
      currentMoviesFullList: [],
      moviesFullListIndex: 0,
      iframeFullScreen: false,
      manualClosePlayer: true,
    });

    let player = null;
    const coreStore = useCoreStore();
    const {
      getPlayerConf,
      updatePlayerConf,
      getAllShortcut,
      updateSystemConf,
      resetPlayInfo,
    } = coreStore;
    const {
      view,
      playMovieUq,
      playInfo,
      playMovieParams,
      playerConf,
      shortcutList,
      systemConf,
      movieParseUrlList,
      movieParseUrl,
      movieParseUrlInfo
    } = storeToRefs(coreStore);

    const iptvStore = useIptvStore();
    const { getAllChannelGroup } = iptvStore;
    const {
      currentChannel,
      channelGroupList,
      channelGroupTree,
    } = storeToRefs(iptvStore);

    const movieStore = useMovieStore();
    const { getSiteByKey, getSiteNameByKey, getAllSite, fetchPlaylist } =
      movieStore;
    const { movieDetailCache, siteList, detail, videoDetailCache } =
      storeToRefs(movieStore);

    const historyStore = useHistoryStore();
    const { refreshHistoryList, getAllHistory, refreshCurrentHistory } =
      historyStore;
    const { historyList, currentHistory } = toRefs(historyStore);

    const downloadStore = useDownloadStore();
    const { downloadList } = toRefs(downloadStore);

    const historyTimer = ref();
    const channelTreeRef = ref();

    const getPlayer = (videoUrl, force = false) => {
      const playerType = getPlayerType(videoUrl);
      const showPalyPrevAndNext =
        moviesInfo.moviesList.length > 1 || playInfo.value.iptv.channelGroupId;
      if (force || !player || !player.dp || playerType !== player.playerType) {
        closePlayer();
        player = new MoviesPlayer(
          playerType,
          playerInfo,
          playerConf.value,
          showPalyPrevAndNext
        );
        bindEvent();
      } else {
        player.dp.playPrevAndNext(showPalyPrevAndNext);
      }
    };

    const playChannel = () => {
      moviesInfo.moviesList = [];
      let channel;
      playInfo.value.name = currentChannel.value.channel_name;
      if (currentChannel.value.channels) {
        channel = _.find(currentChannel.value.channels, { id: playInfo.value.iptv.channelActive });
      } else {
        return;
      }

      iptvInfo.changingIPTV = true;
      getPlayer(channel.url);
      player.dp.switchVideo({
        url: channel.url,
        type: player.dpConfig.video.type,
      });
      iptvInfo.changingIPTV = false;
    };

    const handleNodeClick = (node) => {
      if (node.channelGroup) {
        playInfo.value.iptv.channelGroupId = node.channelGroup.id;
        playInfo.value.iptv.channelGroupId = node.channelGroup.channel_active;
      }
    };

    const getUrls = async () => {
      playerInfo.isPlaying = true;
      if (!player || !player.dp) getPlayer();
      playInfo.value.movie.name = "";
      playInfo.value.movie.onlineUrl = "";
      if (historyTimer.value) {
        clearInterval(historyTimer.value);
        historyTimer.value = null;
      }

      if (playInfo.value.playType === "iptv") {
        // 是直播源，直接播放
        playChannel(currentChannel.value);
      } else if (playInfo.value.playType === "localMovie") {
        iptvInfo.showChannelGroupList = false;
        let downloadInfo = await invoke("get_download_by_id", {downloadId: playInfo.value.download.downloadId});
        const assetUrl = convertFileSrc(downloadInfo.url);
        playInfo.value.name = downloadInfo.movie_name;
        player.dp.switchVideo({
          url: assetUrl,
          type: "mp4",
        });
      } else {
        iptvInfo.showChannelGroupList = false;
        const key = playMovieUq.value;
        let time = undefined;
        moviesInfo.startPosition = { min: "00", sec: "00" };
        moviesInfo.endPosition = { min: "00", sec: "00" };
        if (currentHistory.value) {
          if (currentHistory.value.index == playInfo.value.movie.index) {
            time = currentHistory.value.playTime;
          }

          if (!playInfo.value.movie.index)
            playInfo.value.movie.index = currentHistory.value.index;
          if (!videoDetailCache.value[key]) videoDetailCache.value[key] = {};
          if (!playInfo.value.movie.videoFlag)
            playInfo.value.movie.videoFlag = currentHistory.value.videoFlag;
          if (currentHistory.value.startPosition) {
            // 数据库保存的时长通过快捷键设置时可能为小数, this.startPosition为object对应输入框分秒转化到数据库后肯定为整数
            videoDetailCache.value[key].startPosition =
              currentHistory.value.startPosition;
            moviesInfo.startPosition = {
              min: "" + parseInt(currentHistory.value.startPosition / 60),
              sec: "" + parseInt(currentHistory.value.startPosition % 60),
            };
          }
          if (currentHistory.value.endPosition) {
            videoDetailCache.value[key].endPosition =
              currentHistory.value.endPosition;
            moviesInfo.endPosition = {
              min: "" + parseInt(currentHistory.value.endPosition / 60),
              sec: "" + parseInt(currentHistory.value.endPosition % 60),
            };
          }
        }
        const index = playInfo.value.movie.index || 0;
        playVideo(index, time);
      }
    };

    const moviesFullListChange = async (index) => {
      moviesInfo.moviesList = moviesInfo.currentMoviesFullList[index].list;
      moviesInfo.moviesFullListIndex = index;
      if (currentHistory.value) {
        currentHistory.value.videoFlag =
          moviesInfo.currentMoviesFullList[index].flag;
        currentHistory.value.updateTime = date.getDateTimeStr();
        await invoke("save_history", { history: currentHistory.value });
        refreshHistoryList();
      }
    };

    const playVideo = (index = 0, time = 0) => {
      playerInfo.isLive = false;
      moviesInfo.isStar = false;
      moviesInfo.exportablePlaylist = false;
      fetchPlaylist(playMovieUq.value)
        .then(async (fullList) => {
          moviesInfo.currentMoviesFullList = fullList;
          let playlist = fullList[0].list; // ZY支持的已移到首位
          // 如果设定了特定的video flag, 获取该flag下的视频列表
          const videoFlag = playInfo.value.movie.videoFlag;
          if (videoFlag) {
            fullList.forEach((x, index) => {
              if (x.flag == videoFlag) {
                playlist = x.list;
                moviesInfo.moviesFullListIndex = index;
              }
            });
          }
          moviesInfo.moviesList = playlist;
          const url = playlist[index].includes("$")
            ? playlist[index].split("$")[1]
            : playlist[index];
          if (
            playlist.every((e) =>
              e.includes("$")
                ? e.split("$")[1].endsWith(".m3u8")
                : e.endsWith(".m3u8")
            )
          )
            moviesInfo.exportablePlaylist = true;
          if (!url.endsWith(".m3u8") && !url.endsWith(".mp4")) {
            if (getIsVipMovies(url)) {
              ElMessage.info("即将调用解析接口播放，请等待...");
              movieParseUrlInfo.value.vipPlay = true;
              playInfo.value.movie.onlineUrl = movieParseUrl.websiteParseUrl + url;
            } else {
              playInfo.value.movie.onlineUrl = url;
            }
            player.destroy();
            playInfo.value.movie.playMode = "online";
            videoPlaying("online");
            return;
          } else {
            playInfo.value.movie.playMode = "local";
            const key = playMovieUq.value;
            getPlayer(url);
            player.dp.switchVideo({
              url: url,
              type: player.dpConfig.video.type,
            });
            bindOnceEvent();
            const startTime = videoDetailCache.value[key].startPosition || 0;
            player.dp.seek(time > startTime ? time : startTime);
          }
          videoPlaying();
          playerInfo.skipendStatus = false;
        })
        .catch((err) => {
          ElMessage.error("播放地址可能已失效，请换源并调整收藏");
          otherEvent();
        });
    };

    const videoPlaying = async (isOnline) => {
      const videoFlag = playInfo.value.movie.videoFlag || "";
      let time, duration;
      let startPosition = 0;
      let endPosition = 0;
      if (isOnline) {
        time = duration = 0;
      } else {
        time = player.currentTime();
        duration = player.duration();
      }
      if (!currentHistory.value) {
        const history = {
          id: 0,
          name: playInfo.value.name,
          siteKey: playInfo.value.movie.siteKey,
          ids: playInfo.value.movie.ids.toString(),
          index: playInfo.value.movie.index,
          playTime: time,
          duration: duration,
          startPosition: startPosition,
          endPosition: endPosition,
          detail: JSON.stringify(movieDetailCache.value[playMovieUq.value]),
          onlinePlay: isOnline ? playInfo.value.movie.onlineUrl : "",
          videoFlag: videoFlag,
          hasUpdate: "0",
          updateTime: date.getDateTimeStr(),
        };
        await invoke("save_history", { history: history });
        refreshCurrentHistory();
      }
      if (isOnline) {
        nextTick(() => {
          const iframeElement = document.querySelector("iframe"); // 获取iframe元素
          iframeElement.addEventListener("fullscreenchange", async (event) => {
            if (!moviesInfo.iframeFullScreen) {
              await appWindow.setFullscreen(true);
            } else {
              await appWindow.setFullscreen(false);
              await appWindow.unmaximize();
            }
            moviesInfo.iframeFullScreen = !moviesInfo.iframeFullScreen;
          });
        })
        currentHistory.value.index = playInfo.value.movie.index;
        currentHistory.value.onlinePlay = playInfo.value.movie.onlineUrl;
        currentHistory.value.updateTime = date.getDateTimeStr();
        await invoke("save_history", { history: currentHistory.value });
      } else {
        timerEvent();
      }
      refreshHistoryList();
    };

    // 播放下一集
    const prevNextEvent = (isReverse = false) => {
      if (playInfo.value.playType === "iptv") {
        let index = _.findIndex(channelGroupList.value, [
          "id",
          playInfo.value.iptv.channelGroupId,
        ]);
        if (isReverse) {
          index = index === 0 ? channelGroupList.value.length - 1 : index - 1;
        } else {
          index = index === channelGroupList.value.length - 1 ? 0 : index + 1;
        }
        const channel = channelGroupList.value[index];
        playInfo.value.iptv.channelGroupId = channel.id;
        playInfo.value.iptv.channelActive = channel.channel_active;
      } else if (playInfo.value.playType === "localMovie") {
        
      } else {
        if (isReverse) {
          if (playInfo.value.movie.index >= 1) {
            playInfo.value.movie.index--;
            playInfo.value.movie.time = 0;
          } else {
            ElMessage.warning("这已经是第一集了。");
          }
        } else if (
          playInfo.value.movie.index <
          moviesInfo.moviesList.length - 1
        ) {
          playInfo.value.movie.index++;
          playInfo.value.movie.time = 0;
        } else {
          ElMessage.warning("这已经是最后一集了。");
        }
      }
    };

    // 定时更新历史记录时间
    const timerEvent = () => {
      historyTimer.value = setInterval(async () => {
        if (!player.playing) {
          return;
        }
        if (currentHistory.value) {
          currentHistory.value.index = playInfo.value.movie.index;
          currentHistory.value.playTime = player.currentTime();
          currentHistory.value.duration = player.duration();
          currentHistory.value.updateTime = date.getDateTimeStr();
          await invoke("save_history", { history: currentHistory.value });
        }
      }, 10000);
    };

    // 自动关闭播发器
    const closePlayer = () => {
      if (player && player.dp) {
        player.dp.destroy();
        player.dp = null;
      }
    };

    // 手动关闭播放器
    const closePlayerAndInit = () => {
      playerInfo.isPlaying = false;
      moviesInfo.manualClosePlayer = true;
      resetPlayInfo();
      moviesInfo.moviesList = [];
      if (player && player.dp) {
        player.dp.destroy();
        player.dp = null;
      }

      getPlayer("", true);
    };

    const bindOnceEvent = () => {
      let dp = player.dp;

      dp.on(
        "playing",
        _.once(() => {
          if (playInfo.value.movie.siteKey) {
            const key = playMovieUq.value;
            player.setHighlightByName(
              videoDetailCache.value[key]["startPosition"],
              "片头"
            );
            let time;
            if (videoDetailCache.value[key]["endPosition"]) {
              time =
                player.dp.video.duration -
                videoDetailCache.value[key]["endPosition"];
            }
            player.setHighlightByName(time, "片尾");
            player.durationchange();
          }
        })
      );
    };

    // 绑定事件
    const bindEvent = () => {
      let dp = player.dp;
      let dpConfig = player.dpConfig;
      // 直播卡顿时换源换台
      let stallIptvTimeout;
      let stallCount = 0;
      dp.on("waiting", () => {
        player.playing = false;
        if (
          dpConfig.isLive &&
          playerConf.value.autoChangeSourceWhenIptvStalling
        ) {
          stallIptvTimeout = setTimeout(() => {
            stallCount++;
            if (stallCount >= 4) {
              stallCount = 0;
              prevNextEvent();
            }
          }, 5 * 1000);
        }
      });
      dp.on("canplay", () => {
        player.playing = true;
        stallCount = 0;
        clearTimeout(stallIptvTimeout);
        if (dp.video.paused) {
          dp.play();
        }
      });
      dp.on("destroy", () => {
        stallCount = 0;
        clearTimeout(stallIptvTimeout);
      });

      dp.on(
        "volumechange",
        _.debounce(async () => {
          playerConf.value.volume = player.dp.video.volume;
          updatePlayerConf();
        }, 500)
      );

      dp.on("timeupdate", () => {
        if (dpConfig.isLive) return;
        const key = playMovieUq.value;
        if (
          videoDetailCache.value[key] &&
          videoDetailCache.value[key].endPosition
        ) {
          const time =
            dp.video.duration -
            videoDetailCache.value[key].endPosition -
            dp.video.currentTime;
          if (time > 0 && time < 0.3) {
            // timeupdate每0.25秒触发一次，只有自然播放到该点时才会跳过片尾
            if (!playerInfo.skipendStatus) {
              playerInfo.skipendStatus = true;
              dp.ended();
            }
          }
        }
      });

      dp.on("play", async () => {
        clearTimeout(stallIptvTimeout);
        if (
          !playInfo.value.movie.siteKey &&
          !playInfo.value.iptv.channelGroupId
        ) {
          if (playInfo.value.playType == "onlineMovie" && !playInfo.value.movie.ids) {
            await refreshHistoryList();
            // 如果当前播放页面的播放信息没有被赋值,播放历史记录
            if (historyList.value.length === 0) {
              ElMessage.warning("历史记录为空，无法播放！");
              return;
            }
            const historyItem = historyList.value[0];
            playInfo.value.isLive = false;
            playInfo.value.movie.siteKey = historyItem.siteKey;
            playInfo.value.movie.ids = historyItem.ids;
            playInfo.value.name = historyItem.name;
            playInfo.value.movie.index = historyItem.index;
          } else if (playInfo.value.playType === "iptv") {
            playChannel();
            iptvInfo.showChannelGroupList = false;
          }
        }
      });

      dp.on(
        "ended",
        _.debounce(() => {
          if (
            moviesInfo.moviesList.length > 1 &&
            moviesInfo.moviesList.length - 1 > playInfo.value.movie.index
          ) {
            playInfo.value.movie.time = 0;
            playInfo.value.movie.index++;
          }
        }, 1000)
      );

      dp.on("playPrev", () => {
        prevNextEvent(true);
      });

      dp.on("playNext", () => {
        prevNextEvent();
      });

      dp.on("fullscreen", async () => {
        await appWindow.setFullscreen(true);
      });

      dp.on("fullscreen_cancel", async () => {
        await appWindow.setFullscreen(false);
        await appWindow.unmaximize();
      });
    };

    const otherEvent = () => {
      if (playInfo.value.playType != "iptv") {
        playerInfo.right.type = "other";
        getOtherSites();
        moviesInfo.currentTime = player.dp ? player.dp.video.currentTime : 0;
      } else {
        playerInfo.right.type = "sources";
      }
      playerInfo.right.show = true;
    };

    const getOtherSites = async () => {
      moviesInfo.otherSiteMoviesSources = [];
      const currentSite = getSiteByKey(playInfo.value.movie.siteKey);
      await getAllSite();
      // 排除已关闭的源和当前源
      for (const siteItem of siteList.value.filter(
        (x) =>
          x.isActive &&
          x.group === currentSite.group &&
          x.key !== playInfo.value.movie.siteKey
      )) {
        moviesApi
          .search(siteItem, playInfo.value.movie.name)
          .then((searchRes) => {
            const type = Object.prototype.toString.call(searchRes);
            if (type === "[object Array]") {
              searchRes.forEach(async (item) => {
                const detailRes = item;
                detailRes.key = siteItem.key;
                detailRes.site = siteItem;
                moviesInfo.otherSiteMoviesSources.push(detailRes);
              });
            }
            if (type === "[object Object]") {
              const detailRes = searchRes;
              detailRes.key = siteItem.key;
              detailRes.site = siteItem;
              moviesInfo.otherSiteMoviesSources.push(detailRes);
            }
          });
      }
    };

    const closeListEvent = () => {
      const lastRightType = playerInfo.right.type;
      const lastChannelGroupListState = iptvInfo.showChannelGroupList;
      setTimeout(() => {
        if (lastRightType === playerInfo.right.type) {
          playerInfo.right.show = false;
          playerInfo.right.type = "";
        }
        if (lastChannelGroupListState === iptvInfo.showChannelGroupList) {
          iptvInfo.showChannelGroupList = false;
        }
      }, 50);
    };

    const showShortcutEvent = () => {
      if (playerInfo.right.type === "shortcut") {
        playerInfo.right.show = false;
        playerInfo.right.type = "";
      } else {
        playerInfo.right.show = true;
        playerInfo.right.type = "shortcut";
      }
    };

    const moviesListEvent = () => {
      if (playerInfo.right.type === "movieList") {
        playerInfo.right.show = false;
        playerInfo.right.type = "";
      } else {
        playerInfo.right.show = true;
        playerInfo.right.type = "movieList";
      }
    };

    const localMoviesListEvent = () => {
      if (playerInfo.right.type === "localMovieList") {
        playerInfo.right.show = false;
        playerInfo.right.type = "";
      } else {
        playerInfo.right.show = true;
        playerInfo.right.type = "localMovieList";
      }
    };

    const historyEvent = () => {
      if (playerInfo.right.type === "history") {
        playerInfo.right.show = false;
        playerInfo.right.type = "";
      } else {
        playerInfo.right.show = true;
        playerInfo.right.type = "history";
      }
      refreshHistoryList();
    };

    const starEvent = async () => {
      const statStr = await invoke("get_star_by_uq", playMovieParams.value);
      if (statStr) {
        await invoke("del_star", JSON.parse(statStr).id);
        moviesInfo.isStar = false;
        ElMessage.success("取消收藏成功");
      } else {
        const star = {
          name: playInfo.value.name,
          ids: playInfo.value.movie.ids.toString(),
          siteKey: playInfo.value.movie.siteKey,
          detail: JSON.stringify(movieDetailCache.value[playMovieUq.value]),
        };
        await invoke("save_star", { star: star });
        ElMessage.success("收藏成功");
        moviesInfo.isStar = true;
      }
    };

    const historyItemEvent = (history) => {
      playInfo.value.movie.siteKey = history.siteKey;
      playInfo.value.movie.ids = history.ids;
      playInfo.value.name = history.name;
      playInfo.value.movie.index = history.index;
      playerInfo.right.show = false;
      playerInfo.right.type = "";
    };

    const setProgressDotByInput = async () => {
      const startTime =
        parseInt(moviesInfo.startPosition.min) * 60 +
        parseInt(moviesInfo.startPosition.sec);
      const endTime =
        parseInt(moviesInfo.endPosition.min) * 60 +
        parseInt(moviesInfo.endPosition.sec);
      const videoDuration = player.dp.video.duration;
      if (startTime > videoDuration || endTime > videoDuration) {
        ElMessage.error("设置的跳跃时间长度已超过视频播放时长，请调整设置！");
        return;
      }

      const key = playMovieUq.value;
      if (startTime != 0) {
        videoDetailCache.value[key]["startPosition"] = startTime;
        currentHistory.value["startPosition"] = startTime;
        player.setHighlightByName(startTime, "片头");
      }

      if (endTime != 0) {
        videoDetailCache.value[key]["endPosition"] = endTime;
        currentHistory.value["endPosition"] = endTime;
        const time = videoDuration - endTime;
        player.setHighlightByName(time, "片尾");
      }

      if (startTime != 0 || endTime != 0) {
        player.durationchange();
        currentHistory.value.updateTime = date.getDateTimeStr();
        await invoke("save_history", { history: currentHistory.value });
        refreshHistoryList();
      }
    };

    const listItemEvent = (n) => {
      if (playInfo.value.playType == "iptv") {
        const channel = channelGroupList.value[n];
        // 是直播源，直接播放
        playInfo.value.iptv.channelGroupId = channel.id;
        playInfo.value.iptv.channelActive = channel.channel_active;
      } else if (playInfo.value.playType == "localMovie") {
        const movie = downloadList.value[n];
        playInfo.value.download.downloadId = movie.id;
      } else {
        playInfo.value.movie.index = n;
        playInfo.value.movie.videoFlag =
          moviesInfo.currentMoviesFullList[moviesInfo.moviesFullListIndex].flag;
        playerInfo.right.show = false;
        playerInfo.right.type = "";
      }
    };

    const detailEvent = () => {
      detail.value = {
        show: true,
        siteKey: playInfo.value.movie.siteKey,
        ids: playInfo.value.movie.ids,
        index: playInfo.value.movie.index,
      };
    };

    const ftName = (e, n) => {
      const num = e.split("$");
      if (num.length > 1) {
        return e.split("$")[0];
      } else {
        return `第${n + 1}集`;
      }
    };

    const clearAllHistory = async () => {
      await invoke("del_all_history", {});
      refreshHistoryList();
    };

    const otherItemEvent = (e) => {
      playInfo.value.playType = "movieOnline";
      // 打开当前播放的剧集index, 定位到当前的时间
      playInfo.value.movie = {
        siteKey: e.site.key,
        ids: e.id,
        index: playInfo.value.movie.index,
        name: e.name,
        videoFlag: "",
        onlineUrl: "",
        playMode: "local",
      };
    };

    const clearPosition = () => {
      player.dpConfig.highlight = [];
      player.durationchange();
    };

    const removeHistoryItem = async (history) => {
      await invoke("del_history", { id: history.id });
      await refreshHistoryList();
    };

    const filterNode = (value, data) => {
      if (!value) return true;
      return PinyinMatch.match(data.label, value);
    };

    const exportM3u8 = () => {
      const m3u8Arr = [];
      for (const i of moviesInfo.moviesList) {
        const j = i.split("$");
        let link, name;
        if (j.length > 1) {
          for (let m = 0; m < j.length; m++) {
            if (j[m].indexOf(".m3u8") >= 0 && j[m].startsWith("http")) {
              link = j[m];
              break;
            }
          }
          name = j[0];
        } else {
          name = `第${m3u8Arr.length + 1}集`;
          link = j[0];
        }
        m3u8Arr.push({
          name: name,
          link: link,
        });
      }
      let m3u8Content = "#EXTM3U\n";
      for (const item of m3u8Arr) {
        m3u8Content += `#EXTINF:-1, ${item.name}\n${item.link}\n`;
      }
      const blob = new Blob([m3u8Content], {
        type: "application/vnd.apple.mpegurl",
      });
      const downloadElement = document.createElement("a");
      const href = window.URL.createObjectURL(blob);
      downloadElement.href = href;
      downloadElement.download = `${playInfo.value.name}.m3u`;
      document.body.appendChild(downloadElement);
      downloadElement.click();
      document.body.removeChild(downloadElement);
      window.URL.revokeObjectURL(href);
    };

    const searchTxtChange = () => {
      channelTreeRef.value.filter(playerInfo.searchTxt);
    };

    const mtEvent = () => {
      if (!systemConf.value.shortcutEnabled) {
        mt.reset();
        return;
      }
      if (systemConf.value.shortcutModified) mt.reset();
      shortcutList.value.forEach((shortcut) => {
        mt.bind(shortcut.key, () => {
          if (view.value === "Play") {
            shortcutEvent(shortcut.name);
          }
        });
      });
      systemConf.value.shortcutModified = false;
      updateSystemConf();
    };

    const shortcutEvent = (shortcutName) => {
      switch (shortcutName) {
        case "playAndPause":
          if (player.videoExist()) {
            player.dp.toggle();
          }
          break;
        case "forward":
          if (player.videoExist() && !player.dp.video.paused) {
            player.dp.video.currentTime += parseInt(
              playerConf.value.forwardTimeInSec
            );
          }
          break;
        case "back":
          if (player.videoExist() && !player.dp.video.paused) {
            player.dp.video.currentTime -= parseInt(
              playerConf.value.forwardTimeInSec
            );
          }
          break;
        case "volumeUp":
          if (player.playerExist()) {
            let volume =
              player.dp.video.volume + 0.1 > 1
                ? 1
                : player.dp.video.volume + 0.1;
            player.dp.volume(volume, false, false);
          }
          break;
        case "volumeDown":
          if (player.playerExist()) {
            let volume =
              player.dp.video.volume - 0.1 < 0
                ? 0
                : player.dp.video.volume - 0.1;
            player.dp.volume(volume, false, false);
          }
          break;
        case "next":
          prevNextEvent();
          break;
        case "prev":
          prevNextEvent(true);
          break;
        case "playbackRateUp":
          if (player.videoExist() && !player.dp.video.paused) {
            const rate = player.dp.playbackRate;
            player.dp.playbackRate = rate + 0.25;
            ElMessage.info("当前播放速度为: " + player.dp.playbackRate);
          }
          break;
        case "playbackRateDown":
          if (player.videoExist() && !player.dp.video.paused) {
            const rate = player.dp.playbackRate;
            if (rate > 0.25) {
              player.dp.playbackRate = rate - 0.25;
              ElMessage.info("当前播放速度为: " + player.dp.playbackRate);
            }
          }
          break;
        default:
          break;
      }
      return false;
    };

    const leadingZero = (time) => {
      // 格式化单个调略输入框
      Object.keys(time).forEach((key) => {
        if (time[key] > 59 || time[key] < 0) {
          time[key] = "00";
        } else if (time[key].length > 2) {
          time[key] = "" + parseInt(time[key]);
        } else if (time[key] < 10 && time[key].length === 1) {
          time[key] = "0" + time[key];
        }
      });
    };

    const fmtMSS = (s) => {
      return (s - (s %= 60)) / 60 + (s > 9 ? ":" : ":0") + s;
    };

    watch(
      () => {
        if (playInfo.value.playType == "onlineMovie")
          return (
            playInfo.value.movie.siteKey +
            "@" +
            playInfo.value.movie.ids +
            "@" +
            playInfo.value.movie.index
          );
        if (playInfo.value.playType == "localMovie")
          return playInfo.value.download.downloadId;
        if (playInfo.value.playType == "iptv")
          return playInfo.value.iptv.channelGroupId + "@" + playInfo.value.iptv.channelActive;
      },
      async () => {
        if (playerInfo.changingIPTV) return;
        // 手动关闭播放器，直接返回
        if (moviesInfo.manualClosePlayer) {
          moviesInfo.manualClosePlayer = false;
          return;
        }
        if (playInfo.value.playType == "onlineMovie") {
          await refreshCurrentHistory();
        }
        getUrls();
      },
      { deep: true }
    );

    watch(
      () => view.value,
      () => {
        if (view.value === "Play") {
          playerInfo.right.show = false;
          playerInfo.right.type = "";
          refreshHistoryList();
          getAllChannelGroup();
          if (playerInfo.playType == "") {
            iptvInfo.showChannelGroupList = true;
          }
        }
      },
      { deep: true }
    );

    watch(
      () => moviesInfo.startPosition,
      () => {
        leadingZero(moviesInfo.startPosition)
      },
      { deep: true }
    );

    watch(
      () => moviesInfo.endPosition,
      () => {
        leadingZero(moviesInfo.endPosition)
      },
      { deep: true }
    );

    onBeforeMount(async () => {
      resetPlayInfo();
      getPlayerConf();
      await getAllShortcut();
      mtEvent();
      getAllHistory();
    });

    onMounted(async () => {
      if (playInfo.value.playType) {
        if (playInfo.value.playType == "iptv") {
          iptvInfo.showChannelGroupList = true;
        }
        getPlayer("", true);
      }
    });

    onBeforeUnmount(() => {
      if (historyTimer.value) {
        clearInterval(historyTimer.value);
      }
      closePlayer();
    });

    return {
      playerInfo,
      iptvInfo,
      moviesInfo,
      playInfo,
      otherEvent,
      closeListEvent,
      showShortcutEvent,
      moviesListEvent,
      historyEvent,
      handleNodeClick,
      starEvent,
      historyList,
      historyItemEvent,
      setProgressDotByInput,
      listItemEvent,
      ftName,
      getSiteNameByKey,
      detailEvent,
      clearAllHistory,
      channelGroupTree,
      channelGroupList,
      moviesFullListChange,
      otherItemEvent,
      clearPosition,
      closePlayerAndInit,
      removeHistoryItem,
      filterNode,
      exportM3u8,
      searchTxtChange,
      currentChannel,
      shortcutList,
      fmtMSS,
      localMoviesListEvent,
      downloadList,
      movieParseUrlList,
      movieParseUrlInfo,
    };
  },
  directives: {
    ClickOutside,
  },
});
</script>
<style>
.dplayer {
  height: 100%;
}
</style>
<style lang="scss" scoped>
.play {
  position: absolute;
  left: 80px;
  right: 20px;
  top: 40px;
  bottom: 20px;
  width: calc(100% - 100px);
  height: calc(100% - 60px);
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 5px;
  .box {
    width: 100%;
    height: 100%;
    display: flex;
    border-radius: 5px;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    padding: 0 10px;
    .title {
      width: 100%;
      height: 40px;
      line-height: 40px;
      .right {
        float: right;
        svg {
          display: inline-block;
          margin-top: 8px;
          cursor: pointer;
        }
      }
    }
    .player {
      width: 100%;
      flex: 1;
      overflow: hidden;
    }
    .more {
      width: 100%;
      height: 50px;
      min-height: 50px;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      span {
        display: flex;
        margin-right: 10px;
        cursor: pointer;
      }
      .timespan {
        margin-left: auto;
        justify-content: space-between;
        align-items: center;
        input {
          border: none;
          &::-webkit-inner-spin-button,
          &::-webkit-outer-spin-button {
            opacity: 1;
          }
        }
        input[type="button"] {
          cursor: pointer;
        }

        input:focus {
          outline: none;
        }
      }
    }
  }
  .list {
    position: absolute;
    top: 0;
    right: 0;
    width: 300px;
    height: 100%;
    z-index: 555;
    border-radius: 3px;
    padding: 0 10px;
    display: flex;
    flex-direction: column;
    .el-tree {
      background-color: inherit;
    }
    .list-top {
      display: flex;
      justify-content: space-between;
      align-items: center;
      height: 30px;
      .list-top-title {
        font-size: 16px;
      }
      .list-top-close {
        display: inline-block;
        cursor: pointer;
      }
    }
    .list-body {
      flex: 1;
      overflow-y: auto;
      ul {
        margin: 0;
        padding: 0;
        list-style: none;
        li {
          position: relative;
          height: 28px;
          width: 100%;
          line-height: 28px;
          padding-left: 10px;
          font-size: 14px;
          cursor: pointer;
          display: flex;
          .title {
            display: inline-block;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            width: 231px;
          }
          .btn {
            display: inline-block;
          }
          .detail-delete {
            display: none;
            position: absolute;
            right: 0;
            height: 28px;
            width: 50px;
            text-align: center;
          }
        }
      }
    }
  }
}
</style>
