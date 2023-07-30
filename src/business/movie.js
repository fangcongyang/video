
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import moviesApi from '@/api/movies';
import { ElMessage } from 'element-plus';

export const downloadEvent = async (site, ids) => {
    const historyStr = await invoke("get_history_by_uq", {siteKey: site.key, ids: ids});
    let videoFlag
    if (historyStr) videoFlag = JSON.parse(historyStr).videoFlag
    moviesApi.download(site, ids, videoFlag).then(async(res) => {
        await writeText(res.downloadUrls)
        ElMessage.success(res.info)
    }).catch((err) => {
        ElMessage.error(err.info)
    })
}