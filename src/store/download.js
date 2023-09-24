import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { _ } from "lodash";
import { useCoreStore } from "./index";
import moviesApi from '@/api/movies';
import { ElMessage } from 'element-plus';
import util from '@/util';

export const useDownloadStore = defineStore("download", {
  state: () => {
    return {
      init: {
        downloadListInit: false,
      },
      downloadId: 0,
      downloadList: [],
    };
  },
  actions: {
    async getAllDownload() {
      if (!this.init.downloadListInit) {
        await this.refreshDownloadList();
        this.init.downloadListInit = true;
      }
    },

    async refreshDownloadList() {
      let downloadList = await invoke("select_download_info", {});
      this.downloadList = downloadList;
    },

    async downloadMovie(site, ids) {
      const historyStr = await invoke("get_history_by_uq", {
        siteKey: site.key,
        ids: ids,
      });
      let videoFlag;
      let history;
      if (historyStr) {
        history = JSON.parse(historyStr);
        videoFlag = history.videoFlag;
      }
      moviesApi
        .download(site, ids, videoFlag)
        .then(async (res) => {
          let downloadInfos = [];
          res.downloadUrls.forEach((url) => {
            let movieName = history ? history.name : url.name;
            downloadInfos.push({
              id: null,
              movieName: util.trimAll(movieName),
              url: url.url,
              subTitleName: url.subTitleName,
              status: "parseSource",
              downloadCount: 0,
              count: 0,
              parentId: "0",
              downloadStatus: "wait",
            });
          });
          await invoke("insert_download_infos", { downloadInfoList: downloadInfos });
          this.refreshDownloadList();
          ElMessage.success(res.info);
        })
        .catch((err) => {
          console.log(err);
          ElMessage.error(err.info);
        });
    },
  },
  getters: {
    currentChannel() {
      const core = useCoreStore();
      return this.channelGroupList.filter(
        (item) => item.id == core.playInfo.iptv.channelGroupId
      )[0];
    }
  },
});
