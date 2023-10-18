<template>
  <div class="listpage" id="history" @contextmenu="onContextMenu($event)">
    <context-menu
      v-model:show="historyInfo.contextMenushow"
      :options="historyInfo.options"
    >
      <context-menu-group label="地区" :clickClose="false">
        <context-menu-item
          v-for="item in historyAreas"
          @click="areaClick(item)"
        >
          <template #label>
            <span
              class="dot"
              v-if="historyInfo.selectedAreas.includes(item)"
            ></span>
            <span
              class="label"
              :style="
                historyInfo.selectedAreas.includes(item)
                  ? { marginLeft: '6px' }
                  : { marginLeft: '10px' }
              "
              >{{ item }}</span
            >
          </template>
        </context-menu-item>
      </context-menu-group>
      <context-menu-group label="类型" :clickClose="false">
        <context-menu-item
          v-for="item in historyTypes"
          @click="typeClick(item)"
        >
          <template #label>
            <span
              class="dot"
              v-if="historyInfo.selectedTypes.includes(item)"
            ></span>
            <span
              class="label"
              :style="
                historyInfo.selectedTypes.includes(item)
                  ? { marginLeft: '6px' }
                  : { marginLeft: '10px' }
              "
              >{{ item }}</span
            >
          </template>
        </context-menu-item>
      </context-menu-group>
      <context-menu-group label="排序" :clickClose="false">
        <context-menu-item
          v-for="item in historyInfo.sortKeywords"
          @click="
            () => {
              historyInfo.sortKeyword = item;
              refreshFilteredList();
            }
          "
        >
          <template #label>
            <span class="dot" v-if="historyInfo.sortKeyword == item"></span>
            <span
              class="label"
              :style="
                historyInfo.sortKeyword == item
                  ? { marginLeft: '6px' }
                  : { marginLeft: '10px' }
              "
              >{{ item.name }}</span
            >
          </template>
        </context-menu-item>
      </context-menu-group>
      <context-menu-sperator />
      <context-menu-item
        :label="systemConf.starViewMode == 'picture' ? '表格模式' : '图片模式'"
        @click="toggleViewMode()"
      >
        <template #icon>
          <el-icon><Switch /></el-icon>
        </template>
      </context-menu-item>
      <context-menu-item
        :label="historyInfo.onlyShowItemsHasUpdate ? '显示全部' : '显示更新'"
        @click="
          () => {
            historyInfo.onlyShowItemsHasUpdate =
              !historyInfo.onlyShowItemsHasUpdate;
            refreshFilteredList();
          }
        "
      >
        <template #icon>
          <el-icon><TurnOff /></el-icon>
        </template>
      </context-menu-item>
      <context-menu-sperator />
      <context-menu-item label="导出" @click.stop="exportHistory()">
        <template #icon>
          <el-icon>
            <Download />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="导入" @click.stop="importHistory()">
        <template #icon>
          <el-icon>
            <Upload />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="检测更新" @click="updateAllEvent()">
        <template #icon>
          <el-icon>
            <Refresh />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="清空" @click="removeSelectedItems()">
        <template #icon>
          <el-icon><Delete /></el-icon>
        </template>
      </context-menu-item>
    </context-menu>
    <div class="page-body" id="history-body">
      <div
        class="show-table"
        id="history-table"
        v-if="systemConf.historyViewMode === 'table'"
      >
        <el-table
          size="small"
          fit
          height="100%"
          :data="historyInfo.historyFilterList"
          row-key="id"
          ref="historyTableRef"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
          @row-click="detailEvent"
        >
          <el-table-column type="selection"> </el-table-column>
          <el-table-column
            prop="name"
            :show-overflow-tooltip="true"
            label="片名"
          >
          </el-table-column>
          <el-table-column width="120" label="片源">
            <template #default="scope">
              <span>{{ getSiteByKey(scope.row.siteKey) }}</span>
            </template>
          </el-table-column>
          <el-table-column width="180" label="观看至">
            <template #default="scope">
              <span
                v-if="
                  scope.row.detail &&
                  scope.row.detail.fullList[0].list &&
                  scope.row.detail.fullList[0].list.length > 1
                "
              >
                第{{ scope.row.index + 1 }}集(共{{
                  scope.row.detail.fullList[0].list.length
                }}集)
              </span>
            </template>
          </el-table-column>
          <el-table-column width="180" label="时间进度">
            <template #default="scope">
              <span v-if="scope.row.playTime && scope.row.duration">
                {{ fmtMSS(scope.row.playTime.toFixed(0)) }}/{{
                  fmtMSS(scope.row.duration.toFixed(0))
                }}
                ({{ progress(scope.row) }}%)
              </span>
              <span v-if="scope.row.onlinePlay">在线解析</span>
            </template>
          </el-table-column>
          <el-table-column
            label="操作"
            width="240"
            header-align="center"
            align="right"
          >
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button @click.stop="downloadEvent(scope.row)" link
                >下载</el-button
              >
              <el-button @click.stop="deleteEvent(scope.row)" link
                >删除</el-button
              >
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div
        class="show-picture"
        id="history-picture"
        v-if="systemConf.historyViewMode === 'picture'"
      >
        <Waterfall
          :list="historyInfo.historyFilterList"
          :gutter="20"
          :width="240"
          :breakpoints="{
            1200: {
              //当屏幕宽度小于等于1200
              rowPerView: 4,
            },
            800: {
              //当屏幕宽度小于等于800
              rowPerView: 3,
            },
            500: {
              //当屏幕宽度小于等于500
              rowPerView: 2,
            },
          }"
          :animationDuration="0.5"
          backgroundColor="rgba(0, 0, 0, 0)"
        >
          <template #item="{ item }">
            <div class="card">
              <div class="img">
                <div class="update" v-if="item.hasUpdate == '1'">
                  <span>有更新</span>
                </div>
                <ImageLazy
                  v-if="item.detail && item.detail.pic"
                  style="width: 100%"
                  :url="item.detail.pic"
                  @click="detailEvent(item)"
                />
                <div class="operate">
                  <div class="operate-wrap">
                    <span class="o-play" @click="playEvent(item)">播放</span>
                    <span class="o-star" @click="downloadEvent(item)"
                      >下载</span
                    >
                    <span class="o-star" @click="deleteEvent(item)">删除</span>
                  </div>
                </div>
              </div>
              <div class="name" @click="detailEvent(item)">{{ item.name }}</div>
              <div class="info">
                <span v-if="item.playTime && item.duration">
                  {{ fmtMSS(item.playTime.toFixed(0)) }}/{{
                    fmtMSS(item.duration.toFixed(0))
                  }}
                  ({{ progress(item) }}%)
                </span>
                <span v-if="item.onlinePlay">在线解析</span>
                <span
                  v-if="
                    item.detail &&
                    item.detail.fullList[0].list !== undefined &&
                    item.detail.fullList[0].list.length > 1
                  "
                >
                  第{{ item.index + 1 }}集(共{{
                    item.detail.fullList[0].list.length
                  }}集)
                </span>
              </div>
            </div>
          </template>
        </Waterfall>
      </div>
      <el-backtop target="#history-body" :right="100" :bottom="40" />
    </div>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  onBeforeMount,
  watch,
  onMounted,
  ref,
  nextTick,
} from "vue";
import moviesApi from "@/api/movies";
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useHistoryStore } from "@/store/history";
import { useDownloadStore } from "@/store/download";
import { storeToRefs } from "pinia";
import { _ } from "lodash";
import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { Waterfall } from "vue-waterfall-plugin-next";
import "vue-waterfall-plugin-next/dist/style.css";
import ImageLazy from "@/components/ImageLazy.vue";
import { ElMessage } from "element-plus";
import { Upload, Download, Delete, Refresh } from "@element-plus/icons-vue";
import ContextMenu from "@imengyu/vue3-context-menu";
import { useDark } from "@vueuse/core";

export default defineComponent({
  name: "history",
  components: {
    Waterfall,
    ImageLazy,
  },
  setup() {
    const coreStore = useCoreStore();
    const { getSystemConf, updateSystemConf } = coreStore;
    const { view, systemConf, playInfo } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { getSiteByKey } = movieStore;
    const { detail } = storeToRefs(movieStore);

    const historyStore = useHistoryStore();
    const { refreshHistoryList } = historyStore;
    const { historyList, historyAreas, historyTypes } =
      storeToRefs(historyStore);

    const downloadStore = useDownloadStore();
    const { downloadMovie } = downloadStore; 

    const historyTableRef = ref();

    const isDark = useDark();

    const orderFun = (a, b) => {
      const field = historyInfo.sortKeyword.field;
      let aValue = a, bValue = b;
      if (field.includes(".")) {
        field.split(".").forEach(i => {
          aValue = aValue[i];
          bValue = bValue[i];
        })
      } else {
        aValue = a[field];
        bValue = b[field];
      }
      if (historyInfo.sortKeyword.reverse) {
        let temValue = aValue;
        aValue = bValue;
        bValue = temValue;
      }
      if (historyInfo.sortKeyword.type == 'zh') {
        return aValue.localeCompare(bValue, "zh");
      } else {
        return aValue - bValue;
      }
    };

    const sortByProgress = (a, b) => {
      if (progress(a) >= progress(b)) {
        return -1;
      } else {
        return 1;
      }
    };

    const historyInfo = reactive({
      multipleSelection: [],
      historyFilterList: [],
      onlyShowItemsHasUpdate: false,
      movieNoUpdateNum: 0,
      sortKeyword: {},
      selectedAreas: [],
      selectedTypes: [],
      selectedYears: {
        start: 0,
        end: new Date().getFullYear(),
      },
      sortKeywords: [
        {
          name: "按观看时间",
          field: "updateTime",
        },
        {
          name: "按片名",
          field: "name",
          orderFun: orderFun,
          type: "zh",
          reverse: false,
        },
        {
          name: "按上映年份",
          field: "detail.year",
          orderFun: orderFun,
          type: "en",
          reverse: true,
        },
        {
          name: "按更新时间",
          field: "detail.last",
          orderFun: orderFun,
          type: "zh",
          reverse: true,
        },
        {
          name: "按完成度",
          field: "progress",
          orderFun: sortByProgress,
        },
      ],
      shiftDown: false,
      selectionBegin: "",
      selectionEnd: "",
      options: {
        theme: isDark.value ? "mac dark" : "mac",
        iconFontClass: "iconfont",
        zIndex: 3,
        minWidth: 230,
        x: 500,
        y: 200,
      },
    });

    const exportHistory = async () => {
      const arr = [...historyList.value];
      const str = JSON.stringify(arr, null, 2);
      let filePath = await save({
        filters: [{ name: "JSON file", extensions: ["json"] }],
      });
      if (filePath !== null) {
        if (!filePath.endsWith(".json")) filePath += ".json";
        await writeTextFile(filePath, str);
        ElMessage({ showClose: true, message: "已保存成功", type: "success" });
      }
    };

    const importHistory = async () => {
      const selectedFiles = await open({
        multiple: true,
        filters: [{ name: "JSON file", extensions: ["json"] }],
      });
      let filePaths = [];
      if (Array.isArray(selectedFiles)) {
        filePaths = filePaths.concat(selectedFiles);
      } else if (selectedFiles != null) {
        filePaths.push(selectedFiles);
      }
      filePaths.forEach(async (file) => {
        const str = await readTextFile(file);
        const json = JSON.parse(str);
        json.forEach((record) => {
          if (record.detail && record.detail.m3u8List) {
            record.detail.fullList = [].concat({
              flag: "m3u8",
              list: record.detail.m3u8List,
            });
            delete record.detail.m3u8List;
            record.detail = JSON.stringify(record.detail);
          }
        });
        await invoke("insert_historys", { historys: json });
        updateHistoryList();
        ElMessage.success("导入历史记录成功");
      });
    };

    const removeSelectedItems = async () => {
      if (!historyInfo.multipleSelection.length)
        historyInfo.multipleSelection = historyList.value;
      await invoke("del_historys", {
        ids: _.map(historyInfo.multipleSelection, "id"),
      });
      historyInfo.multipleSelection = [];
      updateHistoryList();
    };

    const playEvent = (e) => {
      if (e.hasUpdate) {
        clearHasUpdateFlag(e);
      }
      playInfo.value.playType = "onlineMovie";
      playInfo.value.name = e.name;
      playInfo.value.movie.siteKey = e.siteKey;
      playInfo.value.movie.ids = e.ids;
      playInfo.value.movie.index = e.index;
      playInfo.value.movie.onlinePlay = e.onlinePlay;
      playInfo.value.movie.videoFlag = e.videoFlag;
      playInfo.value.movie.playMode = "local";
      view.value = "Play";
    };

    const toggleViewMode = () => {
      systemConf.value.historyViewMode =
        systemConf.value.historyViewMode === "picture" ? "table" : "picture";
      if (systemConf.value.historyViewMode === "table") {
        showShiftPrompt();
      }
      updateSystemConf();
    };

    const showShiftPrompt = () => {
      if (systemConf.value.shiftTooltipLimitTimes) {
        ElMessage.info("多选时支持shift快捷键");
        systemConf.value.shiftTooltipLimitTimes--;
        updateSystemConf();
      }
    };

    const refreshFilteredList = () => {
      let filteredData = historyList.value;
      filteredData = _.filter(filteredData, (x) => {
        let areaFilter =
          historyInfo.selectedAreas.length === 0 ||
          historyInfo.selectedAreas.includes(x.detail.area);
        let typeFilter =
          historyInfo.selectedTypes.length === 0 ||
          historyInfo.selectedTypes.includes(x.detail.type);
        return areaFilter && typeFilter;
      });
      filteredData = _.filter(filteredData, (x) => {
        return (
          x.detail.year >= historyInfo.selectedYears.start &&
          x.detail.year <= historyInfo.selectedYears.end
        );
      });

      filteredData.sort(historyInfo.sortKeyword.orderFun);

      historyInfo.historyFilterList = filteredData;
      if (historyInfo.onlyShowItemsHasUpdate) {
        historyInfo.historyFilterList = historyInfo.historyFilterList.filter(
          (x) => x.hasUpdate === "1"
        );
      }
    };

    const updateAllEvent = () => {
      historyInfo.movieNoUpdateNum = 0;
      historyList.value.forEach(async (his) => {
        updateEvent(_.cloneDeep(his));
      });
    };

    const updateEvent = async (his) => {
      try {
        moviesApi
          .detail(getSiteByKey(his.siteKey), his.ids)
          .then(async (newDetail) => {
            if (his.detail.last !== newDetail.last) {
              his.hasUpdate = "1";
              his.detail = newDetail;
              const msg = `检查到"${his.name}"有更新。`;
              ElMessage.success(msg);
            } else {
              historyInfo.movieNoUpdateNum++;
            }
            his.detail = JSON.stringify(his.detail);
            await invoke("save_history", { history: his });

            updateHistoryList(2);
          })
          .catch(() => {
            historyInfo.movieNoUpdateNum++;
          });
      } catch (err) {
        const msg = `更新"${his.name}"失败, 请重试。`;
        ElMessage.warning(msg);
      }
    };

    const updateHistoryList = _.debounce(async (refreshType = 1) => {
      if (refreshType == 2) {
        if (historyInfo.movieNoUpdateNum == historyList.value.length) {
          ElMessage.warning("未查询到任何更新");
        }
      }
      await refreshHistoryList();
      refreshFilteredList();
    }, 500);

    const clearHasUpdateFlag = async (history) => {
      let newHistory = JSON.parse(JSON.stringify(history));
      newHistory.detail = JSON.stringify(newHistory.detail);
      newHistory.hasUpdate = "0";
      await invoke("save_history", { history: newHistory });
      updateHistoryList();
    };

    const downloadEvent = (history) => {
      downloadMovie(getSiteByKey(history.siteKey), history.ids);
    };

    const deleteEvent = async (history) => {
      await invoke("del_history", { id: history.id });
      updateHistoryList();
    };

    const detailEvent = (history) => {
      detail.value = {
        show: true,
        siteKey: history.siteKey,
        ids: history.ids,
      };
      if (history.hasUpdate == "1") {
        clearHasUpdateFlag(history);
      }
    };

    const backTop = () => {
      if (systemConf.value.historyViewMode === "picture") {
        document.getElementById("history-body").scrollTop = 0;
      } else {
        historyTableRef.value.bodyWrapper.scrollTop = 0;
      }
    };

    const fmtMSS = (s) => {
      return (s - (s %= 60)) / 60 + (s > 9 ? ":" : ":0") + s;
    };

    const progress = (e) => {
      return e.duration > 0 ? ((e.playTime / e.duration) * 100).toFixed(0) : 0;
    };

    const selectionCellClick = (selection, row) => {
      // 历史id与顺序刚好相反，大的反而在前面
      if (
        historyInfo.shiftDown &&
        historyInfo.selectionBegin !== "" &&
        selection.includes(row)
      ) {
        historyInfo.selectionEnd = row.id;
        const start = historyInfo.historyFilterList.findIndex(
          (e) =>
            e.id ===
            Math.max(historyInfo.selectionBegin, historyInfo.selectionEnd)
        );
        const end = historyInfo.historyFilterList.findIndex(
          (e) =>
            e.id ===
            Math.min(historyInfo.selectionBegin, historyInfo.selectionEnd)
        );
        const selections = historyInfo.historyFilterList.slice(start, end + 1);
        nextTick(() => {
          selections.forEach((e) =>
            historyTableRef.value.toggleRowSelection(e, true)
          );
        });
        historyInfo.selectionBegin = historyInfo.selectionEnd = "";
        return;
      }
      if (selection.includes(row)) {
        historyInfo.selectionBegin = row.id;
      } else {
        historyInfo.selectionBegin = "";
      }
    };

    const handleSelectionChange = (rows) => {
      historyInfo.multipleSelection = rows;
    };

    const onContextMenu = (e) => {
      e.preventDefault();
      historyInfo.options.x = e.x;
      historyInfo.options.y = e.y;
      historyInfo.contextMenushow = true;
      nextTick(() => {
        var elements = document.querySelectorAll(".mx-menu-ghost-host");
        elements.forEach(function (element) {
          // 为每个元素添加事件监听器
          element.addEventListener("contextmenu", function (e) {
            e.preventDefault(); // 阻止默认行为
          });
        });
      });
    };

    watch(
      () => view.value,
      async () => {
        if (view.value === "History") {
          updateHistoryList();
          if (systemConf.value.historyViewMode === "table") showShiftPrompt();
        }
      }
    );

    onBeforeMount(async () => {
      await getSystemConf();
    });

    onMounted(() => {
      historyInfo.sortKeyword = historyInfo.sortKeywords[0];
    });

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
      deleteEvent,
      detailEvent,
      selectionCellClick,
      historyTableRef,
      handleSelectionChange,
      historyAreas,
      historyTypes,
      onContextMenu,
    };
  },
});
</script>
