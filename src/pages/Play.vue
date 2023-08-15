<template>
  <div class="play">
    <div class="box">
      <div class="title">
        <span v-if="moviesInfo.moviesList.length > 1">『第 {{(movieInfo.index + 1)}} 集』</span>{{movieInfo.name}}
        <span class="right" @click="closePlayerAndInit">
          <svg-icon name="close"></svg-icon>
        </span>
      </div>
      <div class="player" v-show="movieInfo.onlineUrl === ''">
        <div id="dpplayer"></div>
      </div>
      <div class="iframePlayer" v-if="movieInfo.onlineUrl !== ''" style='width:100%;height:100%;'>
        <iframe id="iframePlayer" v-bind:src="movieInfo.onlineUrl" width="100%" height="100%" frameborder="0" scrolling="no" allow='autoplay;fullscreen'>
        </iframe>
      </div>
    
      <div class="more" v-if="video.playType === 'iptv'" :key="video.playType">
        <span class="zy-svg" @click="iptvInfo.showChannelGroupList = !iptvInfo.showChannelGroupList" 
          :class="iptvInfo.showChannelGroupList ? 'active' : ''">
          <svg-icon name="play-play_list" size="22" title="频道列表"></svg-icon>
        </span>
        <span class="zy-svg" @click="otherEvent" :class="playerInfo.right.type === 'sources' ? 'active' : ''">
          <svg-icon name="play-website" size="22" title="换源"></svg-icon>
        </span>
        <span class="zy-svg" @click="showShortcutEvent" :class="playerInfo.right.type === 'shortcut' ? 'active' : ''">
          <svg-icon name="play-guide" size="22" title="快捷键指南"></svg-icon>
        </span>
      </div>
      <div class="more" v-if="video.playType !== 'iptv'" :key="video.playType">
        <span class="zy-svg" @click="otherEvent" v-show="movieInfo.name != ''" :class="playerInfo.right.type === 'other' ? 'active' : ''">
          <svg-icon name="play-website" size="22" title="换源"></svg-icon>
        </span>
        <span class="zy-svg" @click="moviesListEvent" :class="playerInfo.right.type === 'movieList' ? 'active' : ''" 
          v-show="moviesInfo.moviesList.length > 0">
          <svg-icon name="play-play_list" size="22" title="播放列表"></svg-icon>
        </span>
        <span class="zy-svg" @click="historyEvent" :class="playerInfo.right.type === 'history' ? 'active' : ''">
          <svg-icon name="play-history" size="22" title="历史记录"></svg-icon>
        </span>
        <span class="zy-svg" @click="starEvent" :class="moviesInfo.isStar ? 'active' : ''" v-show="moviesInfo.moviesList.length > 0 || moviesInfo.isStar">
          <svg-icon name="play-star" size="22" title="收藏"></svg-icon>
        </span>
        <span class="zy-svg" @click="detailEvent" v-show="moviesInfo.moviesList.length > 0">
          <svg-icon name="play-detail" size="22" title="详情"></svg-icon>
        </span>
        <span class="zy-svg" @click="shareEvent" v-show="moviesInfo.moviesList.length > 0">
          <svg-icon name="play-share" size="22" title="分享"></svg-icon>
        </span>
        <span class="zy-svg" @click="showShortcutEvent" :class="playerInfo.right.type === 'shortcut' ? 'active' : ''" 
          v-show="!movieInfo.onlineUrl && moviesInfo.moviesList.length > 0">
          <svg-icon name="play-guide" size="22" title="快捷键指南"></svg-icon>
        </span>
        <span class="zy-svg" @click="issueEvent" v-show="moviesInfo.moviesList.length > 0">
          <svg-icon name="play-debug_info" size="22" title="复制调试信息"></svg-icon>
        </span>
        <span class="timespanSwitch" v-if="moviesInfo.moviesList.length > 1 && !movieInfo.onlineUrl" title="跳过片头片尾，建议优先通过快捷键设置，更便捷更精准">
          <el-switch v-model="moviesInfo.showTimespanSetting" active-text="手动跳略时长"></el-switch>
        </span>
        <span class="timespan" v-if="moviesInfo.showTimespanSetting">
          <label>片头：</label>
          <input type="number" v-model="moviesInfo.startPosition.min" style="width:45px" min="00" max="59" placeholder="00" required>
          <label>:</label>
          <input type="number" v-model="moviesInfo.startPosition.sec" style="width:45px" min="00" max="59" placeholder="00" required>
          <span></span>
          <label>片尾：</label>
          <input type="number" v-model="moviesInfo.endPosition.min" style="width:45px" min="00" max="59" placeholder="00" required>
          <label>:</label>
          <input type="number" v-model="moviesInfo.endPosition.sec" style="width:45px" min="00" max="59" placeholder="00" required>
          <span></span>
          <input type="button" value="确定" @click="setProgressDotByInput">
          <span></span>
          <input type="button" value="重置" @click="() => { moviesInfo.startPosition.min = moviesInfo.startPosition.sec = moviesInfo.endPosition.min
             = moviesInfo.endPosition.sec = '00'; this.clearPosition() }">
        </span>
        <span class="last-tip" v-if="!playerInfo.isPlaying && historyList.length > 0 && historyList[0]" 
          @click="historyItemEvent(historyList[0])">
          <span>上次播放到:【{{historyList[0].siteKey}}】{{historyList[0].name}} 第{{historyList[0].index+1}}集 </span>
          <span v-if="historyList[0].time && historyList[0].duration">
            {{fmtMSS(historyList[0].time.toFixed(0))}}/{{fmtMSS(historyList[0].duration.toFixed(0))}}
          </span>
          <span v-if="historyList[0].onlinePlay">在线解析</span>
        </span>
      </div>
    </div>
    
    <transition name="slideX">
      <div v-if="playerInfo.right.show" class="list">
        <div class="list-top">
          <span class="list-top-title" v-if="playerInfo.right.type === 'movieList'">播放列表</span>
          <span class="list-top-title" v-if="playerInfo.right.type === 'history'">历史记录</span>
          <span class="list-top-title" v-if="playerInfo.right.type === 'shortcut'">
            快捷键指南{{ video.playType === 'iptv' ? '(直播时部分功能不可用)' : '' }}
          </span>
          <span class="list-top-title" v-if="playerInfo.right.type === 'other'">同组其他源的视频</span>
          <span class="list-top-title" v-if="playerInfo.right.type === 'sources'">该频道可用源</span>
          <span class="list-top-close zy-svg" @click="closeListEvent">
            <svg-icon name="close" title="关闭"></svg-icon>
          </span>
        </div>
        <div class="list-body vop-scroll" :style="{overflowY: moviesInfo.scroll? 'auto' : 'hidden', paddingRight: moviesInfo.scroll ? '0': '5px' }" 
          @mouseenter="moviesInfo.scroll = true" @mouseleave="moviesInfo.scroll = false">
          <ul v-if="playerInfo.right.type === 'movieList'" class="list-item"  v-ClickOutside="closeListEvent">
            <div style="height: 20px" v-if="moviesInfo.currentMoviesFullList.length > 1">
              <el-carousel height="auto" :autoplay="false" indicator-position="none" :initial-index="moviesInfo.moviesFullListIndex" 
                @change="moviesFullListChange">
                <el-carousel-item style="height: 20px; text-align: center;" v-for="(item) in moviesInfo.currentMoviesFullList">
                  {{ item.flag }}
                </el-carousel-item>
              </el-carousel>
            </div>
            <li v-if="moviesInfo.exportablePlaylist" @click="exportM3u8">导出</li>
            <li v-if="moviesInfo.moviesList.length === 0">无数据</li>
            <li @click="listItemEvent(j)" :class="movieInfo.index === j ? 'active' : ''" v-for="(i, j) in moviesInfo.moviesList" :key="j">
              {{ftName(i)}}
            </li>
          </ul>
          <ul v-if="playerInfo.right.type === 'history'" class="list-history"  v-ClickOutside="closeListEvent">
            <li v-if="historyList.length > 0" @click="clearAllHistory">清空</li>
            <li v-if="historyList.length === 0">无数据</li>
            <li @click="historyItemEvent(m)" :class="movieInfo.ids === m.ids ? 'active' : ''" v-for="(m, n) in historyList" :key="n">
              <span class="title" :title="'【' + getSiteByKey(m.siteKey) + '】' + m.name + ' 第' + (m.index+1) + '集'">
                【{{getSiteByKey(m.siteKey)}}】{{m.name}} 第{{m.index+1}}集
              </span>
              <span @click.stop="removeHistoryItem(m)" class="detail-delete">删除</span>
            </li>
          </ul>
          <ul v-if="playerInfo.right.type === 'shortcut'" class="list-shortcut"  v-ClickOutside="closeListEvent">
            <li v-for="(m, n) in shortcutList" :key="n"><span class="title">{{m.desc}} -- [ {{m.key}} ]</span></li>
          </ul>
          <ul v-if="playerInfo.right.type === 'other'" class="list-other" v-ClickOutside="closeListEvent">
            <li v-if="moviesInfo.otherSiteMoviesSources.length === 0">无数据</li>
            <li @click="otherItemEvent(m)" v-for="(m, n) in moviesInfo.otherSiteMoviesSources" :key="n">
              <span class="title">{{m.name}} - [{{m.site.name}}]</span>
            </li>
          </ul>
          <ul v-if="playerInfo.right.type === 'sources'" class="list-channels" v-ClickOutside="closeListEvent">
            <li v-if="getSourcePlayList.length === 0">当前频道已关闭</li>
            <li v-for="(channel, index) in getSourcePlayList" :key="index">
              <span @click="playChannel(channel)" class="title">{{ channel.id === channelId ? channel.name + '[当前]' : channel.name }}</span>
              <span @click="disableChannel(channel)" class="btn" title="关闭频道">隐藏</span>
            </li>
          </ul>
        </div>
      </div>
    </transition> 

    <transition name="slideX">
      <div v-if="iptvInfo.showChannelGroupList && channelGroupList && channelGroupList.length > 0"
        class="list" v-ClickOutside="closeListEvent">
        <div class="list-top">
          <span class="list-top-title">频道列表</span>
          <span class="list-top-close zy-svg" @click="iptvInfo.showChannelGroupList = false">
            <svg-icon name="close"></svg-icon>
          </span>
        </div>
        <div class="list-body vop-scroll" :style="{overflowY: moviesInfo.scroll? 'auto' : 'hidden',paddingRight: moviesInfo.scroll ? '0': '5px' }"
           @mouseenter="moviesInfo.scroll = true" @mouseleave="moviesInfo.scroll = false">
          <el-input
            clearable
            size="small" title="支持按拼音首字母搜索"
            v-model.trim="playerInfo.searchTxt"
            @change="searchTxtChange"
            placeholder="搜索">
          <i slot="prefix" class="el-input__icon el-icon-search"></i>
          </el-input>
          <el-tree ref="channelTreeRef"
            :data="channelGroupTree"
            :props="iptvInfo.defaultProps"
            accordion
            :filter-node-method="filterNode"
            @node-click="handleNodeClick">
          </el-tree>
        </div>
      </div>
    </transition>
  </div>
</template>
<script>
import { defineComponent, reactive, ref, onMounted, onBeforeMount, onBeforeUnmount, watch, toRefs } from 'vue';
import { useCoreStore } from "@/store";
import { useIptvStore } from "@/store/iptv";
import { useMoviesStore } from "@/store/movies";
import { useHistoryStore } from "@/store/history";
import moviesApi from '@/api/movies';
import { ElMessage, ClickOutside } from 'element-plus';
import mt from 'mousetrap';
import PinyinMatch from 'pinyin-match';
import { _ as lodash } from 'lodash';
import { storeToRefs } from 'pinia';
import { MoviesPlayer, getPlayerType, getIsVipMovies } from '@/business/play';
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import { appWindow } from '@tauri-apps/api/window';
import date from '@/util/date';
import SvgIcon from '@/components/SvgIcon.vue';

const VIDEO_DETAIL_CACHE = {}
export default defineComponent({
  name: 'Play',
  components: {
    SvgIcon
  },
  setup() {
    const playerInfo = reactive({
      searchTxt: '',
      skipendStatus: false,
      right: {
        show: false,
        type: "",
      },
      isLive: false,
      isPlaying: false,
    })
    const iptvInfo = reactive({
      showChannelGroupList: false,
      changingIPTV: false,
      defaultProps: {
        label: 'label',
        children: 'children'
      },
    });

    const moviesInfo = reactive({
      showTimespanSetting: false,
      moviesList: [],
      otherSiteMoviesSources: [],
      startPosition: { min: '00', sec: '00' }, // 对应调略输入框
      endPosition: { min: '00', sec: '00' },
      exportablePlaylist: false,
      currentTime: 0,
      isStar: false,
      scroll: false,
      currentMoviesFullList: [],
      moviesFullListIndex: 0,
      iframeFullScreen: false,
      manualClosePlayer: false,
    });

    let player = null;
    const coreStore = useCoreStore();
    const { getPlayerConf, updatePlayerConf, getAllShortcut, updateSystemConf } = coreStore;
    const { view, video, iptv, playerConf, shortcutList, systemConf } = storeToRefs(coreStore);

    const iptvStore = useIptvStore();
    const { getAllChannelGroup, getCurrentChannel } = iptvStore;
    const { channelGroupId, channelGroupList, channelGroupTree, getSourcePlayList, channelId } = storeToRefs(iptvStore);

    const moviesStore = useMoviesStore();
    const { getSiteByKey, getAllSite } = moviesStore;
    const { moviesDetailCache, movieInfo, siteList, detail } = storeToRefs(moviesStore);

    const historyStore = useHistoryStore();
    const { refreshHistoryList, getAllHistory } = historyStore;
    const { historyList } = toRefs(historyStore);

    const historyTimer = ref();
    
    const channelTreeRef = ref();
    
    const getPlayer = (videoUrl, force = false) => {
      const playerType = getPlayerType(videoUrl);
      if (force || !player || (!player.dp || playerType !== player.playerType)) {
        closePlayer();
        player = new MoviesPlayer(playerType, playerInfo, playerConf.value);
        bindEvent();
      }
    }

    const playChannel = (channelGroup) => {
      moviesInfo.moviesList = []
      let channel;
      if (channelGroup.channels) {
        if (channelId.value) {
          channel = lodash.find(channelGroup.channels,  {id: channelId.value});
        } 
        if (!channel) {
          channel = channelGroup.channels.filter(e => e.active)[0]
          channelId.value = channel.id;
        }
      } else {
        return
      }

      iptvInfo.changingIPTV = true // 避免二次执行playChannel
      channelGroupId.value = channel.channelGroupId;
      movieInfo.value.name = channelGroup.name
      getPlayer(channel.url);
      player.dp.switchVideo({url: channel.url, type: player.dpConfig.video.type})
      iptvInfo.changingIPTV = false
    }

    const handleNodeClick = (node) => {
      if (node.channelGroup) {
        playChannel(node.channelGroup)
      }
    }

    const getUrls = async () => {
      playerInfo.isPlaying = true;
      if (!player || !player.dp) getPlayer()
      movieInfo.value.name = ''
      movieInfo.value.onlineUrl = ''
      if (historyTimer.value) {
        clearInterval(historyTimer.value)
        historyTimer.value = null
      }

      if (video.value.playType === 'iptv') {
        // 是直播源，直接播放
        playChannel(getCurrentChannel())
      } else {
        iptvInfo.showChannelGroupList = false
        const key = getMoviesUq();
        let time = movieInfo.value.time
        moviesInfo.startPosition = { min: '00', sec: '00' }
        moviesInfo.endPosition = { min: '00', sec: '00' }
        let historyStr = await invoke("get_history_by_uq", getMoviesUq(2))
        if (historyStr) {
          const history = JSON.parse(historyStr);
          if (!time && history.index === movieInfo.value.index) {
            time = history.playTime
          }
          
          if (!movieInfo.value.index) movieInfo.value.index = history.index;
          if (!VIDEO_DETAIL_CACHE[key]) VIDEO_DETAIL_CACHE[key] = {}
          if (!movieInfo.value.videoFlag) movieInfo.value.videoFlag = history.videoFlag
          if (history.startPosition) { // 数据库保存的时长通过快捷键设置时可能为小数, this.startPosition为object对应输入框分秒转化到数据库后肯定为整数
            VIDEO_DETAIL_CACHE[key].startPosition = history.startPosition
            moviesInfo.startPosition = { min: '' + parseInt(history.startPosition / 60), sec: '' + parseInt(history.startPosition % 60) }
          }
          if (history.endPosition) {
            VIDEO_DETAIL_CACHE[key].endPosition = history.endPosition
            moviesInfo.endPosition = { min: '' + parseInt(history.endPosition / 60), sec: '' + parseInt(history.endPosition % 60) }
          }
        }
        const index = movieInfo.value.index || 0
        playVideo(index, time)
      }
    }

    const moviesFullListChange = async (index) => {
      moviesInfo.moviesList = moviesInfo.currentMoviesFullList[index].list;
      moviesInfo.moviesFullListIndex = index;
      let historyStr = await invoke("get_history_by_uq", getMoviesUq(2))
      if (historyStr) {
        let history = JSON.parse(historyStr);
        history.videoFlag = moviesInfo.currentMoviesFullList[index].flag;
        history.updateTime = date.getDateTimeStr(); 
        await invoke("save_history", { history: history })
        refreshHistoryList();
      }
    }

    const playVideo = (index = 0, time = 0) => {
      playerInfo.isLive = false
      moviesInfo.isStar = false
      moviesInfo.exportablePlaylist = false
      fetchPlaylist().then(async (fullList) => {
        moviesInfo.currentMoviesFullList = fullList;
        let playlist = fullList[0].list // ZY支持的已移到首位
        // 如果设定了特定的video flag, 获取该flag下的视频列表
        const videoFlag = movieInfo.value.videoFlag
        if (videoFlag) {
          fullList.forEach((x, index) => {
            if (x.flag == videoFlag) {
              playlist = x.list
              moviesInfo.moviesFullListIndex = index;
            }
          })
        }
        moviesInfo.moviesList = playlist
        const url = playlist[index].includes('$') ? playlist[index].split('$')[1] : playlist[index]
        if (playlist.every(e => e.includes('$') ? e.split('$')[1].endsWith('.m3u8') : e.endsWith('.m3u8'))) moviesInfo.exportablePlaylist = true
        if (!url.endsWith('.m3u8') && !url.endsWith('.mp4')) {
          if (getIsVipMovies(url)) {
            const websiteParseList = await invoke("select_website_parse", {})
            ElMessage.info('即将调用解析接口播放，请等待...');
            movieInfo.value.onlineUrl = websiteParseList[0].websiteParseUrl + url;
          } else {
            movieInfo.value.onlineUrl = url;
          }
          player.destroy();
          videoPlaying('online');
          return
        } else {
          const key = getMoviesUq();
          getPlayer(url, true)
          player.dp.switchVideo({url: url, type: player.dpConfig.video.type})
          const startTime = VIDEO_DETAIL_CACHE[key].startPosition || 0
          player.dp.seek(time > startTime ? time : startTime)
        }
        if (document.querySelector('xg-btn-showhistory')) document.querySelector('xg-btn-showhistory').style.display = 'block'
        if (document.querySelector('.xgplayer-playbackrate')) document.querySelector('.xgplayer-playbackrate').style.display = 'inline-block'
        videoPlaying()
        playerInfo.skipendStatus = false
      })
    }

    const videoPlaying = async (isOnline) => {
      let historyStr = await invoke("get_history_by_uq", getMoviesUq(2))
      const videoFlag = movieInfo.value.videoFlag || ''
      let time, duration;
      let startPosition = 0
      let endPosition = 0
      if (isOnline) {
        time = duration = 0
      } else {
        time = player.currentTime();
        duration = player.duration();
      }
      if (!historyStr) {
        const history = {
          id: 0,
          name: movieInfo.value.name,
          siteKey: movieInfo.value.siteKey,
          ids: movieInfo.value.ids.toString(),
          index: movieInfo.value.index,
          playTime: time,
          duration: duration,
          startPosition: startPosition,
          endPosition: endPosition,
          detail: JSON.stringify(moviesDetailCache.value[getMoviesUq()]),
          onlinePlay: isOnline ? movieInfo.value.onlineUrl : "",
          videoFlag: videoFlag,
          hasUpdate: "0",
          updateTime: date.getDateTimeStr(),
        }
        await invoke("save_history", {history: history})
      }
      if (isOnline) {
        const iframeElement = document.querySelector('iframe'); // 获取iframe元素
        iframeElement.addEventListener('fullscreenchange', async (event) => {
          if (!moviesInfo.iframeFullScreen) {
            await appWindow.setFullscreen(true);
          } else {
            await appWindow.setFullscreen(false);
            await appWindow.unmaximize();
          }
          moviesInfo.iframeFullScreen = !moviesInfo.iframeFullScreen
        });
        let history = JSON.parse(historyStr);
        history.index = movieInfo.value.index;
        history.onlinePlay = movieInfo.value.onlineUrl;
        history.updateTime = date.getDateTimeStr(); 
        await invoke("save_history", {history: history});
        refreshHistoryList();
      } else {
        timerEvent();
      }
    }
    
    const fetchPlaylist = () => {
      return new Promise((resolve) => {
        const cacheKey = getMoviesUq()
        if (VIDEO_DETAIL_CACHE[cacheKey] && VIDEO_DETAIL_CACHE[cacheKey].list && VIDEO_DETAIL_CACHE[cacheKey].list.length) {
          movieInfo.value.name = VIDEO_DETAIL_CACHE[cacheKey].name
          resolve(VIDEO_DETAIL_CACHE[cacheKey].list)
        }
        let res
        if (!moviesDetailCache.value[cacheKey]) {
          moviesApi.detail(getSiteByKey(movieInfo.value.siteKey, 2), movieInfo.value.ids).then(res => {
            moviesDetailCache.value[cacheKey] = res
            res = moviesDetailCache.value[cacheKey]
            movieInfo.value.name = res.name
            VIDEO_DETAIL_CACHE[cacheKey] = Object.assign(VIDEO_DETAIL_CACHE[cacheKey] || { }, {
              list: res.fullList,
              name: res.name
            })
            resolve(res.fullList)
          }).catch(err => { 
            ElMessage({showClose: true, message: '播放地址可能已失效，请换源并调整收藏', type: 'error',});
            movieInfo.value.name = movieInfo.value.name; 
            otherEvent();
          })
        } else {
          res = moviesDetailCache.value[cacheKey]
          movieInfo.value.name = res.name
          VIDEO_DETAIL_CACHE[cacheKey] = Object.assign(VIDEO_DETAIL_CACHE[cacheKey] || { }, {
            list: res.fullList,
            name: res.name
          })
          resolve(res.fullList)
        }
      })
    }

    // 播放下一集
    const prevNextEvent = (isReverse = false) => {
      if (video.value.playType === "iptv") {
        let index = lodash.findIndex(channelGroupList.value, ['id', channelGroupId.value]);
        if (isReverse) {
          index = index === 0 ? channelGroupList.value.length - 1 : index - 1
        } else {
          index = index === channelGroupList.value.length - 1 ? 0 : index + 1
        }
        const channel = channelGroupList.value[index]
        playChannel(channel)
      } else {
        if (isReverse) {
          if (movieInfo.value.index >= 1) {
            movieInfo.value.index--
            movieInfo.value.time = 0
          } else {
            ElMessage({showClose: true, message: '这已经是第一集了。', type: 'warning',})
          }
        } else if (movieInfo.value.index < (moviesInfo.moviesList.length - 1)) {
          movieInfo.value.index++
          movieInfo.value.time = 0
        } else {
          ElMessage({showClose: true, message: '这已经是最后一集了。', type: 'warning',})
        }
      }
    }

    // 定时更新历史记录时间
    const timerEvent = () => {
      historyTimer.value = setInterval(async () => {
        if (!player.playing) {
          return
        }
        let historyStr = await invoke("get_history_by_uq", getMoviesUq(2))
        if (historyStr) {
          let history = JSON.parse(historyStr);
          history.index = movieInfo.value.index;
          history.playTime = player.currentTime();
          history.duration = player.duration();
          history.updateTime = date.getDateTimeStr(); 
          await invoke("save_history", { history: history })
        }
      }, 10000)
    }
    
    // 自动关闭播发器
    const closePlayer = () => {
      if (player && player.dp) {
        player.dp.destroy();
        player.dp = null;
      }
    }

    // 手动关闭播放器
    const closePlayerAndInit = () => {
      playerInfo.isPlaying = false;
      moviesInfo.manualClosePlayer = true;
      movieInfo.value = {
        siteKey: "",
        ids: "",
        name: "",
        index: 0,
        onlineUrl: "",
      }
      moviesInfo.moviesList = [];
      if (player && player.dp) {
        player.dp.destroy();
        player.dp = null;
      }
      
      getPlayer("", true);
    }
    
    // 绑定事件
    const bindEvent = () => {
      let dp = player.dp;
      let dpConfig = player.dpConfig;
      // 直播卡顿时换源换台
      let stallIptvTimeout
      let stallCount = 0
      dp.on('waiting', () => {
        player.playing = false;
        if (dpConfig.isLive && playerConf.value.autoChangeSourceWhenIptvStalling) {
          stallIptvTimeout = setTimeout(() => {
            stallCount++
            if (stallCount >= 4) {
              stallCount = 0;
              prevNextEvent();
            }
          }, 5 * 1000)
        }
      })
      dp.on('canplay', () => {
        player.playing = true;
        stallCount = 0
        clearTimeout(stallIptvTimeout)
        if (dp.video.paused) {
          dp.play();
        }
      })
      dp.on('destroy', () => {
        stallCount = 0
        clearTimeout(stallIptvTimeout)
      })

      dp.on('volumechange', async () => {
        playerConf.value.volume = player.dp.video.volume;
        updatePlayerConf();
      })

      dp.on('timeupdate', () => {
        if (dpConfig.isLive) return
        const key = getMoviesUq();
        if (VIDEO_DETAIL_CACHE[key] && VIDEO_DETAIL_CACHE[key].endPosition) {
          const time = dp.video.duration - VIDEO_DETAIL_CACHE[key].endPosition - dp.video.currentTime
          if (time > 0 && time < 0.3) { // timeupdate每0.25秒触发一次，只有自然播放到该点时才会跳过片尾
            if (!playerInfo.skipendStatus) {
              playerInfo.skipendStatus = true
              const event = new Event("ended");
              dp.container.querySelector('.dplayer-video-current').dispatchEvent(event);
            }
          }
        }
      })

      dp.on('play', () => {
        clearTimeout(stallIptvTimeout)
        if (!movieInfo.value.siteKey && !channelGroupId.value) {
          if (video.value.playType !== "iptv" && !movieInfo.value.ids) {
            // 如果当前播放页面的播放信息没有被赋值,播放历史记录
            if (historyList.value.length === 0) {
              ElMessage({showClose: true, message: '历史记录为空，无法播放！', type: 'warning',})
              return
            }
            const historyItem = historyList.value[0]
            video.value = { isLive: false, playType: "movies"}
            movieInfo.value = {
              siteKey: historyItem.siteKey,
              ids: historyItem.ids, 
              name: historyItem.name,
              index: historyItem.index,
            }
          } else if (video.value.playType === "iptv") {
            playChannel(getCurrentChannel())
            iptvInfo.showChannelGroupList = false
          }
        }
      })

      dp.on('playing', lodash.once(() => {
        if (movieInfo.value.siteKey) {
          const key = getMoviesUq();
          dpConfig.highlight = [{
            time: VIDEO_DETAIL_CACHE[key]["startPosition"],
            text: '片头',
          }, {
            time: dp.video.duration - VIDEO_DETAIL_CACHE[key]["endPosition"],
            text: '片尾',
          }]
          player.durationchange();
        }
      }))

      dp.on('ended', lodash.debounce(() => {
        if (moviesInfo.moviesList.length > 1 && (moviesInfo.moviesList.length - 1 > movieInfo.value.index)) {
          movieInfo.value.time = 0
          movieInfo.value.index++
        }
      }, 1000))

      dp.on('durationchange', () => {
        if (dp.video.duration !== 1 && dp.video.duration !== Infinity) {
          let highlight = dpConfig.highlight
          let playedBarWrap = dp.container.querySelector('.dplayer-bar-wrap');
          let playedBarTime = dp.container.querySelector('.dplayer-bar-time');
          const highlights = playedBarWrap.querySelectorAll('.dplayer-highlight');
          [].slice.call(highlights, 0).forEach((item) => {
            playedBarWrap.removeChild(item);
          });
          for (let i = 0; i < highlight.length; i++) {
            if (!highlight[i].text || !highlight[i].time) {
                continue;
            }
            const p = document.createElement('div');
            p.classList.add('dplayer-highlight');
            p.style.left = (highlight[i].time / dp.video.duration) * 100 + '%';
            p.innerHTML = '<span class="dplayer-highlight-text">' + highlight[i].text + '</span>';
            playedBarWrap.insertBefore(p, playedBarTime);
          }
        }
      });

      dp.on('playPrev', () => {
        prevNextEvent(true);
      })

      dp.on('playNext', () => {
        prevNextEvent();
      })
      
      dp.on('fullscreen', async () => {
        await appWindow.setFullscreen(true);
      })

      dp.on('fullscreen_cancel', async () => {
        await appWindow.setFullscreen(false);
        await appWindow.unmaximize();
      })
    }
    
    const otherEvent = () => {
      if (video.value.playType != 'iptv') {
        playerInfo.right.type = 'other'
        getOtherSites()
        moviesInfo.currentTime = player.dp ? player.dp.video.currentTime : 0
      } else {
        playerInfo.right.type = 'sources'
      }
      playerInfo.right.show = true
    }

    const getOtherSites = async () => {
      moviesInfo.otherSiteMoviesSources = []
      const currentSite = getSiteByKey(movieInfo.value.siteKey, 2);
      await getAllSite();
      // 排除已关闭的源和当前源
      for (const siteItem of siteList.value.filter(x => x.isActive && x.group === currentSite.group && x.key !== movieInfo.value.siteKey)) {
        moviesApi.search(siteItem, movieInfo.value.name).then(searchRes => {
          const type = Object.prototype.toString.call(searchRes)
          if (type === '[object Array]') {
            searchRes.forEach(async item => {
              const detailRes = item
              detailRes.key = siteItem.key
              detailRes.site = siteItem
              moviesInfo.otherSiteMoviesSources.push(detailRes)
            })
          }
          if (type === '[object Object]') {
            const detailRes = searchRes
            detailRes.key = siteItem.key
            detailRes.site = siteItem
            moviesInfo.otherSiteMoviesSources.push(detailRes)
          }
        })
      }
    }
    
    const closeListEvent = () => {
      const lastRightType = playerInfo.right.type
      const lastChannelGroupListState = iptvInfo.showChannelGroupList
      setTimeout(() => {
        if (lastRightType === playerInfo.right.type) {
          playerInfo.right.show = false
          playerInfo.right.type = ''
        }
        if (lastChannelGroupListState === iptvInfo.showChannelGroupList) {
          iptvInfo.showChannelGroupList = false
        }
      }, 50)
    }

    const showShortcutEvent = () => {
      if (playerInfo.right.type === 'shortcut') {
        playerInfo.right.show = false
        playerInfo.right.type = ''
      } else {
        playerInfo.right.show = true
        shortcut.all().then(res => {
          playerInfo.right.type = 'shortcut'
          playerInfo.right.shortcut = res
        })
      }
    }

    const moviesListEvent = () => {
      if (playerInfo.right.type === 'movieList') {
        playerInfo.right.show = false
        playerInfo.right.type = ''
      } else {
        playerInfo.right.show = true
        playerInfo.right.type = 'movieList'
      }
    }
    
    const historyEvent = () => {
      if (playerInfo.right.type === 'history') {
        playerInfo.right.show = false
        playerInfo.right.type = ''
      } else {
        playerInfo.right.show = true
        playerInfo.right.type = 'history'
      }
      refreshHistoryList()
    }
    
    const issueEvent = async () => {
      const currentSite = getSiteByKey(2);
      const playInfo = {
        movies: Object.assign({ siteInfo: currentSite, moviesDetail: moviesDetailCache[getMoviesUq()] }, movieInfo.value),
        playlist: moviesInfo.moviesList.map(e => e.split('$')[1]),
        playerType: moviesInfo.onlineUrl ? '在线解析' : player.dp.video.type,
        playerError: player.dp.error || '',
        playerState: player.dp.readyState || '',
        networkState: player.dp.networkState || ''
      }
      
      await writeText(JSON.stringify(playInfo, null, 4));
      ElMessage({showClose: true, message: '视频信息复制成功', type: 'success',});
    }

    const getMoviesUq = (type = 1) => {
      if (type == 1) {
        return movieInfo.value.siteKey + '@' + movieInfo.value.ids;
      } else {
        return { siteKey: movieInfo.value.siteKey, ids: movieInfo.value.ids.toString() }
      }
    }

    const starEvent = async () => {
      const statStr = await invoke("get_star_by_uq", getMoviesUq(type = 2));
      if (statStr) {
        await invoke("del_star", JSON.parse(statStr).id);
        moviesInfo.isStar = false;
        ElMessage({showClose: true, message: '取消收藏成功', type: 'success',});
      } else {
        const star = {
          name: movieInfo.value.name,
          ids: movieInfo.value.ids.toString(),
          siteKey: movieInfo.value.siteKey,
          detail: JSON.stringify(moviesDetailCache.value[getMoviesUq()]),
        }
        await invoke("save_star", {star: star});
        ElMessage({showClose: true, message: '收藏成功', type: 'success',});
        moviesInfo.isStar = true;
      }
    }
    
    const historyItemEvent = (history) => {
      movieInfo.value = {
        siteKey: history.siteKey,
        ids: history.ids,
        name: history.name,
        index: history.index
      }
      playerInfo.right.show = false
      playerInfo.right.type = ''
    }

    const setProgressDotByInput = async () => {
      const startTime = parseInt(moviesInfo.startPosition.min) * 60 + parseInt(moviesInfo.startPosition.sec)
      const endTime = parseInt(moviesInfo.endPosition.min) * 60 + parseInt(moviesInfo.endPosition.sec)
      const videoDuration = player.dp.video.duration;
      if (startTime > videoDuration || endTime > videoDuration) {
        ElMessage.error('设置的跳跃时间长度已超过视频播放时长，请调整设置！');
        return
      }

      const key = getMoviesUq();
      let historyStr = await invoke("get_history_by_uq", getMoviesUq(2))
      player.dpConfig.highlight = []
      const history = JSON.parse(historyStr);
      if (startTime != 0) {
        VIDEO_DETAIL_CACHE[key]["startPosition"] = startTime
        history["startPosition"] = startTime
        player.dpConfig.highlight.push({time: startTime, text: '片头',})
      }

      if (endTime != 0 ) {
        VIDEO_DETAIL_CACHE[key]["endPosition"] = endTime;
        history["endPosition"] = endTime
        player.dpConfig.highlight.push({time: videoDuration - endTime, text: '片尾',})
      }
      
      if (startTime != 0 || endTime != 0) {
        player.durationchange();
        history.updateTime = date.getDateTimeStr(); 
        await invoke("save_history", { history: history} );
        refreshHistoryList();
      }
    }
    
    const listItemEvent = (n) => {
      if (video.value.playType == "iptv") {
        const channel = channelGroupList.value[n]
        // 是直播源，直接播放
        playChannel(channel)
      } else {
        movieInfo.value.index = n
        movieInfo.value.videoFlag = moviesInfo.currentMoviesFullList[moviesInfo.moviesFullListIndex].flag;
        playerInfo.right.show = false
        playerInfo.right.type = ''
      }
    }

    const detailEvent = () => {
      detail.value = {
        show: true,
        siteKey: movieInfo.value.siteKey,
        ids: movieInfo.value.ids,
        index: movieInfo.value.index,
      }
    }
    
    const ftName = (e, n) => {
      const num = e.split('$')
      if (num.length > 1) {
        return e.split('$')[0]
      } else {
        return `第${(n + 1)}集`
      }
    }

    const clearAllHistory = async () => {
      await invoke("del_all_history", {});
      refreshHistoryList();
    }

    const otherItemEvent = (e) => {
      video.value.playType = "movies";
      // 打开当前播放的剧集index, 定位到当前的时间
      movieInfo.value = {
        siteKey: e.site.key,
        ids: e.id,
        index: movieInfo.value.index,
        name: e.name,
        videoFlag: "",
      }
    }
    
    const clearPosition = () => {
      player.dpConfig.highlight = [];
      player.durationchange();
    }

    const removeHistoryItem = async (history) => {
      await invoke("del_history", {id: history.id});
      await refreshHistoryList();
    }
    
    const filterNode = (value, data) => {
      if (!value) return true
      return PinyinMatch.match(data.label, value)
    }
    
    const disableChannel = (channel) => {
      const index = this.right.sources.indexOf(channel)
      this.right.sources.splice(index, 1)
      const ele = this.channelList.find(e => e.id === channel.channelID)
      const origin = ele.channels.find(e => e.id === channel.id)
      origin.isActive = false
      ele.isActive = ele.channels.some(e => e.isActive)
      channelList.remove(ele.id)
      channelList.add(ele)
    }

    const exportM3u8 = () => {
      const m3u8Arr = []
      for (const i of moviesInfo.moviesList) {
        const j = i.split('$')
        let link, name
        if (j.length > 1) {
          for (let m = 0; m < j.length; m++) {
            if (j[m].indexOf('.m3u8') >= 0 && j[m].startsWith('http')) {
              link = j[m]
              break
            }
          }
          name = j[0]
        } else {
          name = `第${m3u8Arr.length + 1}集`
          link = j[0]
        }
        m3u8Arr.push({
          name: name,
          link: link
        })
      }
      let m3u8Content = '#EXTM3U\n'
      for (const item of m3u8Arr) {
        m3u8Content += `#EXTINF:-1, ${item.name}\n${item.link}\n`
      }
      const blob = new Blob([m3u8Content], { type: 'application/vnd.apple.mpegurl' })
      const downloadElement = document.createElement('a')
      const href = window.URL.createObjectURL(blob)
      downloadElement.href = href
      downloadElement.download = `${movieInfo.value.name}.m3u`
      document.body.appendChild(downloadElement)
      downloadElement.click()
      document.body.removeChild(downloadElement)
      window.URL.revokeObjectURL(href)
    }
    
    const searchTxtChange = () => {
      channelTreeRef.value.filter(playerInfo.searchTxt)
    }
    
    const mtEvent = () => {
      if (systemConf.value.shortcutModified) mt.reset();
      shortcutList.value.forEach(shortcut => {
        mt.bind(shortcut.key, () => {
          if (view.value === 'Play') {
            shortcutEvent(shortcut.name)
          }
        })
      })
      systemConf.value.shortcutModified= false
      updateSystemConf();
    }

    const shortcutEvent = (shortcutName) => {
      switch (shortcutName) { 
        case 'playAndPause':
          if (player.videoExist()) {
            player.dp.toggle();
          }
          break
        case 'forward':
          if (player.videoExist() && !player.dp.video.paused) {
            player.dp.video.currentTime += parseInt(playerConf.value.forwardTimeInSec)
          }
          break
        case 'back':
          if (player.videoExist() && !player.dp.video.paused) {
            player.dp.video.currentTime -= parseInt(playerConf.value.forwardTimeInSec)
          }
          break
        case 'volumeUp':
          if (player.playerExist()) {
            let volume = player.dp.video.volume + 0.1 > 1 ? 1 : player.dp.video.volume + 0.1;
            player.dp.volume(volume, true, false);
          }
          break
        case 'volumeDown':
          if (player.playerExist()) {
            let volume = player.dp.video.volume - 0.1 < 0 ? 0 : player.dp.video.volume - 0.1;
            player.dp.volume(volume, true, false)
          }
          break
        case 'next':
          nextEvent()
          break
        case 'prev':
          prevEvent()
          break
        case 'playbackRateUp':
          if (player.videoExist() && !player.dp.video.paused) {
            const rate = player.dp.playbackRate
            player.dp.playbackRate = rate + 0.25
            ElMessage.info('当前播放速度为: ' + player.dp.playbackRate);
          }
          break
        case 'playbackRateDown':
          if (player.videoExist() && !player.dp.video.paused) {
            const rate = player.dp.playbackRate
            if (rate > 0.25) {
              player.dp.playbackRate = rate - 0.25
              ElMessage.info('当前播放速度为: ' + player.dp.playbackRate);
            }
          }
          break
        default:
          break
      }
      return false;
    }

    const leadingZero = (time) => { // 格式化单个调略输入框
      Object.keys(time).forEach(key => {
        if (time[key] > 59 || time[key] < 0) {
          time[key] = '00'
        } else if (time[key].length > 2) {
          time[key] = '' + parseInt(time[key])
        } else if (time[key] < 10 && time[key].length === 1) {
          time[key] = '0' + time[key]
        }
      })
    }
    
    const fmtMSS = (s) => {
      return (s - (s %= 60)) / 60 + (s > 9 ? ':' : ':0') + s
    }
    
    watch(() => {
      if (video.value.playType == "movies") return movieInfo.value.siteKey + '@' + movieInfo.value.ids + '@' + movieInfo.value.index
      if (video.value.playType == "iptv") return channelGroupId.value;
    }, () => {
      if (playerInfo.changingIPTV) return
      // 手动关闭播放器，直接返回
      if (moviesInfo.manualClosePlayer) {
        moviesInfo.manualClosePlayer = false;
        return
      }
      getUrls();
    }, { deep: true })

    watch((view.value),
     () => {
      if (view.value === 'Play') {
        playerInfo.right.show = false
        playerInfo.right.type = ''
        getAllChannelGroup()
        if (playerInfo.playType == "") {
          iptvInfo.showChannelGroupList = true
        }
      }
    }, { deep: true })

    onBeforeMount(async () => {
      getPlayerConf();
      await getAllShortcut();
      mtEvent()
      await getAllHistory();
    })

    onMounted(async () => {
      if (video.value.playType) {
        if (video.value.playType == "iptv") {  
          iptvInfo.showChannelGroupList = true
        }
        getUrls();
      } else {
        getPlayer("", true)
      }
    });
    
    onBeforeUnmount(() => {
      if (historyTimer.value) {
        clearInterval(historyTimer.value)
      }
      closePlayer();
    });

    return {
      playerInfo,
      iptvInfo,
      iptv,
      moviesInfo,
      movieInfo,
      video,
      otherEvent,
      closeListEvent,
      showShortcutEvent,
      moviesListEvent,
      historyEvent,
      handleNodeClick,
      issueEvent,
      starEvent,
      historyList,
      historyItemEvent,
      setProgressDotByInput,
      listItemEvent,
      ftName,
      getSiteByKey,
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
      getSourcePlayList,
      channelId,
      shortcutList,
      fmtMSS,
    }
  },
  directives: {
    ClickOutside
  }
});

// export default {
//   computed: {
//     share: {
//       get () {
//         return this.$store.getters.getShare
//       },
//       set (val) {
//         this.SET_SHARE(val)
//       }
//     },
//   },
//   watch: {
//     startPosition: {
//       handler (time) {
//         this.leadingZero(time)
//       },
//       deep: true
//     },
//     endPosition: {
//       handler (time) {
//         this.leadingZero(time)
//       },
//       deep: true
//     }
//   },
//   methods: {
//     shareEvent () {
//       this.share = {
//         show: true,
//         key: this.video.key,
//         info: this.DetailCache[this.video.key + '@' + this.video.info.id],
//         index: this.video.info.index
//       }
//     },
// }
</script>
<style>
.dplayer {
  height: 100%;
}
</style>
<style lang="scss" scoped>
.play{
  position: relative;
  height: calc(100% - 60px);
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 5px;
  .box{
    width: 100%;
    height: 100%;
    display: flex;
    border-radius: 5px;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    padding: 0 10px;
    .title{
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
    .player{
      width: 100%;
      flex: 1;
      overflow: hidden;
    }
    .more{
      width: 100%;
      height: 50px;
      min-height: 50px;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      span{
        display: flex;
        margin-right: 10px;
        cursor: pointer;
      }
      .timespan{
        margin-left: auto;
        justify-content: space-between;
        align-items: center;
        input{
          border: none;
          &::-webkit-inner-spin-button, &::-webkit-outer-spin-button {
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
  .list{
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
    .el-tree{
      background-color: inherit;
    }
    .list-top{
      display: flex;
      justify-content: space-between;
      align-items: center;
      height: 30px;
      .list-top-title{
        font-size: 16px;
      }
      .list-top-close{
        display: inline-block;
        cursor: pointer;
      }
    }
    .list-body{
      flex: 1;
      overflow-y: auto;
      ul{
        margin: 0;
        padding: 0;
        list-style: none;
        li{
          position: relative;
          height: 28px;
          width: 100%;
          line-height: 28px;
          padding-left: 10px;
          font-size: 14px;
          cursor: pointer;
          display: flex;
          .title{
            display: inline-block;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            width: 231px;
          }
          .btn{
            display: inline-block;
          }
          .detail-delete{
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
  .slideX-enter-active, .slideX-leave-active{
    transition: all .5s ease-in-out;
  }
  .slideX-enter, .slideX-leave-to{
    transform: translateX(100%);
    opacity: 0;
  }
}
</style>
