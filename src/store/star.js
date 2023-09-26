import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { useMovieStore } from "./movie";
import moviesApi from "@/api/movies";
import { ElMessage } from "element-plus";

export const useStarStore = defineStore("star", {
  state: () => {
    return {
      init: {
        starListInit: false,
      },
      types: [],
      areas: [],
      starList: [],
      starMap: {},
    };
  },
  actions: {
    async refreshStarList() {
      let starList = await invoke("select_star", {});
      let areas = new Set();
      let types = new Set();
      this.starMap = {};
      starList.map((item) => {
        if (item.area) {
          areas.add(item.area);
        }
        if (item.movieType) {
          types.add(item.movieType);
        }
        if (!this.starMap[item.site_key]) {
          this.starMap[item.site_key] = {};
        }
        this.starMap[item.site_key][item.ids] = true;
      });
      this.areas = areas;
      this.types = types;
      this.starList = starList;
    },

    async getAllStar() {
      if (!this.init.starListInit) {
        await this.refreshStarList();
        this.init.starListInit = true;
      }
    },

    async starMovie(star) {
      const movieStore = useMovieStore();
      star.id = null;
      star.has_update = "0";
      star.position = 0.0;

      let ids = star.ids,
        siteKey = star.site_key;
      const starHistory = await invoke("get_star_by_uq", {
        starSiteKey: siteKey,
        starIds: ids,
      });
      if (starHistory) {
        await invoke("del_star", { starId: starHistory.id });
        moviesInfo.isStar = false;
        this.refreshStarList();
        ElMessage({
          showClose: true,
          message: "取消收藏成功",
          type: "success",
        });
      } else {
        const cacheKey = siteKey + "@" + ids;
        if (!movieStore.movieDetailCache[cacheKey]) {
          movieStore.movieDetailCache[cacheKey] = await moviesApi.detail(
            movieStore.getSiteByKey(siteKey),
            ids
          );
        }
        await invoke("save_star", { starInfo: star });
        ElMessage({ showClose: true, message: "收藏成功", type: "success" });
      }
    },
  },
  getters: {
    starAreas() {
      return this.areas;
    },

    starTypes() {
      return this.types;
    },

    starReverseList() {
      return this.starList.reverse();
    },
  },
});
