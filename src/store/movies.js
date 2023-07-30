import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import moviesApi from '@/api/movies';

export const useMoviesStore = defineStore('movies', {
    state: () => {
      return {
        movieInfo: {
            siteKey: "",
            ids: "",
            name: "",
            index: 0,
            videoFlag: "",
            onlineUrl: "",
        },
        init: {
            siteListInit: false,
            moviesConfInit: false,
            searchRecordListInit: false,
        },
        moviesConf: {
            searchGroup: "全站",
            excludeRootClasses: false,
            rootClassFilter: [],
            excludeR18Films: false,
            r18ClassFilter: [],
            moviesViewMode: "picture",
        },
        detail: {
            ids: "",
            siteKey: "",
            show: false,
        },
        siteList: [],
        moviesDetailCache: {
        },
        searchRecordList: [],
      }
    },
    actions:{
      
      async getAllSite() {
        if (!this.init.siteListInit) {
          await this.refreshSiteList();
          this.init.siteListInit = true;
        }
      },

      getSiteByKey(siteKey, type = 1) {
        const site = this.siteList.find(e => e.key === siteKey)
        if (site) {
          if (type == 1) {
            return site.name
          } else {
            return site
          }
        }
      },

      async refreshSiteList() {
        let siteList = await invoke("select_site", {});
        this.siteList = siteList;
      },

      async getMoviesDetailCacheByKey(siteKey, ids) {
        return new Promise((resolve, reject) => {
          const uqkey = siteKey + '@' + ids;
          if (!this.moviesDetailCache[uqkey]) {
            moviesApi.detail(this.getSiteByKey(siteKey, 2), ids).then(res => {
              this.moviesDetailCache[uqkey] = res
              resolve(this.moviesDetailCache[uqkey])
            })
          } else {
            resolve(this.moviesDetailCache[uqkey]);
          }
        });
      },

      async refreshMoviesConf() {
        await invoke("config_update", { data: {moviesConf: this.moviesConf} });
      },

      async getMoviesConf() {
        if (!this.init.moviesConfInit) {
          let moviesConfStr = await invoke("get_conf_by_name", { confName: "moviesConf" });
          const moviesConf = JSON.parse(moviesConfStr);
          this.moviesConf = moviesConf;
          this.init.moviesConfInit = true;
        }
      },

      async getAllSearchRecord() {
        if (!this.init.searchRecordListInit) {
          await this.refreshSearchRecordList();
          this.init.searchRecordListInit = true;
        }
      },

      async refreshSearchRecordList() {
        let searchRecordList = await invoke("select_search_record", {});
        searchRecordList = searchRecordList.reverse();
        searchRecordList.push({ id: searchRecordList.length + 1, keywords: '清除历史记录...' })
        this.searchRecordList = searchRecordList;
      },
    },
    getters:{
      moviesDetail() {
        return this.detail;
      }
    }
  })