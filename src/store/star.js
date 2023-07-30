import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";

export const useStarStore = defineStore('star', {
    state: () => {
        return {
            init: {
                starListInit: false,
            },
            star: {
                types: [],
                areas: [],
                starList: [],
            },
        }
    },
    actions:{

      async refreshStarList() {
        let starList = await invoke("select_star", {});
        let areas = new Set();
        let types = new Set();
        starList.map(item => {
          if (item.area) {
            areas.add(item.area);
          }
          if (item.type) {
            types.add(item.type);
          }
        });
        this.star = {
          areas: areas,
          types: types,
          starList: starList,
        };
      },
      
      async getAllStar() {
        if (!this.init.starListInit) {
          await this.refreshStarList();
          this.init.starListInit = true;
        }
      },
    },
    getters:{
      starAreas() {
        return this.star.areas;
      },

      starTypes() {
        return this.star.types;
      },

      starReverseList() {
        return this.star.starList.reverse();
      }
    }
  })