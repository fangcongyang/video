<template>
  <div class="listpage" @contextmenu="onContextMenu($event)">
    <context-menu
      v-model:show="movieInfo.contextMenushow"
      :options="movieInfo.options"
    >
      <context-menu-item :clickClose="false" @click.stop="() => {}" v-if="!movieInfo.showFind">
        <template #label>
          <el-input-number
            v-model="moviesPageInfo.pageCount"
            :min="1"
            :max="moviesPageInfo.totalPageCount"
            size="small"
            style="width: 180Px;"
            controls-position="right"
            @blur="startPageChange"
          />
        </template>
      </context-menu-item>
      <context-menu-group label="地区" :clickClose="false">
        <context-menu-item
          v-for="item in movieInfo.areas"
          @click="areaClick(item)"
        >
          <template #label>
            <span class="dot" v-if="movieInfo.selectedAreas.includes(item)"></span>
            <span
              class="label"
              :style="
                movieInfo.selectedAreas.includes(item)
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
          v-for="item in movieInfo.sortKeywords"
          @click="
            () => {
              movieInfo.sortKeyword = item;
              refreshFilteredList();
            }
          "
        >
          <template #label>
            <span class="dot" v-if="movieInfo.sortKeyword == item"></span>
            <span
              class="label"
              :style="
                movieInfo.sortKeyword == item
                  ? { marginLeft: '6px' }
                  : { marginLeft: '10px' }
              "
              >{{ item.name }}</span
            >
          </template>
        </context-menu-item>
      </context-menu-group>
      <context-menu-item :label="moviesConf.moviesViewMode == 'picture' ? '表格模式' : '图片模式'" @click="toggleViewMode()">
        <template #icon>
          <el-icon><Switch /></el-icon>
        </template>
      </context-menu-item>
    </context-menu>
    <div class="listpage-header" id="movies-header">
      <el-select
        v-model="movieInfo.selectedSiteName"
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
        v-model="movieInfo.selectedClassName"
        placeholder="类型"
        :popper-append-to-body="false"
        popper-class="popper"
        style="width: 210Px;"
        @change="classClick"
        v-if="classList && classList.length"
        v-show="!movieInfo.showFind"
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
        v-model="movieInfo.selectedSearchClassNames"
        multiple
        placeholder="类型"
        :popper-append-to-body="false"
        popper-class="popper"
        v-if="movieInfo.searchClassList && movieInfo.searchClassList.length"
        v-show="movieInfo.showFind"
        @remove-tag="refreshFilteredList"
        @change="refreshFilteredList"
      >
        <el-option
          v-for="(item, index) in movieInfo.searchClassList"
          :key="index"
          :label="item"
          :value="item"
        >
        </el-option>
      </el-select>
      <el-autocomplete
        v-model.trim="movieInfo.searchTxt"
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
              v-for="item in movieInfo.searchGroups"
              :key="item"
              :label="item"
              :value="item"
            >
            </el-option>
          </el-select>
        </template>
        <template #append>
          <el-button @click.stop="searchEvent" v-if="!movieInfo.searchRunning">
            <el-icon style="vertical-align: middle">
              <Search />
            </el-icon>
          </el-button>
          <el-button
            icon="el-icon-loading"
            @click.stop="stopSearchEvent"
            v-if="movieInfo.searchRunning"
            title="点击可停止搜索"
          >
            <el-icon>
              <Loading />
            </el-icon>
          </el-button>
        </template>
      </el-autocomplete>
    </div>
    <div class="listpage-body" id="movie-body" infinite-wrapper>
      <div
        class="show-picture"
        v-if="moviesConf.moviesViewMode === 'picture' && !movieInfo.showFind"
      >
        <Waterfall
          v-infinite-scroll="infiniteHandler"
          infinite-scroll-distance="100"
          ref="moviesWaterfall"
          :list="movieInfo.moviesFilteredList"
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
        v-if="moviesConf.moviesViewMode === 'table' && !movieInfo.showFind"
      >
        <el-table
          size="small"
          :data="movieInfo.moviesFilteredList"
          ref="moviesTableRef"
          height="100%"
          :empty-text="movieInfo.statusText"
          @row-click="detailEvent"
          v-infinite-scroll="infiniteHandler"
          infinite-scroll-distance="100"
        >
          <el-table-column prop="name" label="片名"> </el-table-column>
          <el-table-column
            v-if="movieInfo.classType.name === '最新'"
            prop="type"
            label="类型"
            width="100"
          >
          </el-table-column>
          <el-table-column prop="year" label="上映" align="center" width="80">
          </el-table-column>
          <el-table-column prop="area" label="地区" width="100">
          </el-table-column>
          <el-table-column prop="lang" label="语言" width="100">
          </el-table-column>
          <el-table-column
            v-if="movieInfo.showTableLastColumn"
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
            width="220"
          >
            <template #default="scope">
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button @click.stop="starEvent(scope.row)" link
                >收藏</el-button
              >
              <el-button
                @click.stop="
                  downloadEvent(
                    getSiteByKey(scope.row.siteKey),
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
        v-if="moviesConf.moviesViewMode === 'table' && movieInfo.showFind"
      >
        <el-table
          size="small"
          ref="searchResultTableRef"
          :data="movieInfo.filteredSearchContents"
          height="100%"
          :empty-text="movieInfo.statusText"
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
            v-if="movieInfo.showTableLastColumn"
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
              <el-button
                @click.stop="
                  downloadEvent(
                    getSiteByKey(scope.row.siteKey),
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
        v-if="moviesConf.moviesViewMode === 'picture' && movieInfo.showFind"
      >
        <Waterfall
          ref="filmSearchWaterfall"
          :list="movieInfo.filteredSearchContents"
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
      <el-backtop target="#movie-body" :right="100" :bottom="40" />
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
  nextTick,
} from "vue";
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { useStarStore } from "@/store/star";
import { storeToRefs } from "pinia";
import moviesApi from "@/api/movies";
import { ElMessage } from "element-plus";
import { invoke } from "@tauri-apps/api/tauri";
import { _ } from "lodash";
import { Waterfall } from "vue-waterfall-plugin-next";
import "vue-waterfall-plugin-next/dist/style.css";
import ImageLazy from "@/components/ImageLazy.vue";
import { useDownloadStore } from "@/store/download";
import doubanApi from "@/api/douban";
import ContextMenu from '@imengyu/vue3-context-menu';
import { useDark } from "@vueuse/core";

const FILM_DATA_CACHE = {};

export default defineComponent({
  name: "Movie",
  components: {
    Waterfall,
    ImageLazy,
  },
  
  setup() {
    const isDark = useDark();

    const orderFun = (a, b) => {
      const field = movieInfo.sortKeyword.field;
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
      if (movieInfo.sortKeyword.reverse) {
        let temValue = aValue;
        aValue = bValue;
        bValue = temValue;
      }
      if (movieInfo.sortKeyword.type == 'zh') {
        return aValue.localeCompare(bValue, "zh");
      } else {
        return aValue - bValue;
      }
    };

    const movieInfo = reactive({
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
      moviesFilteredList: [],
      infiniteId: 1,
      statusText: "",
      showTableLastColumn: false,
      filteredSearchContents: [],
      searchGroups: ["站内", "组内", "全站"],
      sortKeyword: {},
      sortKeywords: [
        {
          name: "不排序",
          field: "",
          orderFun: null,
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
      ],
      searchRunning: false,
      selectedSearchClassNames: [],
      searchClassList: [],
      currentSite: {},
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

    const coreStore = useCoreStore();
    const { view, playInfo } = storeToRefs(coreStore);

    const downloadStore = useDownloadStore();
    const { downloadMovie } = downloadStore; 

    const movieStore = useMovieStore();
    const {
      getAllSite,
      getSiteByKey,
      refreshMoviesConf,
      getMoviesConf,
      refreshSearchRecordList,
      getAllSearchRecord,
    } = movieStore;
    const {
      siteList,
      detail,
      moviesConf,
      searchRecordList,
    } = storeToRefs(movieStore);

    const starStore = useStarStore();
    const {
      starMovie
    } = starStore;
    const { starMap } = storeToRefs(movieStore);

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
        return [movieInfo.currentSite];
      if (moviesConf.value.searchGroup === "组内")
        return siteList.value.filter(
          (site) => site.group === movieInfo.currentSite.group
        );
      if (moviesConf.value.searchGroup === "全站") return siteList.value;
      return siteList.value.filter((site) => site.isActive);
    });

    const toggleViewMode = () => {
      if (movieInfo.showFind) {
        moviesConf.value.moviesViewMode =
          moviesConf.value.moviesViewMode === "picture" ? "table" : "picture";
      } else {
        moviesConf.value.moviesViewMode =
          moviesConf.value.moviesViewMode === "picture" ? "table" : "picture";
      }
      refreshMoviesConf();
    };

    const initSite = async () => {
      movieInfo.selectedSiteName = siteList.value[0].key;
      siteClick(movieInfo.selectedSiteName);
    };

    const siteClick = (siteKey) => {
      movieInfo.currentSite = getSiteByKey(siteKey);
      if (moviesConf.value.searchGroup === "站内" && movieInfo.searchTxt) {
        searchEvent();
        return;
      } else {
        movieInfo.searchTxt = "";
      }
      movieInfo.showFind = false;
      classList.value = [];
      if (FILM_DATA_CACHE[movieInfo.currentSite.key]) {
        classList.value = FILM_DATA_CACHE[movieInfo.currentSite.key];
        classClick(movieInfo.selectedClassName);
      } else {
        refreshClass();
      }
    };

    const refreshClass = () => {
      let classPromise = new Promise((resolve, reject) => {
        moviesApi
          .getSiteClass(movieInfo.currentSite)
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
            FILM_DATA_CACHE[movieInfo.currentSite.key] = classList.value;
            classClick(movieInfo.classType.name);
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
        movieInfo.classType = classType;
      }
      if (!movieInfo.classType) {
        movieInfo.classType = classList.value[0];
      }
      if (movieInfo.selectedClassName.endsWith("剧"))
        movieInfo.selectedAreas = [];
      const cacheKey =
        movieInfo.currentSite.key + "@" + movieInfo.classType.id;
      if (FILM_DATA_CACHE[cacheKey]) {
        moviesPageInfo.value = FILM_DATA_CACHE[cacheKey];
      } else {
        let params = {};
        if (movieInfo.classType.id != -1) {
          params.t = movieInfo.classType.id;
        }
        moviesApi.pageMovies(movieInfo.currentSite, params).then((res) => {
          moviesPageInfo.value = res;
          infiniteHandler();
        });
      }
    };

    const areaClick = (item) => {
      if (_.includes(movieInfo.selectedAreas, item)) {
        movieInfo.selectedAreas = _.dropWhile(movieInfo.selectedAreas, (o) => {
          return o == item
        })
      } else {
        movieInfo.selectedAreas.push(item);
      }
      refreshFilteredList();
    }

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

    const refreshFilteredList = _.debounce(() => {
      let filteredData = movieInfo.showFind
        ? movieInfo.searchContents
        : moviesPageInfo.value.moviesList;
      if (movieInfo.showFind)
        filteredData = filteredData.filter(
          (x) =>
            movieInfo.selectedSearchClassNames.length === 0 ||
            movieInfo.selectedSearchClassNames.includes(x.type)
        );
      filteredData = filteredData.filter(
        (x) =>
          movieInfo.selectedAreas.length === 0 ||
          movieInfo.selectedAreas.includes(x.area)
      );
      filteredData = filteredData.filter(
        (res) =>
          !moviesConf.value.excludeR18Films ||
          !containsClassFilterKeyword(res.type)
      );
      if (!movieInfo.showFind)
        movieInfo.selectedClassName =
          movieInfo.classType.name +
          "    " +
          filteredData.length +
          "/" +
          moviesPageInfo.value.recordcount;

      if (movieInfo.sortKeyword.orderFun)
        filteredData.sort(movieInfo.sortKeyword.orderFun);

      _.uniqBy(filteredData, "name")
      if (movieInfo.showFind) {
        movieInfo.filteredSearchContents = filteredData;
      } else {
        movieInfo.moviesFilteredList = filteredData;
      }
    }, 500);

    const detailEvent = (e) => {
      let siteKey;
      if (movieInfo.showFind) {
        siteKey = e.site.key;
      } else {
        siteKey = movieInfo.currentSite.key;
      }
      detail.value = {
        show: true,
        siteKey: siteKey,
        ids: e.id.toString(),
      };
    };

    const playEvent = async (e) => {
      playInfo.value.playType = "onlineMovie";
      playInfo.value.name = e.name;
      if (movieInfo.showFind) {
        playInfo.value.movie.siteKey = e.site.key;
      } else {
        playInfo.value.movie.siteKey = movieInfo.currentSite.key;
      }
      playInfo.value.movie.ids = e.id;
      playInfo.value.movie.index = 0;
      playInfo.value.movie.videoFlag = "";
      playInfo.value.movie.playMode = "local";
      view.value = "Play";
    };

    const starEvent = async (e) => {
      let siteKey,
        ids = e.id.toString();
      if (movieInfo.showFind) {
        siteKey = e.site.key;
      } else {
        siteKey = movieInfo.currentSite.key;
      }
      let star = {
        star_name: e.name,
        ids: ids,
        site_key: siteKey,
        movie_type: e.type,
        year: e.year + "年",
        note: e.note,
        douban_rate: await doubanApi.doubanRate(e.name, e.year),
        last_update_time: e.last,
        pic: e.pic,
        area: e.area,
      };
      starMovie(star);
    };

    const downloadEvent = () => {
      downloadMovie(getSiteByKey(detail.value.siteKey), detail.value.ids);
    };
    
    const infiniteHandler = _.debounce(() => {
      const key = movieInfo.currentSite.key;
      let typeTid = movieInfo.classType.id;
      let page = moviesPageInfo.value.pageCount;
      let totalPageCount = moviesPageInfo.value.totalPageCount;
      page = totalPageCount - page + 1;
      movieInfo.statusText = " ";
      if (key === undefined || page < 1 || page > totalPageCount) {
        movieInfo.statusText = "暂无数据";
        return false;
      }
      setTimeout(() => {
        let params = {
          pg: page,
          t: typeTid == -1 ? undefined : typeTid,
        };
        moviesApi.listMovies(movieInfo.currentSite, params).then(
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
                moviesPageInfo.value.moviesList.push(...res);
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
              const cacheKey = movieInfo.currentSite.key + "@" + typeTid;
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
            movieInfo.filteredSearchContents.map((row) => row.site.name)
          ),
        ].map((e) => {
          return { text: e, value: e };
        });
      return [
        ...new Set(movieInfo.filteredSearchContents.map((row) => row[column])),
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
      const wd = movieInfo.searchTxt;
      if (!wd) return;
      movieInfo.searchId += 1;
      movieInfo.searchContents = [];
      movieInfo.showFind = true;
      movieInfo.statusText = " ";
      movieInfo.searchRunning = true;
      movieInfo.siteSearchCount = 0;
      searchSites.value.forEach((site) => {
        const id = movieInfo.searchId;
        moviesApi
          .search(site, wd)
          .then((res) => {
            if (id !== movieInfo.searchId || !movieInfo.searchRunning) return;
            if (_.isArray(res)) {
              let count = 0;
              res.forEach((element) => {
                moviesApi
                  .detail(site, element.id)
                  .then((detailRes) => {
                    if (id !== movieInfo.searchId || !movieInfo.searchRunning)
                      return;
                    if (detailRes) {
                      detailRes.site = site;
                      if (isValidSearchResult(detailRes)) {
                        movieInfo.searchContents.push(detailRes);
                        movieInfo.searchContents.sort(function (a, b) {
                          return a.site.id - b.site.id;
                        });
                        refreshFilteredList();
                      }
                    }
                  })
                  .finally(() => {
                    count++;
                    if (count === res.length) {
                      movieInfo.siteSearchCount++;
                      movieInfo.statusText = "暂无数据";
                    }
                  });
              });
            } else if (_.isObject(res)) {
              moviesApi
                .detail(site, res.id)
                .then((detailRes) => {
                  if (id !== movieInfo.searchId || !movieInfo.searchRunning)
                    return;
                  detailRes.site = site;
                  if (isValidSearchResult(detailRes)) {
                    movieInfo.searchContents.push(detailRes);
                    movieInfo.searchContents.sort(function (a, b) {
                      return a.site.id - b.site.id;
                    });
                  }
                })
                .finally(() => {
                  movieInfo.siteSearchCount++;
                  movieInfo.statusText = "暂无数据";
                });
            } else if (res === undefined) {
              movieInfo.siteSearchCount++;
              movieInfo.statusText = "暂无数据";
              if (moviesConf.value.searchGroup === "站内")
                ElMessage.info("没有查询到数据！");
            }
          })
          .catch(() => {
            movieInfo.siteSearchCount++;
            if (moviesConf.value.searchGroup === "站内")
              ElMessage.error("本次查询状态异常，未获取到数据！");
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
      const wd = movieInfo.searchTxt;
      if (wd) {
        await invoke("save_search_record", {
          searchRecordInfo: { keywords: wd },
        });
        refreshSearchRecordList();
      }
    };

    const searchClearEvent = () => {
      if (!movieInfo.searchTxt) {
        movieInfo.searchContents = [];
        movieInfo.showFind = false;
        movieInfo.searchRunning = false;
        initSite();
      }
    };

    const stopSearchEvent = () => {
      movieInfo.searchRunning = false;
    };

    const onContextMenu = (e ) => {
      e.preventDefault();
      movieInfo.options.x = e.x;
      movieInfo.options.y = e.y;
      movieInfo.contextMenushow = true;
      nextTick(() => {
        var elements = document.querySelectorAll(".mx-menu-ghost-host");
        elements.forEach(function (element) {
          // 为每个元素添加事件监听器
          element.addEventListener("contextmenu", function (e) {
            e.preventDefault(); // 阻止默认行为
          });
        });
      });
    }

    const startPageChange = () => {
      moviesPageInfo.value.moviesList = [];
      infiniteHandler();
    }

    watch(
      () => movieInfo.searchTxt,
      async () => {
        if (movieInfo.searchTxt === "清除历史记录...") {
          await invoke("del_all_search_record", {});
          refreshSearchRecordList();
          movieInfo.searchTxt = "";
          searchClearEvent();
        }
      }
    );

    watch(
      () => movieInfo.siteSearchCount,
      () => {
        if (movieInfo.siteSearchCount === searchSites.value.length) {
          movieInfo.searchRunning = false;
        }
      }
    );
    
    watch(
      () => movieInfo.moviesFilteredList,
      () => {
        let moviesFilteredList = movieInfo.moviesFilteredList.filter(res => !moviesConf.value.excludeR18Films || !containsClassFilterKeyword(res.type))
        movieInfo.areas = [...new Set(moviesFilteredList.map(ele => ele.area))].filter(x => x)
        movieInfo.searchClassList = [...new Set(moviesFilteredList.map(ele => ele.type))].filter(x => x)
        refreshFilteredList()
      },  {
        deep: true
      }
    )

    onMounted(() => {
      movieInfo.sortKeyword = movieInfo.sortKeywords[0];
      getMoviesConf();
      getAllSearchRecord();
      getAllSite().then(() => {
        initSite();
      });
    });

    return {
      movieInfo,
      siteList,
      siteClick,
      classList,
      classClick,
      toggleViewMode,
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
      moviesConf,
      querySearch,
      searchEvent,
      searchAndRecord,
      searchClearEvent,
      stopSearchEvent,
      downloadEvent,
      searchResultTableRef,
      onContextMenu,
      areaClick,
      startPageChange,
      starMap,
    };
  },
});
</script>
