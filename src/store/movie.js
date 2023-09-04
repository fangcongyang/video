import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import moviesApi from '@/api/movies';
import { useCoreStore } from './index';

export const useMovieStore = defineStore('movie', {
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
        movieDetailCache: {
        },
        searchRecordList: [],
        videoDetailCache: {}
      }
    },
    actions:{
      
      async getAllSite() {
        if (!this.init.siteListInit) {
          await this.refreshSiteList();
          this.init.siteListInit = true;
        }
      },

      async refreshSiteList() {
        let siteList = await invoke("select_site", {});
        this.siteList = siteList;
      },

      async getMoviesDetailCacheByKey(siteKey, ids) {
        return new Promise((resolve, reject) => {
          const uqkey = siteKey + '@' + ids;
          if (!this.movieDetailCache[uqkey]) {
            moviesApi.detail(this.getSiteByKey(siteKey, 2), ids).then(res => {
              this.movieDetailCache[uqkey] = res
              resolve(this.movieDetailCache[uqkey])
            })
          } else {
            resolve(this.movieDetailCache[uqkey]);
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

      async fetchPlaylist(cacheKey) {
        return new Promise((resolve, reject) => {
          let videoInfo = this.videoDetailCache[cacheKey];
          const core = useCoreStore()
          if (videoInfo && videoInfo.list && videoInfo.list.length) {
            core.playInfo.name = videoInfo.name
            resolve(this.videoDetailCache[cacheKey].list)
          }
          if (!this.movieDetailCache[cacheKey]) {
            moviesApi.detail(this.getSiteByKey(core.playInfo.movie.siteKey), core.playInfo.movie.ids).then(res => {
              this.movieDetailCache[cacheKey] = res
              core.playInfo.name = res.name
              this.videoDetailCache[cacheKey] = {
                list: res.fullList,
                name: res.name
              }
              resolve(res.fullList)
            }).catch(err => {
              reject();
            })
          } else {
            let res = this.movieDetailCache[cacheKey]
            core.playInfo.name = res.name
            resolve(res.fullList)
          }
        })
      },
      
      getSiteNameByKey(siteKey) {
        return this.getSiteByKey(siteKey).name;
      },

      getSiteByKey(siteKey) {
        const site = this.siteList.find(e => e.key === siteKey)
        return site
      },
    },
    getters:{
      moviesDetail() {
        return this.detail;
      },
    }
  })