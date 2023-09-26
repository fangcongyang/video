<template>
  <div class="listpage" id="star" @contextmenu="onContextMenu($event)">
    <context-menu
      v-model:show="starInfo.contextMenushow"
      :options="starInfo.options"
    >
      <context-menu-group label="地区" :clickClose="false">
        <context-menu-item
          v-for="item in starAreas"
          @click="areaClick(item)"
        >
          <template #label>
            <span class="dot" v-if="starInfo.selectedAreas.includes(item)"></span>
            <span
              class="label"
              :style="
                starInfo.selectedAreas.includes(item)
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
          v-for="item in starTypes"
          @click="typeClick(item)"
        >
          <template #label>
            <span class="dot" v-if="starInfo.selectedTypes.includes(item)"></span>
            <span
              class="label"
              :style="
                starInfo.selectedTypes.includes(item)
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
          v-for="item in starInfo.sortKeywords"
          @click="
            () => {
              starInfo.sortKeyword = item;
              refreshFilteredList();
            }
          "
        >
          <template #label>
            <span class="dot" v-if="starInfo.sortKeyword == item"></span>
            <span
              class="label"
              :style="
                starInfo.sortKeyword == item
                  ? { marginLeft: '6px' }
                  : { marginLeft: '10px' }
              "
              >{{ item.name }}</span
            >
          </template>
        </context-menu-item>
      </context-menu-group>
      <context-menu-sperator />
      <context-menu-item :label="systemConf.starViewMode == 'picture' ? '表格模式' : '图片模式'" @click="toggleViewMode()">
        <template #icon>
          <el-icon><Switch /></el-icon>
        </template>
      </context-menu-item>
      <context-menu-item :label="starInfo.onlyShowItemsHasUpdate ? '显示全部' : '显示更新'" @click="() => {
          starInfo.onlyShowItemsHasUpdate = !starInfo.onlyShowItemsHasUpdate;
          refreshFilteredList()
        }">
        <template #icon>
          <el-icon><TurnOff /></el-icon>
        </template>
      </context-menu-item>
      <context-menu-sperator />
      <context-menu-item label="导出" @click="exportFavoritesEvent()">
        <template #icon>
          <el-icon>
            <Download />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="导入" @click="importFavoritesEvent()">
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
    <div class="page-body" id="star-body">
      <div
        class="show-table"
        id="star-table"
        v-if="systemConf.starViewMode === 'table'"
      >
        <el-table
          size="small"
          fit
          height="100%"
          row-key="id"
          ref="starTableRef"
          :data="starInfo.starFilterList"
          :cell-class-name="checkUpdate"
          @row-click="detailEvent"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
        >
          <el-table-column type="selection"> </el-table-column>
          <el-table-column
            sortable
            :sort-method="(a, b) => sortByLocaleCompare(a.star_name, b.star_name)"
            prop="star_name"
            label="片名"
          >
          </el-table-column>
          <el-table-column width="120" label="源站">
            <template #default="scope">
              <span>{{ getSiteNameByKey(scope.row.siteKey) }}</span>
            </template>
          </el-table-column>
          <el-table-column
            sortable
            :sort-method="
              (a, b) => sortByLocaleCompare(a.movieType, b.movieType)
            "
            prop="movieType"
            label="类型"
            width="100"
          >
          </el-table-column>
          <el-table-column
            sortable
            :sort-by="['year', 'name']"
            prop="year"
            label="上映"
            width="100"
          >
          </el-table-column>
          <el-table-column prop="note" width="120" label="备注">
          </el-table-column>
          <el-table-column
            sortable
            sort-by="doubanRate"
            prop="doubanRate"
            width="120"
            align="center"
            label="豆瓣评分"
          >
          </el-table-column>
          <el-table-column
            label="操作"
            header-align="center"
            align="right"
            width="240"
          >
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button @click.stop="shareEvent(scope.row)" link
                >分享</el-button
              >
              <el-button
                @click.stop="
                  downloadEvent(
                    getSiteByKey(scope.row.siteKey, 2),
                    scope.row.siteKey.ids
                  )
                "
                link
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
        id="star-picture"
        v-if="systemConf.starViewMode === 'picture'"
      >
        <Waterfall
          :list="starInfo.starFilterList"
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
          animationEffect="fadeIn"
          backgroundColor="rgba(0, 0, 0, 0)"
        >
          <template #item="{ item }">
            <div class="card">
              <div class="img">
                <div
                  class="rate"
                  v-if="item.douban_rate && item.douban_rate !== '暂无评分'"
                >
                  <span>{{ item.douban_rate }}分</span>
                </div>
                <div class="update" v-if="item.has_update === '1'">
                  <span>有更新</span>
                </div>
                <ImageLazy
                  v-if="item.pic"
                  style="width: 100%"
                  :url="item.pic"
                  @click="detailEvent(item)"
                />
                <div class="operate">
                  <div class="operate-wrap">
                    <span class="o-play" @click="playEvent(item)">播放</span>
                    <span class="o-share" @click="shareEvent(item)">分享</span>
                    <span
                      class="o-star"
                      @click="
                        downloadEvent(getSiteByKey(item.site_key), item.ids)
                      "
                      >下载</span
                    >
                    <span class="o-star" @click="deleteEvent(item)">删除</span>
                  </div>
                </div>
              </div>
              <div class="name" @click="detailEvent(item)">{{ item.star_name }}</div>
              <div class="info">
                <span>{{ item.area }}</span>
                <span>{{ item.year }}</span>
                <span>{{ item.note }}</span>
                <span>{{ item.type }}</span>
              </div>
            </div>
          </template>
        </Waterfall>
      </div>
      <el-backtop target="#star-body" :right="100" :bottom="40" />
    </div>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  ref,
  watch,
  onMounted,
  nextTick,
} from "vue";
import moviesApi from "@/api/movies";
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useStarStore } from "@/store/star";
import { storeToRefs } from "pinia";
import { _ } from "lodash";
import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { Waterfall } from "vue-waterfall-plugin-next";
import "vue-waterfall-plugin-next/dist/style.css";
import ImageLazy from "@/components/ImageLazy.vue";
import { invoke } from "@tauri-apps/api";
import Sortable from "sortablejs";
import { ElMessage } from "element-plus";
import { Upload, Download, Delete, Refresh } from "@element-plus/icons-vue";
import ContextMenu from "@imengyu/vue3-context-menu";
import { useDark } from "@vueuse/core";

export default defineComponent({
  name: "Star",
  components: {
    Waterfall,
    ImageLazy,
  },

  setup() {
    const coreStore = useCoreStore();
    const { updateSystemConf, showShiftPrompt } = coreStore;
    const { view, systemConf, shiftDown, playInfo } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const { getSiteNameByKey, getSiteByKey } = movieStore;
    const { detail } = storeToRefs(movieStore);

    const starStore = useStarStore();
    const { refreshStarList } = starStore;
    const { starReverseList, starTypes, starAreas } = storeToRefs(starStore);

    const starTableRef = ref();

    const isDark = useDark();

    const orderFun = (a, b) => {
      const field = starInfo.sortKeyword.field;
      return a[field].localeCompare(b[field], 'zh')
    }

    const orderFun1 = (a, b) => {
      const field = starInfo.sortKeyword.field;
      return b[field].localeCompare(a[field], 'zh')
    }

    const starInfo = reactive({
      sortKeyword: {},
      selectedAreas: [],
      selectedTypes: [],
      starFilterList: [],
      onlyShowItemsHasUpdate: false,
      multipleSelection: [],
      movieNoUpdateNum: 0,
      sortKeywords: [{
        name: "按添加时间",
        field: "id",
      }, {
        name: "按片名",
        field: "name",
        orderFun: orderFun
      }, {
        name: "按上映年份",
        field: "year",
        orderFun: orderFun1
      }, {
        name: "按更新时间",
        field: "lastUpdateTime",
        orderFun: orderFun1
      }],
      selectionBegin: "",
      selectionEnd: "",
      contextMenushow: false,
      options: {
        theme: isDark.value ? 'mac dark' : 'mac',
        iconFontClass: "iconfont",
        zIndex: 3,
        minWidth: 230,
        x: 500,
        y: 200,
      },
    });

    const exportFavoritesEvent = async () => {
      const arr = [...starReverseList.value];
      const str = JSON.stringify(arr, null, 2);
      let filePath = await save({
        filters: [{ name: "JSON file", extensions: ["json"] }],
      });
      if (filePath !== null) {
        if (!filePath.endsWith(".json")) filePath += ".json";
        await writeTextFile(filePath, str);
        ElMessage.success("导出收藏成功");
      }
    };

    const areaClick = (item) => {
      if (_.includes(starInfo.selectedAreas, item)) {
        starInfo.selectedAreas = _.dropWhile(starInfo.selectedAreas, (o) => {
          return o == item
        })
      } else {
        starInfo.selectedAreas.push(item);
      }
      refreshFilteredList();
    }

    const typeClick = (item) => {
      if (_.includes(starInfo.selectedTypes, item)) {
        starInfo.selectedTypes = _.dropWhile(starInfo.selectedTypes, (o) => {
          return o == item
        })
      } else {
        starInfo.selectedTypes.push(item);
      }
      refreshFilteredList();
    }

    const refreshFilteredList = () => {
      let starFilterList = [];
      let filteredData = starReverseList.value;
      filteredData = _.filter(filteredData, (x) => {
        let areaFilter =
          starInfo.selectedAreas.length === 0 ||
          starInfo.selectedAreas.includes(x.area);
        let typeFilter =
          starInfo.selectedTypes.length === 0 ||
          starInfo.selectedTypes.includes(x.movieType);
        return areaFilter && typeFilter;
      });
      
      filteredData.sort(starInfo.sortKeyword.orderFun)

      starFilterList = filteredData;
      if (starInfo.onlyShowItemsHasUpdate) {
        starFilterList = starFilterList.filter((x) => x.hasUpdate === "1");
      }
      for (var i = 0; i < starFilterList.length; i++) {
        starFilterList[i].prevId = i == 0 ? 0 : starFilterList[i - 1].id;
        starFilterList[i].nextId =
          i == starFilterList.length - 1
            ? starFilterList[i].id + 1
            : starFilterList[i + 1].id;
      }
      starInfo.starFilterList = starFilterList;
    };

    const importFavoritesEvent = async () => {
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
      let newStars = [];
      filePaths.forEach(async (file) => {
        const str = await readTextFile(file);
        const json = JSON.parse(str);
        json.forEach((star) => {
          const starExists = starReverseList.value.some(
            (x) => x.siteKey === star.siteKey && x.ids === star.ids
          );
          if (!starExists) {
            star.id = 0;
            newStars.push(star);
          }
        });
      });
      await invoke("insert_stars", { historys: json });
      refreshStars();
      ElMessage.success("导入收藏成功");
    };

    const refreshStars = _.debounce(async (refreshType = 1) => {
      if (refreshType == 2) {
        if (starInfo.movieNoUpdateNum == starReverseList.value.length) {
          ElMessage.warning("未查询到任何更新");
        }
      }
      await refreshStarList();
      refreshFilteredList();
    }, 500);

    const removeSelectedItems = async () => {
      if (!starInfo.multipleSelection.length)
        starInfo.multipleSelection = starReverseList.value;
      await invoke("del_stars", {
        ids: _.map(starInfo.multipleSelection, "id"),
      });
      refreshStars();
    };

    const updateAllEvent = () => {
      starInfo.movieNoUpdateNum = 0;
      starReverseList.value.forEach((e) => {
        updateEvent(star);
      });
    };

    const updateEvent = async (star) => {
      try {
        moviesApi
          .detail(getSiteByKey(his.siteKey, 2), his.ids)
          .then(async (newDetail) => {
            if (star.lastUpdateTime != newDetail.detail.last) {
              star.hasUpdate = "1";
              star.lastUpdateTime = newDetail.detail.last;
              const msg = `检查到"${star.name}"有更新。`;
              ElMessage.success(msg);
            } else {
              starInfo.movieNoUpdateNum += 1;
            }
          });
        refreshStars(2);
      } catch (err) {
        const msg = `更新"${star.name}"失败, 请重试。`;
        ElMessage.warning(msg, err);
      }
    };

    const detailEvent = (star) => {
      detail.value = {
        show: true,
        siteKey: star.siteKey,
        ids: star.ids,
      };
      if (star.hasUpdate == "1") {
        clearHasUpdateFlag(star);
      }
    };

    const clearHasUpdateFlag = async (star) => {
      star.hasUpdate = "0";
      await invoke("save_star", { star: star });
    };

    const playEvent = (star) => {
      playInfo.value.playType = "movie";
      playInfo.value.name = star.name;
      playInfo.value.movie.siteKey = star.siteKey;
      playInfo.value.movie.ids = star.ids;
      playInfo.value.movie.index = 0;
      playInfo.value.movie.onlineUrl = "";
      playInfo.value.movie.videoFlag = "";
      if (star.hasUpdate) {
        clearHasUpdateFlag(star);
      }
      view.value = "Play";
    };

    const deleteEvent = async (star) => {
      await invoke("del_star", { id: star.id });
      refreshStars();
    };

    const downloadEvent = (star) => {}

    const toggleViewMode = () => {
      systemConf.value.starViewMode =
        systemConf.value.starViewMode === "picture" ? "table" : "picture";
      if (systemConf.value.starViewMode === "table") {
        setTimeout(() => {
          rowDrop();
        }, 100);
        showShiftPrompt();
      } else {
        updateSystemConf();
      }
    };

    const checkUpdate = ({ row }) => {
      if (row.hasUpdate) {
        return "highlight";
      }
    };

    const rowDrop = () => {
      if (!document.getElementById("star-table")) return;
      const tbody = document
        .getElementById("star-table")
        .querySelector(".el-table__body-wrapper tbody");
      Sortable.create(tbody, {
        async onEnd({ newIndex, oldIndex }) {
          let currStar = starInfo.starFilterList[oldIndex];
          // 前第一个数据的位置
          let prevPosition;
          // 后第一个数据的位置
          let nextPosition;
          if (newIndex > oldIndex) {
            newIndex == 0 ? 0 : starInfo.starFilterList[newIndex].position;
            nextPosition =
              newIndex == starInfo.starFilterList.length - 1
                ? starInfo.starFilterList[newIndex].position + 1
                : starInfo.starFilterList[newIndex + 1].position;
          } else {
            newIndex == 0 ? 0 : starInfo.starFilterList[newIndex - 1].position;
            nextPosition =
              newIndex == starInfo.starFilterList.length - 1
                ? starInfo.starFilterList[newIndex].position + 1
                : starInfo.starFilterList[newIndex].position;
          }
          currStar.position = (prevPosition + nextPosition) / 2;
          await invoke("save_star", { star: currStar });
          refreshStars();
        },
      });
    };

    const sortByLocaleCompare = (a, b) => {
      return a.localeCompare(b, "zh");
    };

    const selectionCellClick = (selection, row) => {
      if (
        shiftDown.value &&
        starInfo.selectionBegin !== "" &&
        selection.includes(row)
      ) {
        starInfo.selectionEnd = row.id;
        const start = starInfo.starFilterList.findIndex(
          (e) =>
            e.id === Math.max(starInfo.selectionBegin, starInfo.selectionEnd)
        );
        const end = starInfo.starFilterList.findIndex(
          (e) =>
            e.id === Math.min(starInfo.selectionBegin, starInfo.selectionEnd)
        );
        const selections = starInfo.starFilterList.slice(start, end + 1);
        nextTick(() => {
          selections.forEach((e) =>
            starTableRef.value.toggleRowSelection(e, true)
          );
        });
        starInfo.selectionBegin = starInfo.selectionEnd = "";
        return;
      }
      if (selection.includes(row)) {
        starInfo.selectionBegin = row.id;
      } else {
        starInfo.selectionBegin = "";
      }
    };

    const handleSelectionChange = (rows) => {
      starInfo.multipleSelection = rows;
    };

    const shareEvent = (star) => {
      this.share = {
        show: true,
        siteKey: star.siteKey,
        info: e.detail,
      };
    };

    const onContextMenu = (e) => {
      e.preventDefault();
      starInfo.options.x = e.x;
      starInfo.options.y = e.y;
      starInfo.contextMenushow = true;
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
        if (view.value === "Star") {
          refreshStars();
          if (systemConf.value.starViewMode === "table") showShiftPrompt();
        }
      }
    );

    onMounted(() => {
      starInfo.sortKeyword = starInfo.sortKeywords[0];
      if (systemConf.value.starViewMode === "table")
        setTimeout(() => {
          rowDrop();
        }, 100);
    });

    return {
      starInfo,
      systemConf,
      exportFavoritesEvent,
      importFavoritesEvent,
      updateAllEvent,
      refreshFilteredList,
      starTypes,
      starAreas,
      getSiteNameByKey,
      detailEvent,
      playEvent,
      deleteEvent,
      toggleViewMode,
      starTableRef,
      checkUpdate,
      downloadEvent,
      sortByLocaleCompare,
      selectionCellClick,
      handleSelectionChange,
      shareEvent,
      onContextMenu,
      areaClick,
      typeClick,
      getSiteByKey,
    };
  },
});
</script>
