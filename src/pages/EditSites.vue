<template>
  <div class="listpage" id="sites">
    <div class="listpage-header" v-show="!siteInfo.enableBatchEdit">
      <el-switch v-model="siteInfo.enableBatchEdit" active-text="批处理分组"
        >></el-switch
      >
      <el-button @click="addSite" :icon="DocumentAdd">新增</el-button>
      <el-button
        @click="exportSites"
        :icon="Upload"
        title="导出全部，自动添加扩展名"
        >导出</el-button
      >
      <el-button
        @click="importSites"
        :icon="Download"
        title="支持同时导入多个文件"
        >导入</el-button
      >
      <el-button @click="checkAllSite" :icon="Refresh" :loading="siteInfo.checkAllSitesLoading" title="可在后台运行">
        检测{{ siteInfo.checkAllSitesLoading ? siteInfo.checkProgress + '/' + siteList.length : '' }}
      </el-button>
      <el-button @click="resetSitesEvent" :icon="RefreshLeft">重置</el-button> 
    </div>
    <div class="listpage-header" v-show="siteInfo.enableBatchEdit">
      <el-switch
        v-model="siteInfo.enableBatchEdit"
        active-text="批处理分组"
      ></el-switch>
      <el-input 
        style="width: 200Px;"
        size="small" placeholder="新组名" v-model="siteInfo.batchGroupName"></el-input>
      <el-switch v-model="siteInfo.batchIsActive" active-text="启用"></el-switch>
      <el-button type="primary" :icon="Edit" @click.stop="saveBatchEdit" title="输入框组名为空时仅保存开关状态">保存分组与开关状态</el-button>
      <el-button @click="removeSelectedSites" :icon="DeleteFilled">删除</el-button>
    </div>
    <div class="listpage-body" id="sites-body">
      <div class="show-table" id="sites-table">
        <el-table
          size="small"
          fit
          height="100%"
          row-key="id"
          ref="editSitesTableRef"
          :data="siteList"
          @select="selectionCellClick"
          @selection-change="handleSelectionChange"
          @sort-change="handleSortChange"
        >
			    <el-table-column label="ID" prop="id" width="70px" align="center" />
            <el-table-column type="selection" v-if="siteInfo.enableBatchEdit">
          </el-table-column>
          <el-table-column class="siteTableName" prop="site_name" label="资源名" width="200" > </el-table-column>
          <el-table-column
            prop="is_active"
            :filters="[
              { text: '启用', value: '1' },
              { text: '停用', value: '0' },
            ]"
            :filter-method="(value, row) => value === row.is_active"
            label="启用"
          >
            <template #default="scope">
              <el-switch
                v-model="scope.row.is_active"
                active-value="1"
                inactive-value="0"
                @click.native.stop="propChangeEvent(scope.row)"
              >
              </el-switch>
            </template>
          </el-table-column>
          <el-table-column
            prop="site_group"
            label="分组"
            :filters="getFilters"
            :filter-method="(value, row) => value === row.site_group"
            filter-placement="bottom-end"
          >
          </el-table-column>
          <el-table-column
            label="状态"
            sortable
            :sort-by="['status']"
          >
            <template #default="scope">
              <span v-if="scope.row.status === ' '">
                <i class="el-icon-loading"></i>
                检测中...
              </span>
              <span v-else>{{ scope.row.status }}</span>
            </template>
          </el-table-column>
          <el-table-column
            label="操作"
            header-align="center"
            align="right"
            :width="
              siteList.every((site) => site.status) &&
              !siteInfo.checkAllSitesLoading
                ? 240
                : 200
            "
          >
            <template #default="scope">
              <el-button @click.stop="moveToTopEvent(scope.row)" link
                >置顶</el-button
              >
              <el-button @click.stop="editSite(scope.row)" link>编辑</el-button>
              <el-button v-if="siteList.every(site => site.status)" v-show="!siteInfo.checkAllSitesLoading" @click.stop="checkSingleSite(scope.row)"
                link>检测</el-button>
              <el-button @click.stop="removeEvent(scope.row)" link>删除</el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>
    
    <!-- 编辑页面 -->
    <div>
      <el-dialog v-model="siteInfo.editSiteDialogVisible" v-if='siteInfo.editSiteDialogVisible' :title="siteInfo.dialogType==='edit'?'编辑源':'新增源'" 
        :append-to-body="true" @close="closeDialog">
        <el-form :model="site" ref='siteRef' label-width="75px" label-position="left" :rules="siteInfo.rules">
          <el-form-item label="源站名" prop='site_name'>
            <el-input v-model="site.site_name" placeholder="请输入源站名" />
          </el-form-item>
          <el-form-item label="API接口" prop='api'>
            <el-input v-model="site.api" :autosize="{ minRows: 2, maxRows: 4}" type="textarea" placeholder="请输入API接口地址"/>
          </el-form-item>
          <el-form-item label="下载接口" prop='download'>
            <el-input v-model="site.download" :autosize="{ minRows: 2, maxRows: 4}" type="textarea" placeholder="请输入Download接口地址，可以空着"/>
          </el-form-item>
          <el-form-item label="分组" prop='site_group'>
            <el-select v-model="site.site_group" allow-create filterable default-first-option placeholder="请输入分组">
              <el-option v-for="item in siteInfo.siteGroup" :key="item" :label="item" :value="item"></el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="源站标识" prop='site_key'>
            <el-input v-model="site.site_key" placeholder="请输入源站标识，如果为空，系统则自动生成" />
          </el-form-item>
        </el-form>
        <span slot="footer" class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="primary" @click="addOrEditSite">保存</el-button>
        </span>
      </el-dialog>
    </div>
  </div>
</template>
<script>
import {
  defineComponent,
  reactive,
  ref,
  onMounted,
  watch,
  computed,
  onBeforeMount,
  nextTick,
} from "vue";
import { useCoreStore } from "@/store";
import { useMovieStore } from "@/store/movie";
import { storeToRefs } from "pinia";
import { ElMessage } from "element-plus";
import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import moviesApi from "@/api/movies";
import Sortable from 'sortablejs';
import {
  DocumentAdd,
  Upload,
  Download,
  Refresh,
  RefreshLeft,
  Edit,
  DeleteFilled
} from "@element-plus/icons-vue";

export default defineComponent({
  name: "editSites",
  setup() {
    const coreStore = useCoreStore();
    const { systemConf, shiftDown } = storeToRefs(coreStore);

    const movieStore = useMovieStore();
    const {
      getAllSite,
      refreshSiteList,
    } = movieStore;
    const {
      siteList,
    } = storeToRefs(movieStore);

    const editSitesTableRef = ref();

    const siteInfo = reactive({
      checkProgress: 0,
      stopFlag: false,
      enableBatchEdit: false,
      checkAllSitesLoading: false,
      batchGroupName: '',
      batchIsActive: true,
      selectionBegin: "",
      selectionEnd: "",
      multipleSelection: [],
      dialogType: "new",
      editOldkey: "",
      editSiteDialogVisible: false,
      siteGroup: [],
      rules: {
        name: [
          { required: true, message: '源站名不能为空', trigger: 'blur' }
        ],
        api: [
          { required: true, message: 'API地址不能为空', trigger: 'blur' }
        ]
      },
    });

    const site = ref({
      key: "",
      name: "",
      api: "",
      download: "",
      group: "",
      isActive: "1",
    });

    const selectionCellClick = (selection, row) => {
      if (
        shiftDown.value &&
        siteInfo.selectionBegin !== "" &&
        selection.includes(row)
      ) {
        siteInfo.selectionEnd = row.id;
        const start = siteList.value.findIndex(
          (e) =>
            e.id === Math.min(siteInfo.selectionBegin, siteInfo.selectionEnd)
        );
        const end = siteList.value.findIndex(
          (e) =>
            e.id === Math.max(siteInfo.selectionBegin, siteInfo.selectionEnd)
        );
        const selections = siteList.value.slice(start, end + 1);
        nextTick(() => {
          selections.forEach((e) =>
            editSitesTableRef.value.toggleRowSelection(e, true)
          );
        });
        siteInfo.selectionBegin = siteInfo.selectionEnd = "";
        return;
      }
      if (selection.includes(row)) {
        siteInfo.selectionBegin = row.id;
      } else {
        siteInfo.selectionBegin = "";
      }
    };

    const addSite = () => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }
      getSitesGroup();
      siteInfo.dialogType = "new";
      siteInfo.editSiteDialogVisible = true;
      site.value = {
        key: "",
        name: "",
        api: "",
        download: "",
        group: "",
        isActive: "1",
        inReverseOrder: "1"
      };
    };

    const getSitesGroup = () => {
      const arr = [];
      for (const i of siteList.value) {
        if (arr.indexOf(i.group) < 0) {
          arr.push(i.group);
        }
      }
      siteInfo.siteGroup = arr;
    };

    const exportSites = async () => {
      let filePath = await save({
        filters: [{ name: "JSON file", extensions: ["json"] }],
      });
      if (filePath !== null) {
        if (!filePath.endsWith(".json")) filePath += ".json";
        const arr = [...siteList.value];
        const str = JSON.stringify(arr, null, 2);
        await writeTextFile(filePath, str);
        ElMessage.success("已保存成功");
      }
    };

    const importSites = async () => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }

      const selectedFiles = await open({
        multiple: true,
        filters: [{ name: "支持的文件格式", extensions: ["json", "txt"] }],
      });
      let filePaths = [];
      if (Array.isArray(selectedFiles)) {
        filePaths = filePaths.concat(selectedFiles);
      } else if (selectedFiles != null) {
        filePaths.push(selectedFiles);
      }
      filePaths.forEach(async (file) => {
        const str = await readTextFile(file);
        const json = JSON.parse(str);
        json.forEach((ele) => {
          if (
            ele.api &&
            siteList.value.filter((x) =>  x.key === ele.key && x.name === ele.name && x.api === ele.api).length === 0
          ) {
            // 不含该key 同时也不含名字和url一样的
            if (ele.isActive === undefined) {
              ele.isActive = '1';
            }
            if (ele.group === undefined) {
              ele.group = "导入";
            }
            if (ele.inReverseOrder === undefined) {
              ele.inReverseOrder = "1";
            }
          }
        });
        await invoke("insert_sites", { sites: json });
        refreshSiteList();
        ElMessage.success("导入成功");
      });
    };

    const handleSelectionChange = (rows) => {
      siteInfo.multipleSelection = rows;
    };

    const handleSortChange = (column, prop, order) => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }
      // this.updateDatabase(this.sites);
    };

    const moveToTopEvent = (i) => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info("正在检测, 请勿操作.");
        return false;
      }
      siteList.value.sort(function (x, y) {
        return x.key === i.key ? -1 : y.key === i.key ? 1 : 0;
      });
    };
    
    const editSite = (row) => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info('正在检测, 请勿操作.')
        return false
      }
      getSitesGroup()
      siteInfo.dialogType = 'edit'
      siteInfo.editOldkey = row.key;
      siteInfo.editSiteDialogVisible = true
      site.value = row
    }

    const checkSingleSite = async (row) => {
      row.status = ' '
      if (siteInfo.stopFlag) {
        siteInfo.checkProgress += 1
        return row.status
      }
      const flag = await moviesApi.siteCheck(row)
      siteInfo.checkProgress += 1
      if (flag) {
        row.status = '可用'
        row.isActive = '1'
      } else {
        row.status = '失效'
        row.isActive = '0'
      }
      await invoke("save_site", { site: row });
      refreshSiteList();
      return row.status
    }
    
    const removeEvent = async (e) => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info('正在检测, 请勿操作.')
        return false
      }
      await invoke("del_site", { id: e.id });
      refreshSiteList();
    }

    const propChangeEvent = async (row) => {
      await invoke("save_site", { site: row });
    }
    
    const resetSitesEvent = async () => {
      await invoke("reset_site", {});
      ElMessage.success('重置源成功')
    }
    
    const checkAllSite = async () => {
      if (siteInfo.checkAllSitesLoading) return
      siteInfo.checkAllSitesLoading = true
      siteInfo.stopFlag = false
      siteInfo.checkProgress = 0
      const uncheckedList = this.sites.filter(e => e.status === undefined || e.status === ' ') // 未检测过的优先
      const other = this.sites.filter(e => !uncheckedList.includes(e))
      await Promise.all(uncheckedList.map(site => checkSingleSite(site)))
      await Promise.all(other.map(site => checkSingleSite(site))).then(res => {
        siteInfo.checkAllSitesLoading = false
        if (!siteInfo.stopFlag) ElMessage.success('视频点播源站批量检测已完成！')
      })
    }

    const getFilters = computed(() => {
      const groups = [...new Set(siteList.value.map(site => site.group))]
      const filters = []
      groups.forEach(g => {
        const doc = {
          text: g,
          value: g
        }
        filters.push(doc)
      })
      return filters
    })
    
    const closeDialog = () => {
      siteInfo.editSiteDialogVisible = false
    }

    const saveBatchEdit = () => {
      siteInfo.multipleSelection.forEach(async (ele) => {
        if (siteInfo.batchGroupName) {
          ele.group = siteInfo.batchGroupName
        }
        ele.isActive = siteInfo.batchIsActive;
        await invoke("save_site", { site: row });
      })
      refreshSiteList();
    }
    
    const removeSelectedSites = () => {
      siteInfo.multipleSelection.forEach(async (e) => {
        await invoke("del_site", { id: e.id });
      });
      editSitesTableRef.value.clearFilter()
      refreshSiteList();
      siteInfo.enableBatchEdit = false
    }
    
    const addOrEditSite = async () => {
      if (!site.value.name || !site.value.api) {
        ElMessage.error('名称和API接口不能为空。')
        return false
      }
      if (!checkSiteKey()) {
        return false
      }
      
      const doc = {
        id: siteInfo.dialogType === 'edit' ? site.value.id : 0,
        key: site.value.key,
        name: site.value.name,
        api: site.value.api,
        download: site.value.download,
        group: site.value.group,
        isActive: site.value.isActive,
        status: '可用',
        position: siteInfo.dialogType === 'edit' ? site.value.position : 0,
        inReverseOrder: site.value.inReverseOrder
      }
      await invoke("save_site", { site: doc });
      site.value = {
        key: '',
        name: '',
        api: '',
        download: '',
        group: '',
        isActive: '',
        status: '',
        position: 0.0,
        inReverseOrder: ''
      }
      
      siteInfo.editSiteDialogVisible = false
      siteInfo.editOldkey = ''
      refreshSiteList();
    }
    
    const checkSiteKey = () => {
      if (siteInfo.dialogType === 'edit' && siteInfo.editOldkey === site.value.key) {
        return true
      } else {
        for (const i of siteList.value) {
          if (i.key === site.value.key) {
            ElMessage.warning(`源站标识: ${i.key} 已存在, 请勿重复填写.`)
            return false
          }
        }
        return true
      }
    }
    
    const rowDrop = () => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info('正在检测, 请勿操作.')
        return false
      }
      const tbody = document.getElementById('sites-table').querySelector('.el-table__body tbody');
      Sortable.create(tbody, {
        group: 'shared',
        animation: 150,
        async onEnd ({ newIndex, oldIndex }) {
          const currSite = siteList.value.splice(oldIndex, 1)[0];
          // 前第一个数据的位置
          let prevPosition;
          // 后第一个数据的位置
          let nextPosition;
          if (newIndex > oldIndex) {
            prevPosition = siteList.value[newIndex].position;
            nextPosition = newIndex == starInfo.starFilterList.length - 1 ? 0 : siteList.value[newIndex + 1].position;
          } else {
            prevPosition = newIndex == 0 ? siteList.value[newIndex].position + 20 : siteList.value[newIndex - 1].position;
            nextPosition = siteList.value[newIndex].position;
          }
          currSite.position = (prevPosition + nextPosition) / 2;
          await invoke("save_site", { site: currSite });
          refreshSiteList();
        }
      })
    }

    watch(() => siteInfo.enableBatchEdit,
    () => {
      if (siteInfo.checkAllSitesLoading) {
        ElMessage.info('正在检测, 请勿操作.')
        siteInfo.enableBatchEdit = false
      }
    })

    onBeforeMount(() => {
      getAllSite();
    });

    onMounted(() => {
      getAllSite().then(() => {
        rowDrop();
      })
    })

    return {
      siteList,
      siteInfo,
      moveToTopEvent,
      selectionCellClick,
      editSitesTableRef,
      handleSelectionChange,
      handleSortChange,
      DocumentAdd,
      Upload,
      Download,
      Refresh,
      RefreshLeft,
      Edit,
      DeleteFilled,
      site,
      addSite,
      exportSites,
      importSites,
      editSite,
      removeEvent,
      propChangeEvent,
      systemConf,
      getFilters,
      resetSitesEvent,
      checkAllSite,
      closeDialog,
      saveBatchEdit,
      removeSelectedSites,
      addOrEditSite,
      checkSingleSite,
    };
  },
});
</script>
