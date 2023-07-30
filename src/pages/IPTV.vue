<template>
  <div class="listpage" id="iptv">
    <div
      class="listpage-header"
      id="iptv-header"
      v-show="!iptvInfo.enableBatchEdit"
    >
      <el-switch
        v-model="iptvInfo.enableBatchEdit"
        active-text="批处理及频道调整"
      ></el-switch>
      <el-button
        :icon="Upload"
        @click.stop="exportChannels"
        title="导出m3u时必须手动添加扩展名，要保存频道配置信息请选择json格式"
        >导出</el-button
      >
      <el-button
        @click.stop="importChannels"
        :icon="Download"
        title='支持同时导入多个文件,导入m3u时网址可带参数、含有"#"号时自动分割'
      >
        导入
      </el-button>
      <el-button
        @click="checkAllChannels"
        :icon="Refresh"
        :loading="iptvInfo.checkAllChannelsLoading"
        title="可在后台运行"
      >
        检测{{
          iptvInfo.checkAllChannelsLoading
            ? iptvInfo.checkProgress + "/" + this.iptvList.length
            : ""
        }}
      </el-button>
      <el-button @click.stop="resetChannelsEvent" :icon="RefreshLeft"
        >重置</el-button
      >
    </div>
    <div
      class="listpage-header"
      id="iptv-header"
      v-show="iptvInfo.enableBatchEdit"
    >
      <el-switch
        v-model="iptvInfo.enableBatchEdit"
        active-text="批处理及频道调整"
      ></el-switch>
      <el-input
        placeholder="新组名/新频道名"
        v-model="iptvInfo.inputContent"
      ></el-input>
      <el-switch
        v-model="iptvInfo.batchIsActive"
        active-text="启用"
      ></el-switch>
      <el-button
        type="primary"
        icon="el-icon-edit"
        @click.stop="saveBatchEdit"
        title="输入框组名为空时仅保存开关状态"
        >保存分组与开关状态</el-button
      >
      <el-button
        type="primary"
        icon="el-icon-film"
        @click.stop="mergeChannel"
        title="勾选单个时可重命名频道"
        >{{
          iptvInfo.multipleSelection.length === 1 ? "频道重命名" : "频道合并"
        }}</el-button
      >
      <el-button
        @click.stop="removeSelectedChannels"
        icon="el-icon-delete-solid"
        >删除</el-button
      >
    </div>
    <div class="listpage-body" id="iptv-table">
      <div class="show-table" id="iptv-table">
        <el-table
          ref="iptvTableRef"
          size="small"
          fit
          height="100%"
          row-key="id"
          :data="filteredTableData()"
          lazy
          :load="(row, treeNode, resolve) => resolve(row.channels)"
          :tree-props="{ hasChildren: 'hasChildren' }"
          @expand-change="expandChange"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
          @sort-change="handleSortChange"
        >
          <el-table-column type="selection" v-if="iptvInfo.enableBatchEdit">
          </el-table-column>
          <el-table-column
            default-sort="ascending"
            prop="name"
            :class-name="iptvInfo.enableBatchEdit ? 'disableExpand' : ''"
            label="频道名"
          >
            <template #header>
              <el-input
                placeholder="搜索"
                size="small"
                v-model.trim="iptvInfo.searchTxt"
                :suffix-icon="Search"
              >
              </el-input>
            </template>
          </el-table-column>
          <el-table-column
            prop="active"
            width="120"
            align="center"
            :filters="[
              { text: '启用', value: '1' },
              { text: '停用', value: '0' },
            ]"
            :filter-method="(value, row) => value === row.active"
            label="启用"
          >
            <template #default="scope">
              <el-switch
                v-model="scope.row.active"
                active-value="1"
                inactive-value="0"
                @click.native.stop="isActiveChangeEvent(scope.row)"
              >
              </el-switch>
            </template>
          </el-table-column>
          <el-table-column
            sortable
            :sort-method="
              (a, b) => sortByLocaleCompare(a.channelGroup, b.channelGroup)
            "
            prop="channelGroup"
            label="分组"
            :filters="channelGroupFilter"
            :filter-method="(value, row) => value === row.channelGroup"
            filter-placement="bottom-end"
          >
            <template #default="scope">
              <el-button link>{{ scope.row.channelGroup }}</el-button>
            </template>
          </el-table-column>
          <el-table-column
            label="状态"
            sortable
            :sort-by="['status']"
            width="120"
          >
            <template #default="scope">
              <span v-if="scope.row.status === ' '">
                <i class="el-icon-loading"></i>
                检测中...
              </span>
              <span v-else>{{ scope.row.status }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" align="right" :width="240">
            <template #header>
              <span>{{
                iptvInfo.enableBatchEdit
                  ? `频道总数:${channelGroupList.length}`
                  : `资源总数:${channelList.length}`
              }}</span>
            </template>
            <template #default="scope">
              <el-button
                @click.stop="moveToTopEvent(scope.row)"
                link
                v-if="scope.row.channels"
                >置顶</el-button
              >
              <el-button @click.stop="playEvent(scope.row)" link
                >播放</el-button
              >
              <el-button
                v-if="channelList.every((channel) => channel.status)"
                v-show="!iptvInfo.checkAllChannelsLoading"
                @click.stop="checkChannel(scope.row)"
                link
                >检测</el-button
              >
              <el-button @click.stop="removeEvent(scope.row)" link
                >删除</el-button
              >
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>
    <el-dialog
      v-model="iptvInfo.importChannelVisible"
      title="频道导入"
      width="30%"
    >
      <el-input
        v-model="iptvInfo.parseM3uUrl"
        size="large"
        placeholder="请输入解析地址"
      />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="importOnlineChannels">在线导入</el-button>
          <el-button type="primary" @click="importLocalChannels">
            本地文件
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  watch,
  onBeforeMount,
  computed,
  nextTick,
  ref,
} from "vue";
import { ElMessage } from "element-plus";
import { useCoreStore } from "@/store";
import { useIptvStore } from "@/store/iptv";
import { storeToRefs } from "pinia";
import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import {
  M3uGenerate,
  M3uParse,
  validityIptvUrl,
  determineGroup,
} from "@/business/iptv";
import fetch from "@/api/fetch";
import iptvApi from "@/api/iptv";
import { _ as lodash } from "lodash";
import { Upload, Download, Refresh, RefreshLeft, Search } from "@element-plus/icons-vue";

export default defineComponent({
  name: "Iptv",
  setup() {
    const iptvInfo = reactive({
      enableBatchEdit: false,
      batchIsActive: false,
      inputContent: "",
      checkAllChannelsLoading: false,
      importChannelVisible: false,
      parseM3uUrl: "",
      searchTxt: "",
      expandedRows: [],
      checkProgress: 0,
      stopFlag: false,
      selectionBegin: "",
      selectionEnd: "",
      multipleSelection: [],
    });
    const coreStore = useCoreStore();
    const { video, view, systemConf, shiftDown } = storeToRefs(coreStore);

    const iptvStore = useIptvStore();
    const { getAllChannelGroup, refreshChannelGroupList } = iptvStore;
    const {
      channelGroupId,
      channelGroupList,
      channelList,
      channelMap,
      channelGroupMap,
      channelGroupFilter,
    } = storeToRefs(iptvStore);

    const iptvTableRef = ref();

    const exportChannels = async () => {
      // 导出导入m3u为iptvList，json为channelList
      let filePath = await save({
        filters: [
          { name: "m3u file", extensions: ["m3u"] },
          { name: "JSON file", extensions: ["json"] },
        ],
      });
      if (filePath !== null) {
        if (filePath.endsWith("m3u")) {
          const m3uGenerate = new M3uGenerate();
          channelList.value.forEach((e) => {
            m3uGenerate.appendM3u(-1, e.name, e.url);
          });
          await writeTextFile(filePath, m3uGenerate.toString());
          ElMessage({
            showClose: true,
            message: "已保存成功",
            type: "success",
          });
        } else {
          if (!filePath.endsWith(".json")) filePath += ".json";
          const arr = [...channelList.value];
          const str = JSON.stringify(arr, null, 2);
          await writeTextFile(filePath, str);
          ElMessage({
            showClose: true,
            message: "已保存成功",
            type: "success",
          });
        }
      }
    };

    const importChannels = () => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage({
          showClose: true,
          message: "正在检测, 请勿操作.",
          type: "info",
        });
        return false;
      }
      iptvInfo.importChannelVisible = true;
    };

    const importOnlineChannels = async () => {
      try {
        const docs = [];
        const res = await fetch.get(iptvInfo.parseM3uUrl, null);
        const result = res.split("\n");
        for (const i of result) {
          if (i.includes("http") && validityIptvUrl(url)) {
            const j = i.split(",");
            const doc = {
              id: lodash.uniqueId("channel_"),
              name: j[0],
              url: j[1],
              ative: "1",
              status: "可用",
            };
            docs.push(doc);
          }
        }
        // 获取url不重复的列表
        const uniqueList = docs.filter((item) => !channelMap.value[item.url]);
        addChannelList(uniqueList);
        ElMessage.success("导入成功");
      } catch (error) {
        console.error(error);
        ElMessage.warning("导入失败");
      }
      iptvInfo.importChannelVisible = false;
    };

    const importLocalChannels = async () => {
      const selectedFiles = await open({
        multiple: true,
        filters: [
          {
            name: "支持的文件格式",
            extensions: ["m3u", "m3u8", "json", "txt"],
          },
        ],
      });
      let filePaths = [];
      if (Array.isArray(selectedFiles)) {
        filePaths = filePaths.concat(selectedFiles);
      } else if (selectedFiles != null) {
        filePaths.push(selectedFiles);
      }
      filePaths.forEach(async (file) => {
        if (file.endsWith("m3u") || file.endsWith("m3u8")) {
          const docs = [];
          const parser = new M3uParse();
          const playlist = await readTextFile(file);
          const result = parser.parse(playlist);
          result.items.forEach((ele) => {
            const urls = ele.url.split("#").filter((e) => e.startsWith("http")); // 网址带#时自动分割
            urls.forEach((url) => {
              if (ele.name && url && validityIptvUrl(url)) {
                // 网址可能带参数
                const doc = {
                  id: lodash.uniqueId("channel_"),
                  name: ele.name,
                  url: url,
                  ative: "1",
                  status: "可用",
                };
                docs.push(doc);
              }
            });
          });
          // 获取url不重复的列表
          const uniqueList = docs.filter((item) => !channelMap.value[item.url]);
          addChannelList(uniqueList);
          ElMessage.success("导入成功");
        }
        if (file.endsWith("json")) {
          const importedListStr = await readTextFile(file);
          const importedList = JSON.parse(importedListStr);
          // 获取url不重复的列表
          const uniqueList = importedList.filter(
            (item) => !channelMap.value[item.url]
          );
          addChannelList(uniqueList);
          ElMessage({ showClose: true, message: "导入成功", type: "success" });
        }
        if (file.endsWith("txt")) {
          try {
            const docs = [];
            const playlist = await readTextFile(file);
            const result = playlist.split("\n");
            for (const i of result) {
              if (i.includes("http")) {
                const j = i.split(",");
                const doc = {
                  id: lodash.uniqueId("channel_"),
                  name: j[0],
                  url: j[1],
                  ative: "1",
                  status: "可用",
                };
                docs.push(doc);
              }
            }
            // 获取url不重复的列表
            const uniqueList = docs.filter(
              (item) => !channelMap.value[item.url]
            );
            console.log(uniqueList);
            addChannelList(uniqueList);
            ElMessage.success("导入成功");
          } catch (error) {
            console.error(error);
            ElMessage.warning("导入失败");
          }
        }
      });
    };

    const addChannelList = async (uniqueList) => {
      const uniqueChannelName = {};
      for (let item of uniqueList) {
        let channelName = item.name
          .trim()
          .replace(/[- ]?(1080p|蓝光|超清|高清|标清|hd|cq|4k)(\d{1,2})?$/i, "");
        if (channelName.match(/cctv/i))
          channelName = channelName.replace("-", "");
        item.active = "1";
        item.status = "可用";
        if (uniqueChannelName[channelName]) {
          !uniqueChannelName[channelName].includes(item) &&
            uniqueChannelName[channelName].push(item);
        } else {
          uniqueChannelName[channelName] = [item];
        }
      }

      if (Object.keys(uniqueChannelName).length) {
        const channelGroupList = Object.keys(uniqueChannelName).map((e) => {
          const channelGroupName = determineGroup(e);
          let channelGroup = channelGroupMap.value[e + channelGroupName];
          if (channelGroup) {
            channelGroup.channels = channelGroup.channels.concat(
              uniqueChannelName[e]
            );
          } else {
            channelGroup = {
              id: 0,
              name: e,
              active: uniqueChannelName[e].some((c) => c.active === "1")
                ? "1"
                : "0",
              status: "可用",
              channelGroup: channelGroupName,
              hasChildren: uniqueChannelName[e].length > 1 ? "1" : "0",
              channels: uniqueChannelName[e],
            };
            channelGroupMap.value[e + channelGroupName] = channelGroup;
          }
          return channelGroup;
        });

        await invoke("save_channel_groups", { channelGroups: channelGroupList });
        refreshChannelGroupList();
      }
      iptvInfo.importChannelVisible = false;
    };

    // 查找电视
    const filteredTableData = () => {
      if (iptvInfo.searchTxt) {
        return channelGroupList.value.filter((x) =>
          x.name.toLowerCase().includes(iptvInfo.searchTxt.toLowerCase())
        );
      } else {
        return channelGroupList.value;
      }
    };

    const resetChannelsEvent = () => {
      iptvInfo.stopFlag = true;
      if (iptvInfo.checkAllChannelsLoading) {
        this.$message.info("部分检测还未完全终止, 请稍等...");
        return;
      }
      this.channelList = [];
      this.iptvList = [];
      iptv
        .clear()
        .then(iptv.bulkAdd(defaultChannels).then(this.updateChannelList()));
    };

    const expandChange = (row, expanded) => {
      const index = iptvInfo.expandedRows.indexOf(row);
      if (expanded && index === -1) {
        iptvInfo.expandedRows.push(row);
      } else if (!expanded && index !== -1) {
        iptvInfo.expandedRows.splice(index, 1);
      }
    };

    const playEvent = (e) => {
      let currChannelGroupId;
      if (e.url) {
        currChannelGroupId = e.id;
      } else {
        let channel = e.channels.filter((c) => c.active)[0];
        if (!channel) {
          ElMessage({
            showClose: true,
            message: "当前频道所有源已全部停用，不可播放！",
            type: "error",
          });
          return;
        }
        currChannelGroupId = channel.channelGroupId;
      }
      video.value.playType = "iptv";
      channelGroupId.value = currChannelGroupId;
      view.value = "Play";
    };

    const selectionCellClick = (selection, row) => {
      if (
        shiftDown.value &&
        iptvInfo.selectionBegin !== "" &&
        selection.includes(row)
      ) {
        iptvInfo.selectionEnd = row.id;
        const start = channelGroupList.value.findIndex(
          (e) =>
            e.id === Math.min(iptvInfo.selectionBegin, iptvInfo.selectionEnd)
        );
        const end = channelGroupList.value.findIndex(
          (e) =>
            e.id === Math.max(iptvInfo.selectionBegin, iptvInfo.selectionEnd)
        );
        const selections = channelGroupList.value.slice(start, end + 1); // 多选时强制不让展开
        nextTick(() => {
          selections.forEach((e) =>
            iptvTableRef.value.toggleRowSelection(e, true)
          );
        });
        iptvInfo.selectionBegin = iptvInfo.selectionEnd = "";
        return;
      }
      if (selection.includes(row)) {
        iptvInfo.selectionBegin = row.id;
      } else {
        iptvInfo.selectionBegin = "";
      }
    };

    const handleSelectionChange = (rows) => {
      iptvInfo.multipleSelection = lodash.cloneDeep(rows);
    };

    const handleSortChange = (column, prop, order) => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info("正在检测, 请勿操作");
        return;
      }
      this.updateDatabase();
    };

    const checkAllChannels = async () => {
      if (iptvInfo.checkAllChannelsLoading) return;
      iptvInfo.checkAllChannelsLoading = true;
      iptvInfo.stopFlag = false;
      iptvInfo.checkProgress = 0;
      this.channelList
        .filter((e) => e.channels.length)
        .forEach((e) => {
          e.status = " ";
          e.hasCheckedNum = 0;
        });
      const uncheckedList = this.iptvList.filter(
        (e) => e.status === undefined || e.status === " "
      ); // 未检测过的优先
      const other = this.iptvList.filter((e) => !uncheckedList.includes(e));
      await this.checkChannelsBySite(uncheckedList);
      await this.checkChannelsBySite(other).then((res) => {
        this.checkAllChannelsLoading = false;
        refreshChannelGroupList();
        if (!iptvInfo.stopFlag)
          ElMessage({
            showClose: true,
            message: "直播频道批量检测已完成！",
            type: "success",
          });
      });
    };

    const removeEvent = async (row) => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }
      try {
        if (row.url) {
          // tree树形控件节点一旦展开，就不再重新加载节点数据
          const ele = channelGroupList.value.find(
            (e) => e.id === row.channelGroupId
          );
          ele.channels.splice(
            ele.channels.findIndex((e) => e.id === row.id),
            1
          );
          if (ele.channels.length) {
            if (ele.channels.length === 1) ele.hasChildren = "0";
            let channelGroup = lodash.cloneDeep(ele);
            channelGroup.channels = JSON.stringify(channelGroup.channels);
            await invoke("save_channel_group", { channelGroup: channelGroup });
          }
        } else {
          await invoke("del_channel_group", { id: row.id });
        }
        refreshChannelGroupList();
      } catch (err) {
        ElMessage.warning("删除频道失败, 错误信息: " + err);
      }
    };

    const checkChannel = (row) => {
      if (row.channels) {
        row.status = " ";
        row.hasCheckedNum = 0;
        row.channels.forEach((e) => checkSingleChannel(e, true));
      } else {
        checkSingleChannel(row);
      }
    };

    const checkSingleChannel = async (channel, force = false) => {
      if (iptvInfo.stopFlag) {
        iptvInfo.checkProgress += 1;
        return;
      }
      const ele = channelGroupList.value.find(
        (e) => e.id === channel.channelGroupId
      );
      if (
        !force &&
        systemConf.value.allowPassWhenIptvCheck &&
        (!channel.isActive || !ele.isActive)
      ) {
        if (!ele.active) {
          ele.status = "跳过";
        } else if (!channel.isActive) {
          channel.status = "跳过";
        }
      } else {
        channel.status = " ";
        const flag = await iptvApi.checkChannel(channel.url);
        if (flag) {
          channel.status = "可用";
        } else {
          channel.status = "失效";
          channel.active = false;
          if (systemConf.value.autocleanWhenIptvCheck) {
            ele.channels.splice(
              ele.channels.findIndex((e) => e.id === channel.id),
              1
            );
            ele.hasCheckedNum--;
          }
        }
      }
      iptvInfo.checkProgress += 1;
      ele.hasCheckedNum++;
      if (ele.hasCheckedNum === ele.channels.length) {
        if (ele.status === " ") {
          ele.status = ele.channels.some((channel) => channel.status === "可用")
            ? "可用"
            : "失效";
          if (ele.status === "失效") ele.active = false;
        }
        let channelGroup = lodash.cloneDeep(ele);
        channelGroup.hasChildren = channelGroup.hasChildren ? "1" : "0";
        channelGroup.channels = JSON.stringify(channelGroup.channels);
        await invoke("save_channel_group", { channelGroup: channelGroup });
        refreshChannelGroupList();
      }
    };

    const sortByLocaleCompare = (a, b) => {
      return a.localeCompare(b, "zh");
    };

    // 批量保存编辑
    const saveBatchEdit = async () => {
      iptvInfo.multipleSelection.forEach((ele) => {
        if (iptvInfo.inputContent) {
          ele.group = iptvInfo.inputContent;
        }
        ele.hasChildren = ele.hasChildren ? "1" : "0";
        ele.channels = JSON.stringify(ele.channels);
        ele.active = iptvInfo.batchIsActive ? "1" : "0";
      });
      await invoke("save_channel_groups", {
        channelGroups: iptvInfo.multipleSelection,
      });
      iptvInfo.multipleSelection = [];
      refreshChannelGroupList();
    };
    
    const removeSelectedChannels = () => {
      this.multipleSelection.forEach(e => channelList.remove(e.id))
      this.$refs.iptvTable.clearFilter()
      this.refreshChannelGroupList()
      this.updateDatabase()
      this.enableBatchEdit = false
    }

    const mergeChannel = () => {
      if (this.inputContent && this.multipleSelection.length) {
        let channels = []
        const id = this.multipleSelection[0].id
        this.multipleSelection.forEach(ele => {
          channels = channels.concat(ele.channels)
          channels.forEach(e => { e.channelID = id })
          channelList.remove(ele.id)
        })
        const mergeChannel = { id: id, name: this.inputContent, isActive: channels.some(c => c.isActive), group: this.determineGroup(this.inputContent), hasChildren: channels.length > 1, channels: channels }
        channelList.add(mergeChannel)
        this.refreshChannelGroupList()
        this.updateDatabase()
      }
    }
    
    const isActiveChangeEvent = async (row) => {
      let channelGroup = row;
      if (row.url) {
        const ele = channelGroupList.value.find(e => e.id === row.channelGroupId);
        channelGroup = ele;
      } else {
        if (row.channels.length === 1) row.channels[0].active = row.active
      }
      const channelGroupClone = lodash.cloneDeep(channelGroup);
      channelGroupClone.channels = JSON.stringify(channelGroupClone.channels);
      await invoke("save_channel_group", { channelGroup: channelGroupClone })
    }

    watch(
      () => view.value,
      async () => {
        if (view.value === "IPTV") {
          getAllChannelGroup();
        }
      }
    );

    return {
      iptvInfo,
      channelGroupList,
      channelList,
      exportChannels,
      importChannels,
      importOnlineChannels,
      importLocalChannels,
      channelGroupFilter,
      filteredTableData,
      expandChange,
      playEvent,
      selectionCellClick,
      handleSortChange,
      handleSelectionChange,
      resetChannelsEvent,
      checkAllChannels,
      removeEvent,
      checkChannel,
      checkSingleChannel,
      iptvTableRef,
      sortByLocaleCompare,
      saveBatchEdit,
      Download,
      Upload,
      Refresh,
      RefreshLeft,
      removeSelectedChannels,
      mergeChannel,
      isActiveChangeEvent,
      Search
    };
  },
});
// import Sortable from 'sortablejs'
// export default {
//   data () {
//     return {
//       sortableTable: ''
//     }
//   },
//   watch: {
//     enableBatchEdit () {
//       if (this.checkAllChannelsLoading) {
//         this.$message.info('正在检测, 请勿操作.')
//         this.enableBatchEdit = false
//         return
//       }
//       if (this.enableBatchEdit) {
//         if (this.setting.shiftTooltipLimitTimes === undefined) this.setting.shiftTooltipLimitTimes = 5
//         if (this.setting.shiftTooltipLimitTimes) {
//           this.$message.info('多选时支持shift快捷键')
//           this.setting.shiftTooltipLimitTimes--
//           setting.find().then(res => {
//             res.shiftTooltipLimitTimes = this.setting.shiftTooltipLimitTimes
//             setting.update(res)
//           })
//         }
//         this.$nextTick(() => {
//           this.expandedRows.forEach(e => this.$refs.iptvTable.toggleRowExpansion(e, false))
//         })
//         this.rowDrop()
//       } else {
//         this.sortableTable.destroy()
//       }
//     }
//   },
//   methods: {
//     ...mapMutations(['SET_VIEW', 'SET_DETAIL', 'SET_VIDEO', 'SET_SHARE', 'SET_SETTING']),

//     moveToTopEvent (row) {
//       if (this.checkAllChannelsLoading) {
//         this.$message.info('正在检测, 请勿操作.')
//         return false
//       }
//       // this.channelList.sort(function (x, y) { return (x.name === i.name && x.url === i.url) ? -1 : (y.name === i.name && y.url === i.url) ? 1 : 0 })
//       if (row.channels) {
//         this.channelList.splice(this.channelList.findIndex(e => e.id === row.id), 1)
//         this.channelList.unshift(row)
//         this.updateDatabase()
//       }
//     },
//     rowDrop () {
//       if (this.checkAllChannelsLoading) {
//         this.$message.info('正在检测, 请勿操作.')
//         return false
//       }
//       const tbody = document.getElementById('iptv-table').querySelector('.el-table__body-wrapper tbody')
//       const _this = this
//       this.sortableTable = new Sortable(tbody, {
//         filter: '.el-table__row--level-1', // 禁止children拖动
//         onEnd ({ newIndex, oldIndex }) {
//           const currRow = _this.channelList.splice(oldIndex, 1)[0]
//           _this.channelList.splice(newIndex, 0, currRow)
//           _this.updateDatabase()
//         }
//       })
//     },
//     async checkChannelsBySite (channels) {
//       const siteList = {}
//       channels.forEach(channel => {
//         const site = channel.url.split('/')[2]
//         if (siteList[site]) {
//           siteList[site].push(channel)
//         } else {
//           siteList[site] = [channel]
//         }
//       })
//       await Promise.all(Object.values(siteList).map(site => this.checkSingleSite(site)))
//     },
//     async checkSingleSite (channelArray) {
//       for (const c of channelArray) {
//         if (this.stopFlag) return false
//         await this.checkSingleChannel(c)
//       }
//     },
//   },
// }
</script>
