import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";

export const useCoreStore = defineStore('core', {
    state: () => {
      return {
        view: "Movies",
        video: {
          isLive: false,
          // iptv, movies, online
          playType: "",
        },
        init: {
          playerConfInit: false,
          systemConfInit: false,
          shortcutInit: false,
        },
        systemConf: {
          theme: "theme-light",
          saveWindowState: false,
          excludeRootClasses: false,
          rootClassFilter: [],
          excludeR18Films: false,
          r18ClassFilter: [],
          shortcut: true,
          allowPassWhenIptvCheck: false,
          autocleanWhenIptvCheck: false,
          pauseWhenMinimize: false,
          starViewMode: "picture",
          historyViewMode: 'picture',
          shiftTooltipLimitTimes: 5,
          mainWidth: 1080.0,
          mainHeight: 720.0,
          shortcutModified: false,
        },
        playerConf: {
          volume: 0.6,
          autoChangeSourceWhenIptvStalling: true,
          waitingTimeInSec: 5,
          forwardTimeInSec: 5,
        },
        shortcutList: [],
        shiftDown: false,
      }
    },
    actions:{
      
      async updateSystemConf() {
        await invoke("config_update", { data: {systemConf: this.systemConf} });
      },

      async refreshSystemConf() {
        let systemConfStr = await invoke("get_conf_by_name", { confName: "systemConf" });
        const systemConf = JSON.parse(systemConfStr);
        this.systemConf = systemConf;
      },

      async getSystemConf() {
        if (!this.init.systemConfInit) {
          await this.refreshSystemConf();
          this.init.systemConfInit = true;
        }
      },

      async refreshPlayerConf() {
        let playerConfStr = await invoke("get_conf_by_name", { confName: "playerConf" });
        const playerConf = JSON.parse(playerConfStr);
        this.playerConf = playerConf;
      },
      
      async updatePlayerConf() {
        await invoke("config_update", { data: {playerConf: this.playerConf} });
      },

      async getPlayerConf() {
        if (!this.init.playerConfInit) {
          await this.refreshPlayerConf();
          this.init.playerConfInit = true;
        }
      },

      async refreshShortcut() {
        this.shortcutList = await invoke("select_shortcut", {});
      },

      async getAllShortcut() {
        if (!this.init.shortcutInit) {
          await this.refreshShortcut();
          this.init.shortcutInit = true;
        }
      },

      showShiftPrompt() {
        if (this.systemConf.shiftTooltipLimitTimes) {
          ElMessage.info('多选时支持shift快捷键');
          this.systemConf.shiftTooltipLimitTimes--;
          updateSystemConf();
        }
      }

    },
    getters:{
    }
  })