import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";

export const useHistoryStore = defineStore('history', {
    state: () => {
        return {
            init: {
                historyListInit: false,
            },
            history: {
                types: [],
                areas: [],
                historyList: [],
            },
        }
    },
    actions:{

      async refreshHistoryList() {
        let historyList = await invoke("select_history", {});
        let areas = new Set();
        let types = new Set();
        historyList.map(item => {
          item.detail = JSON.parse(item.detail);
          if (item.detail.area) {
            areas.add(item.detail.area);
          }
          if (item.detail.type) {
            types.add(item.detail.type);
          }
        });
        this.history = {
          areas: areas,
          types: types,
          historyList: historyList,
        };
      },

      async getAllHistory() {
        if (!this.init.historyListInit) {
          await this.refreshHistoryList();
          this.init.historyListInit = true;
        }
      },
    },
    getters:{
      historyAreas() {
        return this.history.areas;
      },

      historyTypes() {
        return this.history.types;
      },

      historyList() {
        return this.history.historyList;
      },
    }
  })