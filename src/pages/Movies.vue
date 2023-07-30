<template>
  <div class="listpage">
    <div class="listpage-header" id="movies-header">
      <el-select
        v-model="moviesInfo.selectedSiteName"
        placeholder="源站"
        :popper-append-to-body="false"
        @change="siteClick"
      >
        <el-option
          v-for="item in siteList"
          :key="item.key"
          :label="item.name"
          :value="item.key"
        >
        </el-option>
      </el-select>
      <el-select
        v-model="moviesInfo.selectedClassName"
        placeholder="类型"
        :popper-append-to-body="false"
        popper-class="popper"
        @change="classClick"
        v-if="classList && classList.length"
        v-show="!moviesInfo.showFind"
      >
        <el-option
          v-for="item in classList"
          :key="item.id"
          :label="item.name"
          :value="item.name"
        >
        </el-option>
      </el-select>
      <el-select
        v-model="moviesInfo.selectedSearchClassNames"
        multiple
        placeholder="类型"
        :popper-append-to-body="false"
        popper-class="popper"
        v-if="moviesInfo.searchClassList && moviesInfo.searchClassList.length"
        v-show="moviesInfo.showFind && moviesInfo.showToolbar"
        @remove-tag="refreshFilteredList"
        @change="refreshFilteredList"
      >
        <el-option
          v-for="(item, index) in moviesInfo.searchClassList"
          :key="index"
          :label="item"
          :value="item"
        >
        </el-option>
      </el-select>
      <el-autocomplete
        v-model.trim="moviesInfo.searchTxt"
        value-key="keywords"
        :fetch-suggestions="querySearch"
        :popper-append-to-body="false"
        popper-class="popper"
        placeholder="搜索"
        @keyup.enter.native="searchAndRecord"
        @select="searchEvent"
        @clear="searchClearEvent"
        clearable
      >
        <template #prepend>
          <el-select
            v-model="moviesConf.searchGroup"
            :popper-append-to-body="false"
            popper-class="popper"
            default-first-option
            placeholder="请选择"
            @change="searchEvent"
          >
            <el-option
              v-for="item in moviesInfo.searchGroups"
              :key="item"
              :label="item"
              :value="item"
            >
            </el-option>
          </el-select>
        </template>
        <template #append>
          <el-button @click.stop="searchEvent" v-if="!moviesInfo.searchRunning">
            <el-icon style="vertical-align: middle">
              <Search />
            </el-icon>
          </el-button>
          <el-button
            icon="el-icon-loading"
            @click.stop="stopSearchEvent"
            v-if="moviesInfo.searchRunning"
            title="点击可停止搜索"
          >
            <el-icon>
              <Loading />
            </el-icon>
          </el-button>
        </template>
      </el-autocomplete>
    </div>
    <div class="toolbar" v-show="moviesInfo.showToolbar">
      <el-select
        v-model="moviesInfo.selectedAreas"
        multiple
        placeholder="地区"
        popper-class="popper"
        :popper-append-to-body="false"
        @remove-tag="refreshFilteredList"
        @change="refreshFilteredList"
      >
        <el-option
          v-for="item in moviesInfo.areas"
          :key="item"
          :label="item"
          :value="item"
        >
        </el-option>
      </el-select>
      <el-select
        v-model="moviesInfo.sortKeyword"
        placeholder="排序"
        popper-class="popper"
        :popper-append-to-body="false"
        @change="refreshFilteredList"
      >
        <el-option
          v-for="item in moviesInfo.sortKeywords"
          :key="item"
          :label="item"
          :value="item"
        >
        </el-option>
      </el-select>
      <span>
        上映区间：
        <el-input-number
          v-model="moviesInfo.selectedYears.start"
          :min="0"
          :max="new Date().getFullYear()"
          controls-position="right"
          step-strictly
          @change="refreshFilteredList"
        ></el-input-number>
        至
        <el-input-number
          v-model="moviesInfo.selectedYears.end"
          :min="0"
          :max="new Date().getFullYear()"
          controls-position="right"
          step-strictly
          @change="refreshFilteredList"
        ></el-input-number>
      </span>
    </div>
    <el-divider class="listpage-header-divider" content-position="right">
      <el-button link size="small" @click="toggleViewMode">视图切换</el-button>
      <el-button
        link
        size="small"
        @click="showToolbar"
        title="收起工具栏会重置筛选排序"
        >{{ moviesInfo.showToolbar ? "隐藏工具栏" : "显示工具栏" }}</el-button
      >
      <el-button link size="small" @click="backTop">回到顶部</el-button>
    </el-divider>
    <div class="listpage-body" id="film-body" infinite-wrapper>
      <div
        class="show-picture"
        v-if="moviesConf.moviesViewMode === 'picture' && !moviesInfo.showFind"
      >
        <Waterfall
          v-infinite-scroll="infiniteHandler"
          infinite-scroll-distance="600"
          ref="moviesWaterfall"
          :list="moviesInfo.moviesFilteredList"
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
                <ImageLazy
                  style="width: 100%"
                  :url="item.pic"
                  @click="detailEvent(item)"
                />
                <div class="operate">
                  <div class="operate-wrap">
                    <span class="o-play" @click="playEvent(item)">播放</span>
                    <span class="o-star" @click="starEvent(item)">收藏</span>
                    <span class="o-share" @click="shareEvent(item)">分享</span>
                  </div>
                </div>
              </div>
              <div class="name" @click="detailEvent(item)">{{ item.name }}</div>
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
      <div
        class="show-table"
        v-if="moviesConf.moviesViewMode === 'table' && !moviesInfo.showFind"
      >
        <el-table
          size="small"
          :data="moviesInfo.moviesFilteredList"
          ref="moviesTableRef"
          height="100%"
          :empty-text="moviesInfo.statusText"
          @row-click="detailEvent"
          v-infinite-scroll="infiniteHandler"
          infinite-scroll-distance="200"
          style="width: 100%"
        >
          <el-table-column prop="name" label="片名"> </el-table-column>
          <el-table-column
            v-if="moviesInfo.classType.name === '最新'"
            prop="type"
            label="类型"
            width="100"
          >
          </el-table-column>
          <el-table-column prop="year" label="上映" align="center" width="100">
          </el-table-column>
          <el-table-column prop="area" label="地区" width="100">
          </el-table-column>
          <el-table-column prop="lang" label="语言" width="100">
          </el-table-column>
          <el-table-column
            v-if="moviesInfo.showTableLastColumn"
            prop="last"
            label="最近更新"
            :formatter="dateFormat"
            align="left"
            width="120"
          >
          </el-table-column>
          <el-table-column prop="note" label="备注"> </el-table-column>
          <el-table-column
            label="操作"
            header-align="center"
            align="right"
            width="200"
          >
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button @click.stop="starEvent(scope.row)" link
                >收藏</el-button
              >
              <el-button @click.stop="shareEvent(scope.row)" link
                >分享</el-button
              >
              <el-button
                @click.stop="
                  downloadEvent(
                    getSiteByKey(scope.row.siteKey, 2),
                    scope.row.ids
                  )
                "
                link
                >下载</el-button
              >
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div
        class="show-table"
        v-if="moviesConf.moviesViewMode === 'table' && moviesInfo.showFind"
      >
        <el-table
          size="small"
          ref="searchResultTableRef"
          :data="moviesInfo.filteredSearchContents"
          height="100%"
          :empty-text="moviesInfo.statusText"
          @filter-change="filterChange"
          @row-click="detailEvent"
          style="width: 100%"
        >
          <el-table-column
            sortable
            :sort-method="(a, b) => sortByLocaleCompare(a.name, b.name)"
            prop="name"
            label="片名"
          >
          </el-table-column>
          <el-table-column
            v-if="moviesConf.searchGroup !== '站内'"
            sortable
            :sort-method="
              (a, b) => sortByLocaleCompare(a.site.name, b.site.name)
            "
            :filters="getFilters('siteName')"
            :filter-method="
              (value, row, column) => {
                currentColumn = column;
                return value === row.site.name;
              }
            "
            prop="site"
            label="源站"
            width="120"
          >
            <template #default="scope">
              <span>{{ scope.row.site.name }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="type" label="类型" width="100">
          </el-table-column>
          <el-table-column sortable prop="year" label="上映" width="100">
          </el-table-column>
          <el-table-column prop="area" label="地区" width="100">
          </el-table-column>
          <el-table-column
            :filters="getFilters('lang')"
            :filter-method="
              (value, row, column) => {
                currentColumn = column;
                return value === row.lang;
              }
            "
            prop="lang"
            label="语言"
            width="100"
          >
          </el-table-column>
          <el-table-column
            v-if="moviesInfo.showTableLastColumn"
            prop="last"
            label="最近更新"
            :formatter="dateFormat"
            align="left"
            width="120"
          >
          </el-table-column>
          <el-table-column sortable prop="note" label="备注"> </el-table-column>
          <el-table-column
            label="操作"
            header-align="center"
            align="right"
            width="200"
          >
            <template #slot-scope="scope">
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button @click.stop="starEvent(scope.row)" link
                >收藏</el-button
              >
              <el-button @click.stop="shareEvent(scope.row)" link
                >分享</el-button
              >
              <el-button
                @click.stop="
                  downloadEvent(
                    getSiteByKey(scope.row.siteKey, 2),
                    scope.row.ids
                  )
                "
                link
                >下载</el-button
              >
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div
        class="show-picture"
        v-if="moviesConf.moviesViewMode === 'picture' && moviesInfo.showFind"
      >
        <Waterfall
          ref="filmSearchWaterfall"
          :list="moviesInfo.filteredSearchContents"
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
            <div
              class="card"
              v-show="
                !moviesConf.excludeR18Films ||
                !containsClassFilterKeyword(item.type)
              "
            >
              <div class="img">
                <div class="site">
                  <span>{{ item.site.name }}</span>
                </div>
                <ImageLazy
                  style="width: 100%"
                  :url="item.pic"
                  @click="detailEvent(item)"
                />
                <div class="operate">
                  <div class="operate-wrap">
                    <span class="o-play" @click="playEvent(item)">播放</span>
                    <span class="o-star" @click="starEvent(item)">收藏</span>
                    <span class="o-share" @click="shareEvent(item)">分享</span>
                  </div>
                </div>
              </div>
              <div class="name" @click="detailEvent(item)">{{ item.name }}</div>
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
    </div>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  ref,
  onMounted,
  watch,
  computed,
} from "vue";
import { useCoreStore } from "@/store";
import { useMoviesStore } from "@/store/movies";
import { storeToRefs } from "pinia";
import moviesApi from "@/api/movies";
import { ElMessage } from "element-plus";
import { invoke } from "@tauri-apps/api/tauri";
import { debounce } from "lodash";
import { Waterfall } from "vue-waterfall-plugin-next";
import "vue-waterfall-plugin-next/dist/style.css";
import ImageLazy from "@/components/ImageLazy.vue";
import { downloadEvent } from "@/business/movie";
import doubanApi from "@/api/douban";

const FILM_DATA_CACHE = {};

export default defineComponent({
  name: "Movies",
  components: {
    Waterfall,
    ImageLazy,
  },
  setup() {
    const moviesInfo = reactive({
      searchId: 0,
      siteSearchCount: 0,
      selectedSiteName: "",
      selectedClassName: "",
      classType: {
        id: "-1",
        name: "",
      },
      showFind: false,
      areas: [],
      selectedAreas: [],
      showToolbar: false,
      moviesFilteredList: [],
      infiniteId: 1,
      statusText: "",
      showTableLastColumn: false,
      filteredSearchContents: [],
      searchGroups: ["站内", "组内", "全站"],
      sortKeyword: "",
      sortKeywords: ["按片名", "按上映年份", "按更新时间"],
      searchRunning: false,
      selectedSearchClassNames: [],
      searchClassList: [],
      selectedYears: { start: 0, end: new Date().getFullYear() },
      currentSite: {},
    });

    const coreStore = useCoreStore();
    const { view, video } = storeToRefs(coreStore);

    const moviesStore = useMoviesStore();
    const {
      getAllSite,
      getSiteByKey,
      refreshMoviesConf,
      getMoviesConf,
      refreshSearchRecordList,
      getAllSearchRecord,
    } = moviesStore;
    const {
      siteList,
      moviesDetailCache,
      movieInfo,
      detail,
      moviesConf,
      searchRecordList,
    } = storeToRefs(moviesStore);

    const classList = ref([]);
    const moviesPageInfo = ref({
      totalPageCount: 0,
      pageCount: 0,
      recordcount: 0,
      moviesList: [],
    });

    const moviesWaterfall = ref();
    const currentColumn = ref();
    const searchResultTableRef = ref();
    const moviesTableRef = ref();

    const searchSites = computed(() => {
      if (moviesConf.value.searchGroup === "站内")
        return [moviesInfo.currentSite];
      if (moviesConf.value.searchGroup === "组内")
        return siteList.value.filter(
          (site) => site.group === moviesInfo.currentSite.group
        );
      if (moviesConf.value.searchGroup === "全站") return siteList.value;
      return siteList.value.filter((site) => site.isActive);
    });

    const toggleViewMode = () => {
      if (moviesInfo.showFind) {
        moviesConf.value.moviesViewMode =
          moviesConf.value.moviesViewMode === "picture" ? "table" : "picture";
      } else {
        moviesConf.value.moviesViewMode =
          moviesConf.value.moviesViewMode === "picture" ? "table" : "picture";
      }
      refreshMoviesConf();
    };

    const initSite = async () => {
      moviesInfo.selectedSiteName = siteList.value[0].key;
      siteClick(moviesInfo.selectedSiteName);
    };

    const siteClick = (siteKey) => {
      backTop();
      moviesInfo.currentSite = getSiteByKey(siteKey, 2);
      if (moviesConf.value.searchGroup === "站内" && moviesInfo.searchTxt) {
        searchEvent();
        return;
      } else {
        moviesInfo.searchTxt = "";
      }
      moviesInfo.showFind = false;
      classList.value = [];
      if (FILM_DATA_CACHE[moviesInfo.currentSite.key]) {
        classList.value = FILM_DATA_CACHE[moviesInfo.currentSite.key];
        classClick(moviesInfo.selectedClassName);
      } else {
        refreshClass();
      }
    };

    const refreshClass = () => {
      let classPromise = new Promise((resolve, reject) => {
        moviesApi
          .getSiteClass(moviesInfo.currentSite)
          .then((res) => {
            const allClass = [{ name: "最新", id: 0 }];
            res.classList.forEach((element) => {
              if (!containsClassFilterKeyword(element.name)) {
                allClass.push(element);
              }
            });
            resolve(allClass);
          })
          .catch((err) => {
            reject(err);
          });
      });
      classPromise
        .then(
          (res) => {
            classList.value = res;
            FILM_DATA_CACHE[moviesInfo.currentSite.key] = classList.value;
            classClick(moviesInfo.classType.name);
          },
          () => {
            ElMessage.error("获取分类资源失败");
          }
        )
        .catch((e) => {
          ElMessage.error("分类资源处理失败");
        });
    };

    const classClick = (className) => {
      let classType = classList.value.find((x) => x.name === className);
      if (classType) {
        moviesInfo.classType = classType;
      }
      if (!moviesInfo.classType) {
        moviesInfo.classType = classList.value[0];
      }
      if (moviesInfo.selectedClassName.endsWith("剧"))
        moviesInfo.selectedAreas = [];
      const cacheKey =
        moviesInfo.currentSite.key + "@" + moviesInfo.classType.id;
      if (FILM_DATA_CACHE[cacheKey]) {
        moviesPageInfo.value = FILM_DATA_CACHE[cacheKey];
      } else {
        let params = {};
        if (moviesInfo.classType.id != -1) {
          params.t = moviesInfo.classType.id;
        }
        moviesApi.pageMovies(moviesInfo.currentSite, params).then((res) => {
          moviesPageInfo.value = res;
          infiniteHandler();
        });
      }
    };

    const containsClassFilterKeyword = (name) => {
      let ret = false;
      // 主分类过滤, 检测关键词是否包含分类名
      if (moviesConf.value.excludeRootClasses) {
        ret = moviesConf.value.rootClassFilter?.some((v) => v.includes(name));
      }
      // 福利过滤,检测分类名是否包含关键词
      if (moviesConf.value.excludeR18Films && !ret) {
        ret = moviesConf.value.r18ClassFilter?.some((v) => name?.includes(v));
      }
      return ret;
    };

    const backTop = () => {
      if (moviesConf.value.moviesViewMode === "picture") {
        document.getElementById("film-body").scrollTop = 0;
      } else {
        const table = moviesInfo.showFind ? searchResultTableRef.value : moviesTableRef.value;
        table.bodyWrapper.scrollTop = 0
      }
    };

    const showToolbar = () => {
      moviesInfo.showToolbar = !moviesInfo.showToolbar;
      if (!moviesInfo.showToolbar) refreshFilteredList();
    };

    const refreshFilteredList = () => {
      if (!moviesInfo.showToolbar) {
        moviesInfo.sortKeyword = "";
        moviesInfo.selectedAreas = [];
        moviesInfo.selectedSearchClassNames = [];
        moviesInfo.selectedYears.start = 0;
        moviesInfo.selectedYears.end = new Date().getFullYear();
      }
      let filteredData = moviesInfo.showFind
        ? moviesInfo.searchContents
        : moviesPageInfo.value.moviesList;
      if (moviesInfo.showFind)
        filteredData = filteredData.filter(
          (x) =>
            moviesInfo.selectedSearchClassNames.length === 0 ||
            moviesInfo.selectedSearchClassNames.includes(x.type)
        );
      filteredData = filteredData.filter(
        (x) =>
          moviesInfo.selectedAreas.length === 0 ||
          moviesInfo.selectedAreas.includes(x.area)
      );
      filteredData = filteredData.filter(
        (res) =>
          !moviesConf.value.excludeR18Films ||
          !containsClassFilterKeyword(res.type)
      );
      filteredData = filteredData.filter(
        (res) => res.year >= moviesInfo.selectedYears.start
      );
      filteredData = filteredData.filter(
        (res) => res.year <= moviesInfo.selectedYears.end
      );
      if (!moviesInfo.showFind)
        moviesInfo.selectedClassName =
          moviesInfo.classType.name +
          "    " +
          filteredData.length +
          "/" +
          moviesPageInfo.value.recordcount;
      switch (moviesInfo.sortKeyword) {
        case "按上映年份":
          filteredData.sort(function (a, b) {
            return b.year - a.year;
          });
          break;
        case "按片名":
          filteredData.sort(function (a, b) {
            return a.name.localeCompare(b.name, "zh");
          });
          break;
        case "按更新时间":
          filteredData.sort(function (a, b) {
            return new Date(b.last) - new Date(a.last);
          });
          break;
        default:
          filteredData.sort(function (a, b) {
            return new Date(b.last) - new Date(a.last);
          });
          break;
      }

      filteredData = Array.from(new Set(filteredData));
      if (moviesInfo.showFind) {
        moviesInfo.filteredSearchContents = filteredData;
      } else {
        moviesInfo.moviesFilteredList = filteredData;
      }
    };

    const detailEvent = (e) => {
      detail.value = {
        show: true,
        siteKey: moviesInfo.currentSite.key,
        ids: e.id.toString(),
      };
    };

    const playEvent = async (e) => {
      video.value.playType = "movies";
      movieInfo.value = {
        siteKey: moviesInfo.currentSite.key,
        ids: e.id,
        name: e.name,
        index: 0,
        videoFlag: "",
      };
      view.value = "Play";
    };

    const starEvent = async (e) => {
      let siteKey = moviesInfo.currentSite.key,
        ids = e.id.toString();
      const starStr = await invoke("get_star_by_uq", {
        siteKey: siteKey,
        ids: ids,
      });
      if (starStr) {
        ElMessage({ showClose: true, message: "当前影片已收藏", type: "info" });
      } else {
        const cacheKey = siteKey + "@" + ids;
        if (!moviesDetailCache.value[cacheKey]) {
          moviesDetailCache.value[cacheKey] = await moviesApi.detail(
            moviesInfo.currentSite,
            e.id
          );
        }
        const star = {
          id: 0,
          name: e.name,
          ids: e.id.toString(),
          siteKey: siteKey,
          movieType: e.type,
          year: e.year + "年",
          note: e.note,
          doubanRate: await doubanApi.doubanRate(e.name, e.year),
          hasUpdate: "0",
          lastUpdateTime: e.last,
          position: 0.0,
          pic: e.pic,
          area: e.area,
        };
        await invoke("save_star", { star: star });
        ElMessage({ showClose: true, message: "收藏成功", type: "success" });
      }
    };

    const shareEvent = (e) => {
      this.share = {
        show: true,
        key: moviesInfo.currentSite.key,
        info: e,
      };
    };

    const infiniteHandler = debounce(() => {
      const key = moviesInfo.currentSite.key;
      let typeTid = moviesInfo.classType.id;
      let page = moviesPageInfo.value.pageCount;
      let totalPageCount = moviesPageInfo.value.totalPageCount;
      if (moviesInfo.currentSite.reverseOrder) {
        page = totalPageCount - page + 1;
      }
      moviesInfo.statusText = " ";
      if (key === undefined || page < 1 || page > totalPageCount) {
        moviesInfo.statusText = "暂无数据";
        return false;
      }
      setTimeout(() => {
        let params = {
          pg: page,
          t: typeTid == -1 ? undefined : typeTid,
        };
        moviesApi.listMovies(moviesInfo.currentSite, params).then(
          (res) => {
            if (res) {
              moviesPageInfo.value.pageCount -= 1;
              const type = Object.prototype.toString.call(res);
              if (type === "[object Array]") {
                // 过滤掉无链接的项
                res = res.filter(
                  (e) =>
                    e.dl.dd &&
                    (e.dl.dd._t ||
                      (Object.prototype.toString.call(e.dl.dd) ===
                        "[object Array]" &&
                        e.dl.dd.some((i) => i._t)))
                );
                if (!moviesInfo.currentSite.reverseOrder) {
                  // zy.list 返回的是按时间从旧到新排列, 我门需要翻转为从新到旧
                  moviesPageInfo.value.moviesList.push(...res.reverse());
                } else {
                  // 如果是需要解析的视频网站，zy.list已经是按从新到旧排列
                  moviesPageInfo.value.moviesList.push(...res);
                }
              } else if (type === "[object Object]") {
                if (
                  res.dl.dd &&
                  (res.dl.dd._t ||
                    (Object.prototype.toString.call(res.dl.dd) ===
                      "[object Array]" &&
                      res.dl.dd.some((e) => e._t)))
                ) {
                  moviesPageInfo.value.moviesList.push(res);
                }
              }
              // 更新缓存数据
              const cacheKey = moviesInfo.currentSite.key + "@" + typeTid;
              FILM_DATA_CACHE[cacheKey] = moviesPageInfo.value;
              refreshFilteredList();
            }
          },
          () => {
            ElMessage.error("获取资源列表失败");
          }
        );
      }, 300);
    }, 1000);

    const dateFormat = (row, column) => {
      const date = row[column.property];
      if (date === undefined) {
        return "";
      }
      return date.split(/\s/)[0];
    };

    const getFilters = (column) => {
      if (column === "siteName")
        return [
          ...new Set(
            moviesInfo.filteredSearchContents.map((row) => row.site.name)
          ),
        ].map((e) => {
          return { text: e, value: e };
        });
      return [
        ...new Set(moviesInfo.filteredSearchContents.map((row) => row[column])),
      ].map((e) => {
        return { text: e, value: e };
      });
    };

    const sortByLocaleCompare = (a, b) => {
      return a.localeCompare(b, "zh");
    };

    const filterChange = (filters) => {
      // 一次只能一列
      if (Object.values(filters)[0].length) {
        const otherColumns = searchResultTableRef.value.columns.filter(col => col.id !== currentColumn.value.id)
        otherColumns.forEach(col => { col.filterable = false })
      } else {
        const filterLabels = ['源站', '语言']
        const columns = searchResultTableRef.value.columns.filter(col => filterLabels.includes(col.label))
        columns.forEach(col => { col.filterable = true })
      }
    };

    const querySearch = (queryString, cb) => {
      const searchList = searchRecordList.value.slice(0, -1);
      const results = queryString
        ? searchList.filter(createFilter(queryString))
        : searchRecordList.value;
      // 调用 callback 返回建议列表的数据
      cb(results);
    };

    const searchEvent = () => {
      const wd = moviesInfo.searchTxt;
      if (!wd) return;
      moviesInfo.searchId += 1;
      moviesInfo.searchContents = [];
      moviesInfo.showFind = true;
      moviesInfo.statusText = " ";
      moviesInfo.searchRunning = true;
      moviesInfo.siteSearchCount = 0;
      searchSites.value.forEach((site) => {
        const id = moviesInfo.searchId;
        moviesApi
          .search(site, wd)
          .then((res) => {
            if (id !== moviesInfo.searchId || !moviesInfo.searchRunning) return;
            const type = Object.prototype.toString.call(res);
            if (type === "[object Array]") {
              let count = 0;
              res.forEach((element) => {
                moviesApi
                  .detail(site, element.id)
                  .then((detailRes) => {
                    if (id !== moviesInfo.searchId || !moviesInfo.searchRunning)
                      return;
                    if (detailRes) {
                      detailRes.site = site;
                      if (isValidSearchResult(detailRes)) {
                        moviesInfo.searchContents.push(detailRes);
                        moviesInfo.searchContents.sort(function (a, b) {
                          return a.site.id - b.site.id;
                        });
                        refreshFilteredList();
                      }
                    }
                  })
                  .finally(() => {
                    count++;
                    if (count === res.length) {
                      moviesInfo.siteSearchCount++;
                      moviesInfo.statusText = "暂无数据";
                    }
                  });
              });
            } else if (type === "[object Object]") {
              moviesApi
                .detail(site, res.id)
                .then((detailRes) => {
                  if (id !== moviesInfo.searchId || !moviesInfo.searchRunning)
                    return;
                  detailRes.site = site;
                  if (isValidSearchResult(detailRes)) {
                    moviesInfo.searchContents.push(detailRes);
                    moviesInfo.searchContents.sort(function (a, b) {
                      return a.site.id - b.site.id;
                    });
                  }
                })
                .finally(() => {
                  moviesInfo.siteSearchCount++;
                  moviesInfo.statusText = "暂无数据";
                });
            } else if (res === undefined) {
              moviesInfo.siteSearchCount++;
              moviesInfo.statusText = "暂无数据";
              if (moviesConf.value.searchGroup === "站内")
                ElMessage.info("没有查询到数据！");
            }
          })
          .catch(() => {
            moviesInfo.siteSearchCount++;
            if (moviesConf.value.searchGroup === "站内")
              ElMessage({
                showClose: true,
                message: "本次查询状态异常，未获取到数据！",
                type: "error",
              });
          });
      });
    };

    const isValidSearchResult = (detailRes) => {
      return (
        detailRes.dl.dd &&
        (detailRes.dl.dd._t ||
          (Object.prototype.toString.call(detailRes.dl.dd) ===
            "[object Array]" &&
            detailRes.dl.dd.some((i) => i._t)))
      );
    };

    const createFilter = (queryString) => {
      return (item) => {
        return (
          item.keywords.toLowerCase().indexOf(queryString.toLowerCase()) === 0
        );
      };
    };

    const searchAndRecord = () => {
      addSearchRecord();
      searchEvent();
    };

    const addSearchRecord = async () => {
      const wd = moviesInfo.searchTxt;
      if (wd) {
        await invoke("save_search_record", {
          searchRecord: { id: 0, keywords: wd },
        });
        refreshSearchRecordList();
      }
    };

    const searchClearEvent = () => {
      if (!moviesInfo.searchTxt) {
        moviesInfo.searchContents = [];
        moviesInfo.showFind = false;
        initSite();
      }
    };

    const stopSearchEvent = () => {
      moviesInfo.searchRunning = false;
    };

    watch(
      () => {
        return moviesInfo.searchTxt;
      },
      async () => {
        if (moviesInfo.searchTxt === "清除历史记录...") {
          await invoke("del_all_search_record", {});
          moviesInfo.searchTxt = "";
          searchChangeEvent();
        }
      }
    );

    watch(
      () => {
        return moviesInfo.siteSearchCount;
      },
      () => {
        if (moviesInfo.siteSearchCount === searchSites.value.length) {
          moviesInfo.searchRunning = false;
        }
      }
    );
    
    watch(() => {
      return moviesInfo.searchContents;
    },
    () => {
      moviesFilteredList = moviesInfo.moviesFilteredList.filter(res => !moviesConf.value.excludeR18Films || !containsClassFilterKeyword(res.type))
      moviesInfo.areas = [...new Set(moviesFilteredList.map(ele => ele.area))].filter(x => x)
      moviesInfo.searchClassList = [...new Set(moviesFilteredList.map(ele => ele.type))].filter(x => x)
      refreshFilteredList()
    },  {
      deep: true
    })

    onMounted(() => {
      getMoviesConf();
      getAllSearchRecord();
      getAllSite().then(() => {
        initSite();
      });
    });

    return {
      moviesInfo,
      siteList,
      siteClick,
      classList,
      classClick,
      toggleViewMode,
      backTop,
      showToolbar,
      moviesWaterfall,
      detailEvent,
      playEvent,
      infiniteHandler,
      moviesPageInfo,
      dateFormat,
      getFilters,
      sortByLocaleCompare,
      filterChange,
      currentColumn,
      refreshFilteredList,
      starEvent,
      shareEvent,
      moviesConf,
      querySearch,
      searchEvent,
      searchAndRecord,
      searchClearEvent,
      stopSearchEvent,
      downloadEvent,
      searchResultTableRef,
    };
  },
});
</script>
