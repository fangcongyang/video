<template>
  <div class="listpage" id="star">
    <div class="listpage-header" id="star-header">
        <el-button @click.stop="exportFavoritesEvent" :icon="Download" title="导出全部，自动添加扩展名">导出</el-button>
        <el-button @click.stop="importFavoritesEvent" :icon="Upload" title="支持同时导入多个文件">导入</el-button>
        <el-button @click.stop="removeSelectedItems" :icon="Delete">
          {{ starInfo.multipleSelection.length === 0 ? "清空" : "删除所选" }}
        </el-button>
        <el-button-group>
          <el-switch v-model="starInfo.onlyShowItemsHasUpdate" active-text="有更新" inactive-text="全部" @change="refreshFilteredList"></el-switch>
          <el-button @click.stop="updateAllEvent" :icon="Refresh">检查更新</el-button>
        </el-button-group>
    </div>
    <div class="toolbar" v-show="starInfo.showToolbar">
      <el-select v-model="starInfo.selectedAreas" multiple placeholder="地区" popper-class="popper" :popper-append-to-body="false"
        @remove-tag="refreshFilteredList" @change="refreshFilteredList">
        <el-option
          v-for="item in starAreas"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <el-select v-model="starInfo.selectedTypes" multiple placeholder="类型" popper-class="popper" :popper-append-to-body="false"
        @remove-tag="refreshFilteredList" @change="refreshFilteredList">
        <el-option
          v-for="item in starTypes"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <el-select v-model="starInfo.sortKeyword" placeholder="排序" popper-class="popper" :popper-append-to-body="false" 
        @change="refreshFilteredList">
        <el-option
          v-for="item in starInfo.sortKeywords"
          :key="item"
          :label="item"
          :value="item">
        </el-option>
      </el-select>
      <span>
       上映区间：
       <el-input-number v-model="starInfo.selectedYears.start" :min=0 :max="new Date().getFullYear()" controls-position="right" 
        step-strictly @change="refreshFilteredList"></el-input-number>
       至
       <el-input-number v-model="starInfo.selectedYears.end" :min=0 :max="new Date().getFullYear()" controls-position="right" 
        step-strictly @change="refreshFilteredList"></el-input-number>
       </span>
    </div>
    <el-divider class="listpage-header-divider" content-position="right">
      <el-button link @click="toggleViewMode" size="small">视图切换</el-button>
      <el-button link @click='() => { starInfo.showToolbar = !starInfo.showToolbar; if (!starInfo.showToolbar) refreshFilteredList() }'
        title="收起工具栏会重置筛选排序" size="small">{{ starInfo.showToolbar ? '隐藏工具栏' : '显示工具栏' }}
        </el-button>
      <el-button link @click="backTop" size="small">回到顶部</el-button>
    </el-divider>
    <div class="listpage-body" id="star-body">
      <div class="show-table" id="star-table"  v-if="systemConf.starViewMode === 'table'">
        <el-table size="small" fit height="100%" row-key="id"
          ref="starTableRef"
          :data="starInfo.starFilterList"
          :cell-class-name="checkUpdate"
          @row-click="detailEvent"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange">
          <el-table-column
            type="selection">
          </el-table-column>
          <el-table-column
            sortable
            :sort-method="(a , b) => sortByLocaleCompare(a.name, b.name)"
            prop="name"
            label="片名">
          </el-table-column>
          <el-table-column
            width="120"
            label="源站">
            <template #default="scope">
              <span>{{ getSiteNameByKey(scope.row.siteKey) }}</span>
            </template>
          </el-table-column>
          <el-table-column
            sortable
            :sort-method="(a , b) => sortByLocaleCompare(a.movieType, b.movieType)"
            prop="movieType"
            label="类型"
            width="100">
          </el-table-column>
          <el-table-column
            sortable
            :sort-by="['year', 'name']"
            prop="year"
            label="上映"
            width="100">
          </el-table-column>
          <el-table-column
            prop="note"
            width="120"
            label="备注">
          </el-table-column>
          <el-table-column
            sortable
            sort-by="doubanRate"
            prop="doubanRate"
            width="120"
            align="center"
            label="豆瓣评分">
          </el-table-column>
          <el-table-column
            label="操作"
            header-align="center"
            align="right"
            width="240">
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link>播放</el-button>
              <el-button @click.stop="shareEvent(scope.row)" link>分享</el-button>
              <el-button @click.stop="downloadEvent(getSiteByKey(scope.row.siteKey, 2), scope.row.siteKey.ids)" link>下载</el-button>
              <el-button @click.stop="deleteEvent(scope.row)" link>删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div class="show-picture" id="star-picture" v-if="systemConf.starViewMode === 'picture'">
        <Waterfall :list="starInfo.starFilterList" :gutter="20" :width="240"
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
                  <div class="rate" v-if="item.doubanRate && item.doubanRate !== '暂无评分'">
                    <span>{{item.doubanRate}}分</span>
                  </div>
                  <div class="update" v-if="item.hasUpdate === '1'">
                    <span>有更新</span>
                  </div>
                  <ImageLazy v-if="item.pic" style="width: 100%" :url="item.pic" @click="detailEvent(item)"/>
                  <div class="operate">
                    <div class="operate-wrap">
                      <span class="o-play" @click="playEvent(item)">播放</span>
                      <span class="o-share" @click="shareEvent(item)">分享</span>
                      <span class="o-star" @click="downloadEvent(getSiteByKey(item.siteKey, 2), item.ids)">下载</span>
                      <span class="o-star" @click="deleteEvent(item)">删除</span>
                    </div>
                  </div>
                </div>
                <div class="name" @click="detailEvent(item)">{{item.name}}</div>
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
</template>
<script>
import { defineComponent, reactive, ref, watch, onMounted, nextTick } from 'vue';
import moviesApi from '@/api/movies';
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useStarStore } from "@/store/star";
import { storeToRefs } from 'pinia';
import { _ } from 'lodash';
import { open, save } from '@tauri-apps/api/dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { Waterfall } from 'vue-waterfall-plugin-next';
import 'vue-waterfall-plugin-next/dist/style.css';
import ImageLazy from '@/components/ImageLazy.vue';
import { invoke } from '@tauri-apps/api';
import Sortable from 'sortablejs';
import { ElMessage } from "element-plus";
import { downloadEvent } from '@/business/movie';
import { Upload, Download, Delete, Refresh } from "@element-plus/icons-vue";

export default defineComponent({
  name: 'Star',
  components: {
      Waterfall,
      ImageLazy
  },
  
  setup() {
    const coreStore = useCoreStore();
    const { updateSystemConf, showShiftPrompt } = coreStore;
    const { view, systemConf, shiftDown } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { getSiteNameByKey } = movieStore;
    const { movieInfo, detail } = storeToRefs(movieStore);

    const starStore = useStarStore();
    const { refreshStarList } = starStore;
    const { starReverseList, starTypes, starAreas } = storeToRefs(starStore);

    const starTableRef = ref();

    const starInfo = reactive({
      showToolbar: false,
      sortKeyword: '',
      selectedAreas: [],
      selectedTypes: [],
      selectedYears: {
        start: 0,
        end: new Date().getFullYear(),
      },
      starFilterList: [],
      onlyShowItemsHasUpdate: false,
      multipleSelection: [],
      movieNoUpdateNum: 0,
      sortKeywords: ['按片名', '按上映年份', '按更新时间'],
      selectionBegin: '',
      selectionEnd: '',
    })
      
    const exportFavoritesEvent = async () => {
      const arr = [...starReverseList.value];
      const str = JSON.stringify(arr, null, 2);
      let filePath = await save({
        filters: [{ name: 'JSON file', extensions: ['json'] }],
      });
      if (filePath !== null) {
        if (!filePath.endsWith('.json')) filePath += '.json'
        await writeTextFile(filePath, str);
        ElMessage.success('导出收藏成功');
      }
    }
    
    const refreshFilteredList = () => {
      let starFilterList = [];
      if (!starInfo.showToolbar) {
        starInfo.sortKeyword = '';
        starInfo.selectedAreas = [];
        starInfo.selectedTypes = [];
        starInfo.selectedYears.start = 0;
        starInfo.selectedYears.end = new Date().getFullYear();
        starFilterList = starReverseList.value;
      } else {
        let filteredData = starReverseList.value;
        filteredData = _.filter(filteredData, (x) => {
          let areaFilter = (starInfo.selectedAreas.length === 0) || starInfo.selectedAreas.includes(x.detail.area);
          let typeFilter = (starInfo.selectedTypes.length === 0) || starInfo.selectedTypes.includes(x.detail.type);
          return areaFilter && typeFilter
        });
        filteredData = _.filter(filteredData, (x) => {
          return x.detail.year >= starInfo.selectedYears.start && x.detail.year <= starInfo.selectedYears.end;
        })
        switch (starInfo.sortKeyword) {
          case '按上映年份':
            filteredData.sort(function (a, b) {
              return b.detail.year - a.detail.year
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
          default:
            break
        }
        starFilterList = filteredData
      }
      if (starInfo.onlyShowItemsHasUpdate) {
        starFilterList = starFilterList.filter(x => x.hasUpdate === '1')
      }
      for (var i = 0; i < starFilterList.length; i++) {
        starFilterList[i].prevId = i == 0 ? 0 : starFilterList[i-1].id;
        starFilterList[i].nextId = i == starFilterList.length - 1 ? starFilterList[i].id + 1 : starFilterList[i+1].id;
      }
      starInfo.starFilterList = starFilterList;
    }
    
    const importFavoritesEvent = async () => {
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
      let newStars = [];
      filePaths.forEach(async file => {
        const str = await readTextFile(file);
        const json = JSON.parse(str)
        json.forEach(star => {
          const starExists = starReverseList.value.some(x => x.siteKey === star.siteKey && x.ids === star.ids)
          if (!starExists) {
            star.id = 0;
            newStars.push(star);
          }
        })
      })
      await invoke("insert_stars", { historys: json });
      refreshStars();
      ElMessage.success('导入收藏成功');
    }

    const refreshStars = _.debounce(async (refreshType = 1) => {
      if (refreshType == 2) { 
        if (starInfo.movieNoUpdateNum == starReverseList.value.length) {
          ElMessage.warning("未查询到任何更新");
        }
      }
      await refreshStarList();
      refreshFilteredList();
    }, 500)
    
    const removeSelectedItems = async () => {
      if (!starInfo.multipleSelection.length) starInfo.multipleSelection = starReverseList.value;
      await invoke("del_stars", { ids: _.map(starInfo.multipleSelection, "id") });
      refreshStars();
    }
    
    const updateAllEvent = () => {
      starInfo.movieNoUpdateNum = 0
      starReverseList.value.forEach(e => {
        updateEvent(star)
      })
    }

    const updateEvent = async (star) => {
      try {
        moviesApi.detail(getSiteByKey(his.siteKey, 2), his.ids).then(async (newDetail) => {
          if (star.lastUpdateTime != newDetail.detail.last) {
            star.hasUpdate = '1';
            star.lastUpdateTime = newDetail.detail.last;
            const msg = `检查到"${star.name}"有更新。`;
            ElMessage.success(msg);
          } else {
            starInfo.movieNoUpdateNum += 1
          }
        })
        refreshStars(2)
      } catch (err) {
        const msg = `更新"${star.name}"失败, 请重试。`;
        ElMessage.warning(msg, err);
      }
    }
    
    const detailEvent = (star) => {
      detail.value = {
        show: true,
        siteKey: star.siteKey,
        ids: star.ids,
      }
      if (star.hasUpdate == '1') {
        clearHasUpdateFlag(star)
      }
    }
    
    const clearHasUpdateFlag = async (star) => {
      star.hasUpdate = "0";
      await invoke("save_star", { star: star });
    }
    
    const playEvent = (star) => {
      movieInfo.value = {
        siteKey: star.siteKey,
        ids: star.ids,
        name: star.name,
        index: 0,
        onlineUrl: "",
      }
      if (star.hasUpdate) {
        clearHasUpdateFlag(star)
      }
      view.value = 'Play'
    }

    const deleteEvent = async (star) => {
      await invoke("del_star", {id: star.id});
      refreshStars();
    }
    
    const toggleViewMode = () => {
      systemConf.value.starViewMode = systemConf.value.starViewMode === 'picture' ? 'table' : 'picture'
      if (systemConf.value.starViewMode === 'table') {
        setTimeout(() => { rowDrop() }, 100)
        showShiftPrompt()
      } else {
        updateSystemConf();
      }
    }
    
    const backTop = () => {
      if (systemConf.value.starViewMode === 'picture') {
        document.getElementById('star-body').scrollTop = 0
      } else {
        starTableRef.value.bodyWrapper.scrollTop = 0
      }
    }
    
    const checkUpdate = ({ row }) => {
      if (row.hasUpdate) {
        return 'highlight'
      }
    }

    const rowDrop = () => {
      if (!document.getElementById('star-table')) return
      const tbody = document.getElementById('star-table').querySelector('.el-table__body-wrapper tbody');
      Sortable.create(tbody, {
        async onEnd ({ newIndex, oldIndex }) {
          let currStar = starInfo.starFilterList[oldIndex];
          // 前第一个数据的位置
          let prevPosition;
          // 后第一个数据的位置
          let nextPosition;
          if (newIndex > oldIndex) {
            newIndex == 0 ? 0 : starInfo.starFilterList[newIndex].position;
            nextPosition = newIndex == starInfo.starFilterList.length - 1 ? starInfo.starFilterList[newIndex].position + 1 
              : starInfo.starFilterList[newIndex + 1].position;
          } else {
            newIndex == 0 ? 0 : starInfo.starFilterList[newIndex - 1].position;
            nextPosition = newIndex == starInfo.starFilterList.length - 1 ? starInfo.starFilterList[newIndex].position + 1 
              : starInfo.starFilterList[newIndex].position;
          }
          currStar.position = (prevPosition + nextPosition) / 2;
          await invoke("save_star", { star: currStar });
          refreshStars();
        }
      })
    }
    
    const sortByLocaleCompare = (a, b) => {
      return a.localeCompare(b, 'zh')
    }
    
    const selectionCellClick = (selection, row) => {
      if (shiftDown.value && starInfo.selectionBegin !== '' && selection.includes(row)) {
        starInfo.selectionEnd = row.id
        const start = starInfo.starFilterList.findIndex(e => e.id === Math.max(starInfo.selectionBegin, starInfo.selectionEnd))
        const end = starInfo.starFilterList.findIndex(e => e.id === Math.min(starInfo.selectionBegin, starInfo.selectionEnd))
        const selections = starInfo.starFilterList.slice(start, end + 1)
        nextTick(() => {
          selections.forEach(e => starTableRef.value.toggleRowSelection(e, true))
        })
        starInfo.selectionBegin = starInfo.selectionEnd = ''
        return
      }
      if (selection.includes(row)) {
        starInfo.selectionBegin = row.id
      } else {
        starInfo.selectionBegin = ''
      }
    }
    
    const handleSelectionChange = (rows) => {
      starInfo.multipleSelection = rows
    }

    const shareEvent = (star) => {
      this.share = {
        show: true,
        siteKey: star.siteKey,
        info: e.detail
      }
    }

    watch(() => view.value, async () => {
      if (view.value === "Star") {
        refreshStars();
        if (systemConf.value.starViewMode === 'table') showShiftPrompt()
      }
    })

    onMounted(() => {
      if (systemConf.value.starViewMode === 'table') setTimeout(() => { rowDrop() }, 100)
    })

    return {
      starInfo,
      exportFavoritesEvent,
      systemConf,
      importFavoritesEvent,
      refreshFilteredList,
      removeSelectedItems,
      updateAllEvent,
      starTypes,
      starAreas,
      getSiteNameByKey,
      detailEvent,
      playEvent,
      deleteEvent,
      toggleViewMode,
      backTop,
      starTableRef,
      checkUpdate,
      downloadEvent,
      sortByLocaleCompare,
      selectionCellClick,
      handleSelectionChange,
      shareEvent,
      Upload,
      Download,
      Delete,
      Refresh,
    }
  }
})
</script>
