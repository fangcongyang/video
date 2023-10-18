<template>
  <div class="listpage" id="iptv" @contextmenu="onContextMenu($event)">
    <context-menu
      v-model:show="iptvInfo.contextMenushow"
      :options="iptvInfo.options"
    >
      <context-menu-item label="导出" @click="exportChannels()">
        <template #icon>
          <el-icon>
            <Download />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="导入" @click="importChannels()">
        <template #icon>
          <el-icon>
            <Upload />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-sperator />
      <context-menu-item label="检测" @click="checkAllChannels()">
        <template #icon>
          <el-icon>
            <Refresh />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="在线搜索" @click="showIptvOnlineSearch()">
        <template #icon>
          <el-icon>
            <DataLine />
          </el-icon>
        </template>
      </context-menu-item>
      <context-menu-item label="删除" @click="removeSelectedChannels()">
        <template #icon>
          <el-icon>
            <Delete />
          </el-icon>
        </template>
      </context-menu-item>
    </context-menu>
    <div class="page-body">
      <div class="show-table" id="iptv-table">
        <el-table
          ref="iptvTableRef"
          size="small"
          fit
          height="100%"
          row-key="id"
          :data="filteredTableData"
          lazy
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
          @sort-change="handleSortChange"
        >
          <el-table-column type="selection">
          </el-table-column>
          <el-table-column
            default-sort="ascending"
            prop="channel_name"
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
            width="100"
            sortable
            :sort-method="
              (a, b) => sortByLocaleCompare(a.channel_group_name, b.channel_group_name)
            "
            prop="channel_group_name"
            label="分组"
            :filters="channelGroupFilter"
            :filter-method="(value, row) => value === row.channel_group_name"
            filter-placement="bottom-end"
          >
            <template #default="scope">
              <el-button link>{{ scope.row.channel_group_name }}</el-button>
            </template>
          </el-table-column>
          <el-table-column
            prop="active"
            :filter-method="(value, row) => value === row.channel_active"
            label="线路"
          >
            <template #default="scope">
              <el-select v-model="scope.row.channel_active" placeholder="选择线路" 
                @change="channelActiveChangeEvent(scope.row)">
                <el-option
                  v-for="item in scope.row.channels"
                  :key="item.id"
                  :label="item.label"
                  :value="item.id"
                >
                  <span :style="item.status == '可用' ? {float: 'left'} : {float: 'left', color: 'red'}">{{ item.label }}</span>
                  <div style="font-size: 12Px; height: 100%; display: flex; align-items: center; justify-content: flex-end;" 
                    @click.stop="removeChannelEvent(item, scope.row)">
                    <Close style="float: right; width: 14Px; height: 14Px;"/>
                  </div>
                </el-option>
              </el-select>
            </template>
          </el-table-column>
          <el-table-column
            label="状态"
            sortable
            :sort-by="['channel_status']"
            width="100"
          >
            <template #default="scope">
              <span v-if="scope.row.channel_status === ' '">
                <i class="el-icon-loading"></i>
                检测中...
              </span>
              <span v-else>{{ scope.row.channel_status }}</span>
            </template>
          </el-table-column>
          <el-table-column label="操作" align="right" :width="240">
            <template #header>
              <span>{{
                  `频道总数:${channelGroupList.length}/资源总数:${channelList.length}`
              }}</span>
            </template>
            <template #default="scope">
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
    <el-drawer
      v-model="iptvInfo.showOnlineSearch"
      title="iptv在线搜索"
      size="400"
      direction="rtl"
    >
      <el-input v-model="iptvInfo.onlineSearchTxt" @keyup.enter="iptvOnlineSearch" placeholder="请输入iptv名称" />
      <el-table :data="iptvInfo.searchIptvList" style="width: 100%">
        <el-table-column prop="iptvName" show-overflow-tooltip label="IPTV名称" />
        <el-table-column prop="iptvUrl" label="IPTV播放地址" show-overflow-tooltip width="120" />
        <el-table-column label="操作">
          <template #default="scope">
            <el-button size="small" @click="addChnanel(scope.row)" link>
              保存
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-drawer>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  watch,
  nextTick,
  ref,
  computed,
  onMounted,
} from "vue";
import { ElMessage, ElLoading } from "element-plus";
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
import { _ } from "lodash";
import { Refresh, DataLine, Search } from "@element-plus/icons-vue";
import Sortable from 'sortablejs'
import ContextMenu from '@imengyu/vue3-context-menu';
import { useDark } from "@vueuse/core";
import {
  Close
} from "@element-plus/icons-vue";

export default defineComponent({
  name: "Iptv",
  components: {
  },
  setup() {
    const isDark = useDark();

    const iptvInfo = reactive({
      checkAllChannelsLoading: false,
      importChannelVisible: false,
      parseM3uUrl: "",
      searchTxt: "",
      checkProgress: 0,
      stopFlag: false,
      selectionBegin: "",
      selectionEnd: "",
      multipleSelection: [],
      showOnlineSearch: false,
      searchIptvList: [],
      contextMenushow: false,
      options: {
        theme: isDark.value ? 'mac dark' : 'mac',
        iconFontClass: "iconfont",
        zIndex: 3,
        minWidth: 230,
        x: 500,
        y: 200,
      },
    });
    const coreStore = useCoreStore();
    const { view, systemConf, shiftDown, playInfo } = storeToRefs(coreStore);

    const iptvStore = useIptvStore();
    const { getAllChannelGroup, refreshChannelGroupList } = iptvStore;
    const {
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
          channelGroupList.value.forEach((channelGroup) => {
            channelGroup.channels.forEach((e) => {
              m3uGenerate.appendM3u(-1, channelGroup.channel_name, e.url);
            });
          })
          await writeTextFile(filePath, m3uGenerate.toString());
        } else {
          if (!filePath.endsWith(".json")) filePath += ".json";
          const arr = [...channelList.value];
          const str = JSON.stringify(arr, null, 2);
          await writeTextFile(filePath, str);
        }
        ElMessage.success("保存成功");
      }
    };

    const importChannels = () => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info("正在检测, 请勿操作.");
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
              name: j[0],
              url: j[1],
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
                  name: ele.name,
                  url: url,
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
                  name: j[0],
                  url: j[1],
                  status: "可用",
                };
                docs.push(doc);
              }
            }
            // 获取url不重复的列表
            const uniqueList = docs.filter(
              (item) => !channelMap.value[item.url]
            );
            addChannelList(uniqueList);
            ElMessage.success("导入成功");
          } catch (error) {
            console.error(error);
            ElMessage.warning("导入失败");
          }
        }
      });
    };

    const addChnanel = (row) => {
      if (channelMap.value[row.iptvUrl]) {
        ElMessage.warning("当前资源已存在");
        return;
      }
      const doc = {
        name: row.iptvName,
        url: row.iptvUrl,
        status: "可用",
      };
      addChannelList([doc]);
    }

    const addChannelList = async (uniqueList) => {
      const uniqueChannelName = {};
      for (let item of uniqueList) {
        let channelName = item.name
          .trim()
          .replace(/[- ]?(1080p|蓝光|超清|高清|标清|hd|cq|4k)(\d{1,2})?$/i, "");
        if (channelName.match(/cctv/i))
          channelName = channelName.replace("-", "");
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
          if (!channelGroup) {
            channelGroup = {
              channel_name: e,
              channel_status: "可用",
              channel_group_name: channelGroupName,
              channels: [],
            };
            channelGroupMap.value[e + channelGroupName] = channelGroup;
          }
          channelGroup.channels = channelGroup.channels.concat(
            uniqueChannelName[e]
          );
          let id = 1;
          channelGroup.channels.forEach((channel) =>{
            channel.id = id.toString();
            channel.label = "线路" + channel.id;
            id += 1;
          })
          channelGroup.channel_active = channelGroup.channels[0].id;
          channelGroup.channels = JSON.stringify(channelGroup.channels);
          return channelGroup;
        });
        
        await invoke("save_channel_groups", { channelGroups: channelGroupList });
        refreshChannelGroupList();
        ElMessage.success("添加资源成功");
      }
      iptvInfo.importChannelVisible = false;
    };

    // 查找电视
    const filteredTableData = computed(() => {
      if (iptvInfo.searchTxt) {
        return channelGroupList.value.filter((x) =>
          x.channel_name.toLowerCase().includes(iptvInfo.searchTxt.toLowerCase())
        );
      } else {
        return channelGroupList.value;
      }
    });

    const iptvOnlineSearch = () => {
      const loadingInstance = ElLoading.service({ fullscreen: true })
      iptvApi.iptvOnlineSearch(iptvInfo.onlineSearchTxt).then(res => {
        iptvInfo.searchIptvList = res;
        nextTick(() => {
          loadingInstance.close()
        })
      }).catch((err) => {
        loadingInstance.close()
      })
    };

    const showIptvOnlineSearch = () => {
      iptvInfo.searchIptvList = [];
      iptvInfo.showOnlineSearch = true;
    };

    const playEvent = (e) => {
      playInfo.value.playType = "iptv";
      playInfo.value.iptv.channelGroupId = e.id;
      playInfo.value.iptv.channelActive = e.channel_active;
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
      iptvInfo.multipleSelection = _.cloneDeep(rows);
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
      channelList.value
        .filter((e) => e.channels.length)
        .forEach((e) => {
          e.status = " ";
          e.hasCheckedNum = 0;
        });
      const uncheckedList = channelList.value.filter(
        (e) => e.status === undefined || e.status === " "
      ); // 未检测过的优先
      const other = channelList.value.filter((e) => !uncheckedList.includes(e));
      await this.checkChannelsBySite(uncheckedList);
      await this.checkChannelsBySite(other).then((res) => {
        this.checkAllChannelsLoading = false;
        refreshChannelGroupList();
        if (!iptvInfo.stopFlag)
          ElMessage.success("直播频道批量检测已完成！");
      });
    };

    const removeEvent = async (row) => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }

      await invoke("del_channel_group", { id: row.id });
      refreshChannelGroupList();
    };

    const removeChannelEvent = async (channel, row) => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }
      let channelGroup = _.cloneDeep(row);
      _.remove(channelGroup.channels, (cl) => {
        return cl.id == channel.id;
      });
      
      if (channelGroup.channels.length) {
        channelGroup.channels = JSON.stringify(channelGroup.channels);
        await invoke("save_channel_group", { channelGroupInfo: channelGroup });
      } else {
        await invoke("del_channel_group", { channelGroupId: channelGroup.id });
      }
      refreshChannelGroupList();
    }

    const checkChannel = async (row) => {
      if (row.channels) {
        row.channel_status = " ";
        row.hasCheckedNum = 0;
        await Promise.all(row.channels.map((e) => checkSingleChannel(e, true)))
      }
      ElMessage.success("检测完成");
    };

    const checkSingleChannel = async (channel, force = false) => {
      if (iptvInfo.stopFlag) {
        iptvInfo.checkProgress += 1;
        return;
      }
      const ele = channelGroupList.value.find(
        (e) => e.id === channel.channelGroupId
      );
      
      const flag = await iptvApi.checkChannel(channel.url);
      if (flag) {
        channel.status = "可用";
      } else {
        channel.status = "失效";
        if (systemConf.value.autocleanWhenIptvCheck) {
          ele.channels.splice(
            ele.channels.findIndex((e) => e.id === channel.id),
            1
          );
          if (ele.channel_active == channel.id) {
            ele.channel_active = ele.channels.length > 0 ? ele.channels[0] : ""
          }
          ele.hasCheckedNum--;
        }
      }

      iptvInfo.checkProgress += 1;
      ele.hasCheckedNum++;
      if (ele.hasCheckedNum === ele.channels.length) {
        if (ele.channel_status === " ") {
          ele.channel_status = ele.channels.some((channel) => channel.status === "可用")
            ? "可用"
            : "失效";
        }
        let channelGroup = _.cloneDeep(ele);
        channelGroup.channels = JSON.stringify(channelGroup.channels);
        await invoke("save_channel_group", { channelGroupInfo: channelGroup });
        refreshChannelGroupList();
      }
    };

    const sortByLocaleCompare = (a, b) => {
      return a.localeCompare(b, "zh");
    };
    
    const removeSelectedChannels = async () => {
      let channelGroupIds = iptvInfo.multipleSelection.map((item) => {
        return item.id;
      })
      await invoke("del_channel_groups", { channelGroupIds: channelGroupIds });
      iptvTableRef.value.clearFilter()
      refreshChannelGroupList()
    }
    
    const channelActiveChangeEvent = async (row) => {
      const channelGroupClone = _.cloneDeep(row);
      channelGroupClone.channels = JSON.stringify(channelGroupClone.channels);
      await invoke("save_channel_group", { channelGroupInfo: channelGroupClone })
      refreshChannelGroupList()
    }
    
    const checkChannelsBySite = async () => {
      const siteList = {}
      channelList.value.forEach(channel => {
        const site = channel.url.split('/')[2]
        if (siteList[site]) {
          siteList[site].push(channel)
        } else {
          siteList[site] = [channel]
        }
      })
      await Promise.all(Object.values(siteList).map(site => checkSingleSite(site)))
    }
    
    const checkSingleSite = async (channelArray) => {
      for (const c of channelArray) {
        if (iptvInfo.stopFlag) return false
        await checkSingleChannel(c)
      }
    }

    const rowDrop = () => {
      if (iptvInfo.checkAllChannelsLoading) {
        ElMessage.info('正在检测, 请勿操作.')
        return false
      }
      const tbody = document.getElementById('iptv-table').querySelector('.el-table__body-wrapper tbody')
      
      Sortable.create(tbody, {
        async onEnd({ newIndex, oldIndex }) {
          console.log(newIndex, oldIndex)
          let currChannelGroup = filteredTableData.value[oldIndex];
          // 前第一个数据的位置
          let prevPosition;
          // 后第一个数据的位置
          let nextPosition;
          // 上移
          if (newIndex < oldIndex) {
            prevPosition = newIndex == 0 ? filteredTableData.value[newIndex].position + 10 : filteredTableData.value[newIndex].position;
            nextPosition = filteredTableData.value[newIndex].position;
          } else { // 下移
            prevPosition = filteredTableData.value[newIndex].position;
            nextPosition =
              newIndex == filteredTableData.value.length - 1
                ? 0
                : filteredTableData.value[newIndex].position;
          }
          currChannelGroup.position = (prevPosition + nextPosition) / 2;
          const channelGroupClone = _.cloneDeep(currChannelGroup);
          channelGroupClone.channels = JSON.stringify(channelGroupClone.channels);
          await invoke("save_channel_group", { channelGroupInfo: channelGroupClone })
          refreshChannelGroupList()
        },
      });
    }
    
    const onContextMenu = (e ) => {
      e.preventDefault();
      iptvInfo.options.x = e.x;
      iptvInfo.options.y = e.y;
      iptvInfo.contextMenushow = true;
      nextTick(() => {
        var elements = document.querySelectorAll(".mx-menu-ghost-host");
        elements.forEach(function (element) {
          // 为每个元素添加事件监听器
          element.addEventListener("contextmenu", function (e) {
            e.preventDefault(); // 阻止默认行为
          });
        });
      });
    }

    watch(
      () => view.value,
      async () => {
        if (view.value === "IPTV") {
          getAllChannelGroup();
        }
      }
    );

    onMounted(() => {
      rowDrop();
    })

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
      playEvent,
      selectionCellClick,
      handleSortChange,
      handleSelectionChange,
      iptvOnlineSearch,
      checkAllChannels,
      removeEvent,
      checkChannel,
      checkSingleChannel,
      iptvTableRef,
      sortByLocaleCompare,
      showIptvOnlineSearch,
      removeSelectedChannels,
      channelActiveChangeEvent,
      Search,
      checkChannelsBySite,
      addChnanel,
      onContextMenu,
      Close,
      removeChannelEvent,
    };
  },
});
</script>
