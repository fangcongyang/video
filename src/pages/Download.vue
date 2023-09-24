<template>
  <div class="listpage" id="download" @contextmenu="onContextMenu($event)">
    <!-- <context-menu
      v-model:show="downloadInfo.contextMenushow"
      :options="downloadInfo.options"
    >
    </context-menu> -->
    <div class="page-body">
      <div class="show-table" id="download-table">
        <el-auto-resizer>
          <template #default="{ height, width }">
            <el-table-v2
              :columns="columns"
              :data="downloadList"
              :width="width"
              :height="height"
              fixed
            />
          </template>
        </el-auto-resizer>
      </div>
    </div>
  </div>
</template>
<script lang="jsx" setup>
import {
  defineComponent,
  reactive,
  watch,
  nextTick,
  ref,
  computed,
  onMounted,
  onBeforeMount,
  onBeforeUnmount,
} from "vue";
import {
  ElMessage,
  ElLoading,
  ElTag,
  ElButton,
  ElProgress,
  ElCheckbox,
} from "element-plus";
import { useCoreStore } from "@/store";
import { useDownloadStore } from "@/store/download";
import { storeToRefs } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { _ } from "lodash";
import { Refresh, DataLine, Search } from "@element-plus/icons-vue";
import Sortable from "sortablejs";
import ContextMenu from "@imengyu/vue3-context-menu";
import { useDark } from "@vueuse/core";

const colors = [
  { color: "#f56c6c", percentage: 20 },
  { color: "#e6a23c", percentage: 40 },
  { color: "#5cb87a", percentage: 60 },
  { color: "#1989fa", percentage: 80 },
  { color: "#6f7ad3", percentage: 100 },
];

const SelectionCell = ({
  value,
  intermediate = false,
  onChange,
}) => {
  return (
    <ElCheckbox
      onChange={onChange}
      modelValue={value}
      indeterminate={intermediate}
    />
  )
}

const columns = [
  {
    key: "movieName",
    title: "影片名称",
    dataKey: "movieName",
    width: 200,
  },
  {
    key: "subTitleName",
    title: "子标题",
    dataKey: "subTitleName",
    width: 100,
    align: "center",
  },
  {
    key: "status",
    title: "状态",
    dataKey: "status",
    cellRenderer: ({ cellData: status }) => {
      switch (status) {
        case "parseSource":
          return <ElTag type="info">解析资源</ElTag>;
        case "downloadSlice":
          return <ElTag type="">下载ts分片</ElTag>;
        case "checkSouce":
          return <ElTag type="">检测完整性</ElTag>;
        case "merger":
          return <ElTag type="">合并视频</ElTag>;
        case "downloadEnd":
          return <ElTag type="success">下载结束</ElTag>;
      }
    },
    width: 150,
    align: "center",
  },
  {
    key: "downloadStatus",
    title: "下载状态",
    dataKey: "downloadStatus",
    cellRenderer: ({ cellData: downloadStatus }) => {
      switch (downloadStatus) {
        case "parseSource":
          return <ElTag type="info">等待下载</ElTag>;
        case "downloading":
          return <ElTag type="info">下载中</ElTag>;
        case "downloadFail":
          return <ElTag type="danger">下载失败</ElTag>;
        case "downloadSuccess":
          return <ElTag type="success">下载成功</ElTag>;
      }
    },
    width: 100,
    align: "center",
  },
  {
    key: "downloadCount",
    title: "下载进度",
    cellRenderer: ({ rowData }) => {
      let percentageNum = rowData.count == rowData.count == 0 ? 0.00 : _.divide(rowData.downloadCount, rowData.count) * 100
      let percentage = _.ceil(percentageNum, 2);
      return (
        <ElProgress
          class=".el-progress--line"
          color={colors}
          stroke-width={16}
          percentage={percentage}
          text-inside={true}
        ></ElProgress>
      );
    },
    width: 200,
    align: "center",
  },
  {
    key: "operations",
    title: "操作",
    cellRenderer: ({ rowData }) => (
      <>
        { rowData.downloadStatus == 'downloadSuccess' ? <ElButton size="small" link onClick={() => playEvent(rowData)}>
          播放
        </ElButton> : ''}
        { rowData.status != 'downloadEnd' ? <ElButton size="small" link onClick={() => retryEvent(rowData)}>
          重试
        </ElButton> : ''}
        <ElButton size="small" link type="danger"  onClick={() => removeEvent(rowData)}>
          删除
        </ElButton>
      </>
    ),
    width: 150,
    align: "center",
  },
];

columns.unshift({
  key: 'selection',
  width: 50,
  cellRenderer: ({ rowData }) => {
    const onChange = (value ) => (rowData.checked = value)
    return <SelectionCell value={rowData.checked} onChange={onChange} />
  },

  headerCellRenderer: () => {
    const onChange = (value) =>
      (data.value = downloadList.value.map((row) => {
        row.checked = value
        return row
      }))
    const allSelected = downloadList.value.every((row) => row.checked)
    const containsChecked = downloadList.value.some((row) => row.checked)

    return (
      <SelectionCell
        value={allSelected}
        intermediate={containsChecked && !allSelected}
        onChange={onChange}
      />
    )
  },
})

const isDark = useDark();

const downloadInfo = reactive({
  batchIsActive: false,
  searchTxt: "",
  expandedRows: [],
  multipleSelection: [],
  contextMenushow: false,
  options: {
    theme: isDark.value ? "mac dark" : "mac",
    iconFontClass: "iconfont",
    zIndex: 3,
    minWidth: 230,
    x: 500,
    y: 200,
  },
});

const coreStore = useCoreStore();
const { view, playInfo } = storeToRefs(coreStore);

const downloadStore = useDownloadStore();
const { getAllDownload, refreshDownloadList } =
  downloadStore;
const { downloadList } = storeToRefs(downloadStore);

const playEvent = (downloadInfo) => {
  playInfo.value.playType = "localMovie";
  playInfo.value.download.downloadId = downloadInfo.id;
  playInfo.value.movie.playMode = "localFile";
  view.value = "Play";
};

const retryEvent = async (downloadInfo) => {
  downloadInfo.downloadStatus = "downloading";
  await invoke("retry_download", {download: downloadInfo});
}

const removeEvent = async (downloadInfo) => {
  await invoke("del_download_info", {download: downloadInfo});
  refreshDownloadList();
}

const onContextMenu = (e) => {
  e.preventDefault();
  downloadInfo.options.x = e.x;
  downloadInfo.options.y = e.y;
  downloadInfo.contextMenushow = true;
  nextTick(() => {
    var elements = document.querySelectorAll(".mx-menu-ghost-host");
    elements.forEach(function (element) {
      // 为每个元素添加事件监听器
      element.addEventListener("contextmenu", function (e) {
        e.preventDefault(); // 阻止默认行为
      });
    });
  });
};

watch(
  () => view.value,
  async () => {
    if (view.value === "Download") {
      refreshDownloadList();
    }
  }
);

onBeforeMount(() => {
  getAllDownload();
});

onMounted(() => {
  // intervalRefreshData();
});

// export default defineComponent({
//   setup() {

//     const iptvTableRef = ref();

//     // 查找电视
//     const filteredTableData = computed(() => {
//       if (iptvInfo.searchTxt) {
//         return channelGroupList.value.filter((x) =>
//           x.name.toLowerCase().includes(iptvInfo.searchTxt.toLowerCase())
//         );
//       } else {
//         return channelGroupList.value;
//       }
//     });

//     const selectionCellClick = (selection, row) => {
//       if (
//         shiftDown.value &&
//         iptvInfo.selectionBegin !== "" &&
//         selection.includes(row)
//       ) {
//         iptvInfo.selectionEnd = row.id;
//         const start = channelGroupList.value.findIndex(
//           (e) =>
//             e.id === Math.min(iptvInfo.selectionBegin, iptvInfo.selectionEnd)
//         );
//         const end = channelGroupList.value.findIndex(
//           (e) =>
//             e.id === Math.max(iptvInfo.selectionBegin, iptvInfo.selectionEnd)
//         );
//         const selections = channelGroupList.value.slice(start, end + 1); // 多选时强制不让展开
//         nextTick(() => {
//           selections.forEach((e) =>
//             iptvTableRef.value.toggleRowSelection(e, true)
//           );
//         });
//         iptvInfo.selectionBegin = iptvInfo.selectionEnd = "";
//         return;
//       }
//       if (selection.includes(row)) {
//         iptvInfo.selectionBegin = row.id;
//       } else {
//         iptvInfo.selectionBegin = "";
//       }
//     };

//     const handleSelectionChange = (rows) => {
//       iptvInfo.multipleSelection = _.cloneDeep(rows);
//     };

//     const handleSortChange = (column, prop, order) => {
//       if (iptvInfo.checkAllChannelsLoading) {
//         ElMessage.info("正在检测, 请勿操作");
//         return;
//       }
//       this.updateDatabase();
//     };

//     const sortByLocaleCompare = (a, b) => {
//       return a.localeCompare(b, "zh");
//     };

//     const removeSelectedChannels = () => {
//       this.multipleSelection.forEach((e) => channelList.remove(e.id));
//       this.$refs.iptvTable.clearFilter();
//       this.refreshChannelGroupList();
//       this.updateDatabase();
//     };

//     const moveToTopEvent = (row) => {
//       if (iptvInfo.checkAllChannelsLoading) {
//         ElMessage.info("正在检测, 请勿操作.");
//         return false;
//       }
//       // this.channelList.sort(function (x, y) { return (x.name === i.name && x.url === i.url) ? -1 : (y.name === i.name && y.url === i.url) ? 1 : 0 })
//       if (row.channels) {
//         this.channelList.splice(
//           this.channelList.findIndex((e) => e.id === row.id),
//           1
//         );
//         this.channelList.unshift(row);
//         this.updateDatabase();
//       }
//     };

//     const rowDrop = () => {
//       if (iptvInfo.checkAllChannelsLoading) {
//         ElMessage.info("正在检测, 请勿操作.");
//         return false;
//       }
//       const tbody = document
//         .getElementById("iptv-table")
//         .querySelector(".el-table__body-wrapper tbody");
//       const _this = this;
//       this.sortableTable = new Sortable(tbody, {
//         filter: ".el-table__row--level-1", // 禁止children拖动
//         onEnd({ newIndex, oldIndex }) {
//           const currRow = _this.channelList.splice(oldIndex, 1)[0];
//           _this.channelList.splice(newIndex, 0, currRow);
//           _this.updateDatabase();
//         },
//       });
//     };

// });
</script>

<style type="scss">
.el-progress--line {
  width: 150px;
}
</style>
