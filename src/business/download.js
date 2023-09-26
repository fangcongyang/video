import { invoke } from "@tauri-apps/api/tauri";

export class DownloadBus {
    wsAddr = "ws://127.0.0.1:8000";
    downloadRequest = {
        id: "0",
        downloadInfo: null,
    }
    ws;
    updateDownloadInfoEvent;
    downloadInterval;

    constructor(downloadInfo) {
        this.downloadRequest.downloadInfo = downloadInfo;
        this.ws = new WebSocket(this.wsAddr);
        this.initSocket();
    }

    isWsOpen = () => {
        return this.ws && this.ws.readyState === 1
    };

    intervalGetDownloadInfo = () => {
      this.downloadInterval = setInterval(async () => {
        let downloadInfo = await invoke("get_download_info_by_queue", {});
        if (downloadInfo != null && downloadInfo != undefined) {
          clearInterval(this.downloadInterval);
          this.downloadRequest.downloadInfo = downloadInfo;
          this.ws.send(JSON.stringify(this.downloadRequest));
        }
      }, 1000);
    }

    initSocket = () => {
        this.ws.onopen = () => {
          this.intervalGetDownloadInfo();
        }

        this.ws.onclose = () => {
          setTimeout(() => {
            this.initSocket();
          }, 3000)
        }

        this.ws.onerror = () => {
          console.log('websoket连接失败，请刷新！')
        }
        
        this.ws.onmessage = ({ data }) => {
          const dataObj = JSON.parse(data);
          this.downloadRequest.downloadInfo.status = dataObj.status;
          this.downloadRequest.downloadInfo.download_status = dataObj.download_status;
          this.updateDownloadInfoEvent(dataObj);
          if (dataObj.download_status == "downloadSuccess" || dataObj.download_status == "downloadFail") {
            this.intervalGetDownloadInfo();
          } else {
            if (dataObj.mes_type == "progress" && dataObj.status == "downloadSlice") {
              this.downloadRequest.downloadInfo.download_count = dataObj.download_count;
            } else {
              this.downloadRequest.downloadInfo.count = dataObj.count;
              this.downloadRequest.downloadInfo.download_count = dataObj.download_count;
              this.isWsOpen() && this.ws.send(JSON.stringify(this.downloadRequest));
            }
          }
      };
    }
}