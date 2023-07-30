<template>
  <div class="listpage" id="history">
    <div class="listpage-header" id="history-header">
        <el-button @click.stop="exportHistory" icon="el-icon-upload2" title="导出全部，自动添加扩展名">导出</el-button>
        <el-button @click.stop="importHistory" icon="el-icon-download" title="支持同时导入多个文件">导入</el-button>
        <el-button @click.stop="removeSelectedItems" icon="el-icon-delete-solid">
          {{ historyInfo.multipleSelection.length === 0 ? "清空" : "删除所选" }}
        </el-button>
        <el-button-group>
          <el-switch v-model="historyInfo.onlyShowItemsHasUpdate" active-text="有更新" inactive-text="全部" @change="refreshFilteredList"></el-switch>
          <el-button @click.stop="updateAllEvent" icon="el-icon-refresh">检查更新</el-button>
        </el-button-group>
    </div>
   <div class="toolbar" v-show="historyInfo.showToolbar">
      <el-select v-model="historyInfo.selectedAreas" multiple placeholder="地区" popper-class="popper" :popper-append-to-body="false" 
        @remove-tag="refreshFilteredList" @change="refreshFilteredList">
        <el-option
          v-for="item in history.areas"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <el-select v-model="historyInfo.selectedTypes" multiple placeholder="类型" popper-class="popper" :popper-append-to-body="false" 
        @remove-tag="refreshFilteredList" @change="refreshFilteredList">
        <el-option
          v-for="item in history.types"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <el-select v-model="historyInfo.sortKeyword" placeholder="排序" popper-class="popper" :popper-append-to-body="false" 
        @change="refreshFilteredList">
        <el-option
          v-for="item in historyInfo.sortKeywords"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <span>
       上映区间：
      <el-input-number v-model="historyInfo.selectedYears.start" :min=0 :max="new Date().getFullYear()" controls-position="right" 
        step-strictly @change="refreshFilteredList">
      </el-input-number>
       至
      <el-input-number v-model="historyInfo.selectedYears.end" :min=0 :max="new Date().getFullYear()" controls-position="right" 
        step-strictly @change="refreshFilteredList"></el-input-number>
      </span>
    </div>
    <el-divider class="listpage-header-divider" content-position="right">
      <el-button link size="small" @click="toggleViewMode">视图切换</el-button>
      <el-button link size="small" @click='() => { historyInfo.showToolbar = !historyInfo.showToolbar; 
        if (!historyInfo.showToolbar) refreshFilteredList() }' title="收起工具栏会重置筛选排序">
        {{ historyInfo.showToolbar ? '隐藏工具栏' : '显示工具栏' }}
      </el-button>
      <el-button link size="small" @click="backTop">回到顶部</el-button>
    </el-divider>
    <div class="listpage-body" id="history-body">
      <div class="show-table" id="history-table" v-if="systemConf.historyViewMode === 'table'">
        <el-table size="small" fit height="100%"
          :data="historyInfo.historyFilterList"
          row-key="id"
          ref="historyTableRef"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
          @row-click="detailEvent">
          <el-table-column
            type="selection">
          </el-table-column>
          <el-table-column
            prop="name"
            :show-overflow-tooltip="true"
            label="片名">
          </el-table-column>
          <el-table-column
            width="120"
            label="片源">
            <template #default="scope">
              <span>{{ getSiteByKey(scope.row.siteKey) }}</span>
            </template>
          </el-table-column>
          <el-table-column
            width="180"
            label="观看至">
            <template #default="scope">
              <span v-if="scope.row.detail && scope.row.detail.fullList[0].list && scope.row.detail.fullList[0].list.length > 1">
                第{{ scope.row.index + 1 }}集(共{{scope.row.detail.fullList[0].list.length}}集)
              </span>
            </template>
          </el-table-column>
          <el-table-column
            width="180"
            label="时间进度">
            <template #default="scope">
               <span v-if="scope.row.playTime && scope.row.duration">
                {{fmtMSS(scope.row.playTime.toFixed(0))}}/{{fmtMSS(scope.row.duration.toFixed(0))}} ({{progress(scope.row)}}%)
              </span>
               <span v-if="scope.row.onlinePlay">在线解析</span>
            </template>
          </el-table-column>
          <el-table-column
            label="操作"
            width="240"
            header-align="center"
            align="right">
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link>播放</el-button>
              <el-button @click.stop="shareEvent(scope.row)" link>分享</el-button>
              <el-button @click.stop="downloadEvent(scope.row)" link>下载</el-button>
              <el-button @click.stop="deleteEvent(scope.row)" link>删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div class="show-picture" id="star-picture" v-if="systemConf.historyViewMode === 'picture'">
        <Waterfall :list="historyInfo.historyFilterList" :gutter="20" :width="240"
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
          :animationDuration="0.5"
          backgroundColor="rgba(0, 0, 0, 0)">
            <template #item="{item}">
              <div class="card">
                <div class="img">
                  <div class="update" v-if="item.hasUpdate == '1'">
                    <span>有更新</span>
                  </div>
                  <ImageLazy v-if="item.detail && item.detail.pic" style="width: 100%" :url="item.detail.pic" @click="detailEvent(item)"/>
                  <div class="operate">
                    <div class="operate-wrap">
                      <span class="o-play" @click="playEvent(item)">播放</span>
                      <span class="o-share" @click="shareEvent(item)">分享</span>
                      <span class="o-star" @click="downloadEvent(item)">下载</span>
                      <span class="o-star" @click="deleteEvent(item)">删除</span>
                    </div>
                  </div>
                </div>
                <div class="name" @click="detailEvent(item)">{{item.name}}</div>
                <div class="info">
                 <span v-if="item.playTime && item.duration">
                    {{fmtMSS(item.playTime.toFixed(0))}}/{{fmtMSS(item.duration.toFixed(0))}} ({{progress(item)}}%)
                  </span>
                  <span v-if="item.onlinePlay">在线解析</span>
                  <span v-if="item.detail && item.detail.fullList[0].list !== undefined && item.detail.fullList[0].list.length > 1">
                    第{{ item.index + 1 }}集(共{{item.detail.fullList[0].list.length}}集)
                  </span>
                </div>
              </div>
            </template>
        </Waterfall>
      </div>
    </div>
    </div>
</template>
<script>
import { defineComponent, reactive, onBeforeMount, watch, onMounted, ref, nextTick } from 'vue';
import moviesApi from '@/api/movies';
import { useCoreStore } from "@/store";
import { useMoviesStore } from "@/store/movies";
import { useHistoryStore } from "@/store/history";
import { storeToRefs } from 'pinia';
import { _ } from 'lodash';
import { open, save } from '@tauri-apps/api/dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { writeText } from '@tauri-apps/api/clipboard';
import { invoke } from "@tauri-apps/api/tauri";
import { Waterfall } from 'vue-waterfall-plugin-next';
import 'vue-waterfall-plugin-next/dist/style.css';
import ImageLazy from '@/components/ImageLazy.vue';
import { ElMessage } from 'element-plus';

export default defineComponent({
  name: 'history',
  components: {
      Waterfall,
      ImageLazy
  },
  setup() {
    const coreStore = useCoreStore();
    const { getSystemConf, updateSystemConf } = coreStore;
    const { view, video, systemConf } = storeToRefs(coreStore);

    const moviesStore = useMoviesStore();
    const { getSiteByKey } = moviesStore;
    const { movieInfo, detail } = storeToRefs(moviesStore);

    const historyStore = useHistoryStore();
    const { refreshHistoryList } = historyStore;
    const { historyList } = storeToRefs(historyStore);

    const historyInfo = reactive({
      multipleSelection: [],
      historyFilterList: [],
      showToolbar: false,
      onlyShowItemsHasUpdate: false,
      movieNoUpdateNum: 0,
      sortKeyword: "",
      selectedAreas: [],
      selectedTypes: [],
      selectedYears: {
        start: 0,
        end: new Date().getFullYear(),
      },
      sortKeywords: ['按片名', '按上映年份', '按更新时间', '按完成度'],
      shiftDown: false,
      selectionBegin: "",
      selectionEnd: "",
    })

    const historyTableRef = ref();

    const exportHistory = async () => {
      const arr = [...historyList.value]
      const str = JSON.stringify(arr, null, 2)
      let filePath = await save({
        filters: [{ name: 'JSON file', extensions: ['json'] }],
      });
      if (filePath !== null) {
        if (!filePath.endsWith('.json')) filePath += '.json'
        await writeTextFile(filePath, str);
        ElMessage({showClose: true, message: '已保存成功', type: 'success',});
      }
    }

    const importHistory = async () => {
      const selectedFiles = await open({
        multiple: true,
        filters: [{ name: 'JSON file', extensions: ['json'] }]
      });
      let filePaths = [];
      if (Array.isArray(selectedFiles)) {
        filePaths = filePaths.concat(selectedFiles);
      } else if (selectedFiles != null) {
        filePaths.push(selectedFiles);
      }
      filePaths.forEach(async file => {
        const str = await readTextFile(file);
        const json = JSON.parse(str)
        json.forEach(record => {
          if (record.detail && record.detail.m3u8List) {
            record.detail.fullList = [].concat({ flag: 'm3u8', list: record.detail.m3u8List })
            delete record.detail.m3u8List
            record.detail = JSON.stringify(record.detail)
          }
        })
        await invoke("insert_historys", { historys: json });
        updateHistoryList();
        ElMessage.success('导入历史记录成功');
      })
    }

    const removeSelectedItems = async () => {
      if (!historyInfo.multipleSelection.length) historyInfo.multipleSelection = historyList.value;
      await invoke("del_historys", { ids: _.map(historyInfo.multipleSelection, "id") });
      historyInfo.multipleSelection = []
      updateHistoryList();
    }

    const playEvent = (e) => {
      if (e.hasUpdate) {
        clearHasUpdateFlag(e)
      }
      video.value.playType = "movies";
      movieInfo.value = {
        siteKey: e.siteKey,
        ids: e.ids,
        name: e.name,
        index: e.index,
        onlineUrl: e.onlinePlay,
      }
      view.value = "Play";
    }
    
    const toggleViewMode = () => {
      systemConf.value.historyViewMode = systemConf.value.historyViewMode === 'picture' ? 'table' : 'picture'
      if (systemConf.value.historyViewMode === 'table') {
        showShiftPrompt()
      }
      updateSystemConf();
    }
    
    const showShiftPrompt = () => {
      if (systemConf.value.shiftTooltipLimitTimes) {
        ElMessage.info('多选时支持shift快捷键');
        systemConf.value.shiftTooltipLimitTimes--;
        updateSystemConf();
      }
    }

    const refreshFilteredList = () => {
      if (!historyInfo.showToolbar) {
        historyInfo.sortKeyword = ''
        historyInfo.selectedAreas = []
        historyInfo.selectedYears.start = 0
        historyInfo.selectedYears.end = new Date().getFullYear()
        historyInfo.historyFilterList = historyList.value;
      } else {
        let filteredData = historyList.value;
        filteredData = _.filter(filteredData, (x) => {
          let areaFilter = (historyInfo.selectedAreas.length === 0) || historyInfo.selectedAreas.includes(x.detail.area);
          let typeFilter = (historyInfo.selectedTypes.length === 0) || historyInfo.selectedTypes.includes(x.detail.type);
          return areaFilter && typeFilter
        })
        filteredData = _.filter(filteredData, (x) => {
          return x.detail.year >= historyInfo.selectedYears.start && x.detail.year <= historyInfo.selectedYears.end
        })
        switch (historyInfo.sortKeyword) {
          case '按上映年份':
            filteredData.sort(function (a, b) {
              return a.detail.year - b.detail.year
            })
            break
          case '按片名':
            filteredData.sort(function (a, b) {
              return a.detail.name.localeCompare(b.detail.name, 'zh')
            })
            break
          case '按更新时间':
            filteredData.sort(function (a, b) {
              return new Date(b.detail.last) - new Date(a.detail.last)
            })
            break
          case '按完成度':
            filteredData.sort(sortByProgress)
            break
          default:
            break
        }
        historyInfo.historyFilterList = filteredData
      }
      if (historyInfo.onlyShowItemsHasUpdate) {
        historyInfo.historyFilterList = historyInfo.historyFilterList.filter(x => x.hasUpdate === '1')
      }
    }
    
    const sortByProgress = (a, b) => {
      if (progress(a) < progress(b)) {
        return -1
      } else {
        return 1
      }
    }

    const updateAllEvent = () => {
      historyInfo.movieNoUpdateNum = 0
      historyList.value.forEach(async (his) => {
        updateEvent(_.cloneDeep(his));
      })
    }

    const updateEvent = async (his) => {
      try {
        moviesApi.detail(getSiteByKey(his.siteKey, 2), his.ids).then(async (newDetail) => {
          if (his.detail.last !== newDetail.last) {
            his.hasUpdate = "1"
            his.detail = newDetail;
            const msg = `检查到"${his.name}"有更新。`
            ElMessage.success(msg);
          } else {
            historyInfo.movieNoUpdateNum++;
          }
          his.detail = JSON.stringify(his.detail);
          await invoke("save_history", {history: his});
          
          updateHistoryList(2);
        }).catch(() => {
          historyInfo.movieNoUpdateNum++;
        });
      } catch (err) {
        const msg = `更新"${his.name}"失败, 请重试。`
        ElMessage.warning(msg);
      }
    }

    const updateHistoryList = _.debounce(async (refreshType = 1) => {
      if (refreshType == 2) { 
        if (historyInfo.movieNoUpdateNum == historyList.value.length) {
          ElMessage.warning("未查询到任何更新");
        }
      }
      await refreshHistoryList();
      refreshFilteredList();
    }, 500)
    
    const clearHasUpdateFlag = async (history) => {
      let newHistory = JSON.parse(JSON.stringify(history));
      newHistory.detail = JSON.stringify(newHistory.detail);
      newHistory.hasUpdate = "0";
      await invoke("save_history", { history: newHistory });
      updateHistoryList();
    }
    
    const shareEvent = (e) => {
      this.share = {
        show: true,
        key: e.site,
        info: e.detail
      }
    }
    
    const downloadEvent = (history) => {
      moviesApi.download(getSiteByKey(history.siteKey, 2), history.ids, history.videoFlag).then(async res => {
        await writeText(res.downloadUrls)
        ElMessage.success(res.info);
      }).catch((err) => {
        ElMessage.error(err.info);
      })
    }
    
    const deleteEvent = async (history) => {
      await invoke("del_history", {id: history.id});
      updateHistoryList();
    }
    
    const detailEvent = (history) => {
      detail.value = {
        show: true,
        siteKey: history.siteKey,
        ids: history.ids,
      }
      if (history.hasUpdate == '1') {
        clearHasUpdateFlag(history)
      }
    }
    
    const backTop = () => {
      if (systemConf.value.historyViewMode === 'picture') {
        document.getElementById('history-body').scrollTop = 0
      } else {
        historyTableRef.value.bodyWrapper.scrollTop = 0
      }
    }

    const fmtMSS = (s) => {
      return (s - (s %= 60)) / 60 + (s > 9 ? ':' : ':0') + s
    }
    
    const progress = (e) => {
      return e.duration > 0 ? ((e.playTime / e.duration) * 100).toFixed(0) : 0
    }
    
    const selectionCellClick = (selection, row) => { // 历史id与顺序刚好相反，大的反而在前面
      if (historyInfo.shiftDown && historyInfo.selectionBegin !== '' && selection.includes(row)) {
        historyInfo.selectionEnd = row.id
        const start = historyInfo.historyFilterList.findIndex(e => e.id === Math.max(historyInfo.selectionBegin, historyInfo.selectionEnd))
        const end = historyInfo.historyFilterList.findIndex(e => e.id === Math.min(historyInfo.selectionBegin, historyInfo.selectionEnd))
        const selections = historyInfo.historyFilterList.slice(start, end + 1)
        nextTick(() => {
          selections.forEach(e => historyTableRef.value.toggleRowSelection(e, true))
        })
        historyInfo.selectionBegin = historyInfo.selectionEnd = ''
        return
      }
      if (selection.includes(row)) {
        historyInfo.selectionBegin = row.id
      } else {
        historyInfo.selectionBegin = ''
      }
    }
    
    const handleSelectionChange = (rows) => {
      historyInfo.multipleSelection = rows
    }

    watch(() => view.value, async () => {
      if (view.value === "History") {
        updateHistoryList();
        if (systemConf.value.historyViewMode === 'table') showShiftPrompt()
      }
    })

    onBeforeMount(async () => {
      await getSystemConf();
    })

    onMounted(() => {
      addEventListener('keydown', event => { if (event.code === 16) historyInfo.shiftDown = true })
      addEventListener('keyup', event => { if (event.code === 16) historyInfo.shiftDown = false })
    })

    return {
      historyInfo,
      history,
      systemConf,
      exportHistory,
      importHistory,
      removeSelectedItems,
      playEvent,
      toggleViewMode,
      backTop,
      fmtMSS,
      progress,
      refreshFilteredList,
      updateAllEvent,
      downloadEvent,
      getSiteByKey,
      shareEvent,
      deleteEvent,
      detailEvent,
      selectionCellClick,
      historyTableRef,
      handleSelectionChange,
    }
  }
});
</script>
