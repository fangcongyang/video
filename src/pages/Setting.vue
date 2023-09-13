<template>
  <div class="setting">
    <div class="setting-box vop-scroll">
      <div class="logo"><img src="@/assets/image/logo.png" alt="" /></div>
      <div class="info">
        <a @click="linkOpen('https://github.com/fangcongyang/video')"
          >Github</a
        >
        <a
          @click="
            linkOpen('https://github.com/fangcongyang/video/discussions/1')
          "
          >软件完全免费，如遇收费，请立即给差评并退费！</a
        >
        <!-- <a style="color:#38dd77" @click="openUpdate()" v-show="update.find" >最新版本v{{update.version}}</a> -->
      </div>
      <div class="setting-item">
        <div class="title">快捷键</div>
        <div class="setting-item-box">
          <div class="vop-select" @mouseleave="settingInfo.shortcut = false">
            <div class="vs-placeholder" @click="settingInfo.shortcut = true">
              <span>快捷键</span>
            </div>
            <div class="vs-options" v-show="settingInfo.shortcut">
              <ul class="vop-scroll">
                <li
                  :class="systemConf.shortcut === true ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  开启
                </li>
                <li
                  :class="systemConf.shortcut === false ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  关闭
                </li>
              </ul>
            </div>
          </div>
          <div class="vop-select">
            <div class="vs-placeholder vs-noAfter" @click="expShortcut">
              <span>导出</span>
            </div>
          </div>
          <div class="vop-select">
            <div class="vs-placeholder vs-noAfter" @click="resetShortcut">
              <span>重置</span>
            </div>
          </div>
        </div>
      </div><div class="setting-item">
        <div class="title">
          影片配置
        </div>
        <div class="setting-item-box">
          <div class="vop-select" @mouseleave="settingInfo.excludeRootClasses = false">
            <div class="vs-placeholder" @click="settingInfo.excludeRootClasses = true">
              <span>排除主分类</span>
            </div>
            <div class="vs-options" v-show="settingInfo.excludeRootClasses">
              <ul class="vop-scroll">
                <li
                  :class="systemConf.excludeRootClasses === true ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  开启
                </li>
                <li
                  :class="systemConf.excludeRootClasses === false ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  关闭
                </li>
              </ul>
            </div>
          </div>
          <div class="vop-select" @mouseleave="settingInfo.excludeR18Films = false">
            <div class="vs-placeholder" @click="settingInfo.excludeR18Films = true">
              <span>排除18禁</span>
            </div>
            <div class="vs-options" v-show="settingInfo.excludeR18Films">
              <ul class="vop-scroll">
                <li
                  :class="systemConf.excludeR18Films === true ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  开启
                </li>
                <li
                  :class="systemConf.excludeR18Films === false ? 'active' : ''"
                  @click="updateSystemConf"
                >
                  关闭
                </li>
              </ul>
            </div>
          </div>
          <div class="vop-array-input">
            <span>主分类</span><el-input v-model="settingInfo.rootClass" placeholder="输入后按回车确认" class="vop-input"
              allow-clear 
              @keyup.enter="addRootClassFilter()" />
            <div class="tags">
              <div v-for="(o,index) in systemConf.rootClassFilter" :key="index" class="tag">
                {{ o }}<el-icon class="icon" style="margin-left: 4px;font-size: 12px;" @click="removeRootClassFilter(o)"><Close /></el-icon>
              </div>
            </div>
          </div>
          <div class="vop-array-input">
            <span>R18禁分类</span><el-input v-model="settingInfo.r18Class" placeholder="输入后按回车确认" class="vop-input"
              allow-clear 
              @keyup.enter="addR18ClassFilter()" />
            <div class="tags">
              <div v-for="(o,index) in systemConf.r18ClassFilter" :key="index" class="tag">
                {{ o }}<el-icon class="icon" style="margin-left: 4px;font-size: 12px;" @click="removeR18ClassFilter(o)"><Close /></el-icon>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="setting-item">
        <div class="title">缓存</div>
        <div class="setting-item-box">
          <div class="vop-select">
            <div
              class="vs-placeholder vs-noAfter"
              title="清理缓存后图片资源需重新下载，不建议清理，软件会根据磁盘空间动态管理缓存大小"
              @click="clearCache"
            >
              <span>清理缓存</span>
            </div>
          </div>
        </div>
      </div>
      <div class="setting-item">
        <div class="title">定位时间设置</div>
        <div class="setting-item-box">
          <div class="vop-input">
            左/右方向键:
            <el-input-number
              style="width: 100px"
              controls-position="right"
              v-model="playerConf.forwardTimeInSec"
              :min="5"
              :max="20"
              @change="updatePlayerConf"
            />秒
          </div>
        </div>
      </div>
      <div class="setting-item">
        <div class="title">直播源管理</div>
        <div class="setting-item-box">
          <div class="vop-input">
            <input
              type="checkbox"
              v-model="systemConf.allowPassWhenIptvCheck"
              @change="updateSettingEvent"
            />
            检测时自动跳过停用源
          </div>
          <div class="vop-input">
            <input
              type="checkbox"
              v-model="systemConf.autocleanWhenIptvCheck"
              @change="updateSettingEvent"
            />
            检测时自动清理无效源
          </div>
          <div class="vop-input">
            <input
              type="checkbox"
              v-model="playerConf.autoChangeSourceWhenIptvStalling"
              @change="updatePlayerConf"
            />
            卡顿时自动换源换台:
            <el-input-number
              style="width: 100px"
              controls-position="right"
              :min="5"
              :max="20"
              v-model="playerConf.waitingTimeInSec"
              @change="updatePlayerConf"
            />秒
          </div>
        </div>
      </div>
      <div class="setting-item">
        <div class="title">源管理</div>
        <div class="setting-item-box">
          <div class="vop-select">
            <div class="vs-placeholder vs-noAfter" @click="editSitesEvent">
              <span>编辑源</span>
            </div>
          </div>
          <div class="vop-select">
            <div
              class="vs-placeholder vs-noAfter"
              @click="settingInfo.configDefaultParseUrlDialog = true"
            >
              <span>设置默认解析接口</span>
            </div>
          </div>
        </div>
      </div>
      <div class="setting-item">
        <div class="title">窗口及播放</div>
        <div class="setting-item-box">
          <div class="vop-input">
            <input
              type="checkbox"
              v-model="systemConf.saveWindowState"
              @change="updateSystemConf"
            />
            记录并恢复窗口位置和大小
            <input
              type="checkbox"
              v-model="systemConf.pauseWhenMinimize"
              @change="updateSystemConf"
            />
            最小化时暂停播放
          </div>
        </div>
      </div>
      <div class="theme">
        <div class="title">主题</div>
        <div class="theme-box">
          <div @click="changeTheme('light')" class="theme-item light">
            <div class="theme-image">
              <img src="../assets/image/light.png" alt="" />
            </div>
            <div class="theme-name">Light</div>
          </div>
          <div @click="changeTheme('dark')" class="theme-item dark">
            <div class="theme-image">
              <img src="../assets/image/dark.png" alt="" />
            </div>
            <div class="theme-name">Dark</div>
          </div>
        </div>
      </div>
      <div class="clearDB">
        <span @click="clearDBEvent" class="clearBtn">检查更新</span>
        <span @click="clearDBEvent" class="clearBtn">软件重置</span>
        <span @click="changePasswordEvent" class="clearBtn">设置密码</span>
        <div class="clearTips">
          如果新安装用户, 无法显示资源, 请点击软件重置. 如非必要, 切勿点击.
          会清空用户数据, 恢复默认设置. 点击即软件重置, 并关闭软件.
        </div>
      </div>
      <div class="Tips">
        <span
          >所有资源来自网上, 本软件不参与任何制作, 上传, 储存等内容,
          禁止传播违法资源. 本软件仅供学习参考, 请于安装后24小时内删除.</span
        >
      </div>
    </div>
    <div>
      <!-- 设置默认解析接口 -->
      <el-dialog
        :visible.sync="settingInfo.configDefaultParseUrlDialog"
        v-if="settingInfo.configDefaultParseUrlDialog"
        title="设置默认解析接口"
        :append-to-body="true"
        @close="closeDialog"
      >
        <el-form label-width="45px" label-position="left">
          <el-form-item label="URL:">
            <el-input
              v-model="setting.defaultParseURL"
              :autosize="{ minRows: 2, maxRows: 4 }"
              type="textarea"
              placeholder="请输入解析接口地址，为空时会自动设置，重置时会自动更新默认接口地址"
            />
          </el-form-item>
        </el-form>
        <span slot="footer" class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="danger" @click="resetDefaultParseURL"
            >重置</el-button
          >
          <el-button type="primary" @click="configDefaultParseURL"
            >确定</el-button
          >
        </span>
      </el-dialog>
    </div>
    <div>
      <!-- 输入密码页面 -->
      <el-dialog v-model="settingInfo.checkPasswordDialog" v-if='settingInfo.checkPasswordDialog' :append-to-body="true" 
        @close="closeDialog" width="300px">
        <el-form label-width="75px" label-position="left">
          <el-form-item label="当前密码" prop='name'>
            <el-input v-model="settingInfo.inputPassword" placeholder="请输入您的当前密码" />
          </el-form-item>
        </el-form>
        <span slot="footer" class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="primary" @click="checkPasswordEvent">确定</el-button>
        </span>
      </el-dialog>
    </div>
    <div>
      <!-- 修改密码页面 -->
      <el-dialog v-model="settingInfo.changePasswordDialog" v-if='settingInfo.changePasswordDialog' :append-to-body="true" 
        @close="closeDialog" width="300px">
        <el-form label-width="75px" label-position="left">
          <el-form-item label="新密码" prop='name'>
            <el-input v-model="settingInfo.inputPassword" placeholder="请输入您的新密码" />
          </el-form-item>
        </el-form>
        <span slot="footer" class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="primary" @click="confirmedChangePasswordEvent">确定</el-button>
        </span>
      </el-dialog>
    </div>
    <div>
      <!-- 代理设置界面 -->
      <!-- <el-dialog :visible.sync="show.proxyDialog" :append-to-body="true" @close="closeDialog" width="400px">
        <el-form label-width="50px" label-position="left" size="small">
          <el-form-item label="协议: " prop='scheme'>
            <el-col :span="15">
              <el-select v-model="proxy.scheme" placeholder="请选择协议类型">
                <el-option label="http" value="http"></el-option>
                <el-option label="socks5" value="socks5"></el-option>
              </el-select>
            </el-col>
          </el-form-item>
          <el-form-item label="地址: " prop='url'>
            <el-col :span="15">
              <el-input v-model="proxy.url" placeholder="地址" />
            </el-col>
            <el-col class="line" :span="2" style="text-align: center;">:</el-col>
            <el-col :span="7">
              <el-input v-model="proxy.port" placeholder="端口" width="80px" />
            </el-col>
          </el-form-item>
        </el-form>
        <span slot="footer" class="dialog-footer">
          <el-button @click="closeDialog">取消</el-button>
          <el-button type="primary" @click="proxyConfirm">确定</el-button>
        </span>
      </el-dialog> -->
    </div>
    <div class="update" v-if="updateInfo.showUpdate">
      <div class="wrapper">
        <div class="body">
          <div class="content" v-html="updateBody"></div>
        </div>
        <div class="footer">
          <el-progress
            :color="colors"
            :percentage="percentage"
            :indeterminate="true"
          />
          <el-button size="small" @click="closeUpdate">关闭</el-button>
          <el-button size="small" v-show="updateInfo.downloaded == 0" @click="startUpdate">更新</el-button>
          <el-button size="small" v-show="updateInfo.downloaded < total">正在更新...</el-button>
          <el-button size="small" v-show="updateInfo.downloaded > total">安装中...</el-button>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
import { defineComponent, reactive, ref, computed, onMounted, onBeforeMount } from "vue";
import { useCoreStore } from "@/store";
import { storeToRefs } from "pinia";
import { writeText } from "@tauri-apps/api/clipboard";
import { open } from "@tauri-apps/api/shell";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { listen } from "@tauri-apps/api/event";
import { getVersion } from "@tauri-apps/api/app";
import { ElMessage } from "element-plus";
import { _ } from "lodash";
import bcrypt from "bcryptjs";
import {marked} from 'marked';
import {
  Close
} from "@element-plus/icons-vue";

export default defineComponent({
  name: "Setting",
  setup() {
    const coreStore = useCoreStore();
    const { getPlayerConf, getSystemConf, updatePlayerConf, updateSystemConf, refreshSystemConf } =
      coreStore;
    const { view, systemConf, playerConf } = storeToRefs(coreStore);

    const settingInfo = reactive({
      shortcut: false,
      excludeRootClasses: false,
      excludeR18Films: false,
      configDefaultParseUrlDialog: false,
      checkPasswordDialog: false,
      changePasswordDialog: false,
      rootClass: "",
      r18Class: "",
      inputPassword: "",
      action: "",
    });

    const unlisten = ref(0);
    const eventId = ref(0);
    const updateInfo = reactive({
      body: "",
      total: 0,
      downloaded: 0,
      showUpdate: false,
    });

    const colors = [
      { color: '#f56c6c', percentage: 20 },
      { color: '#e6a23c', percentage: 40 },
      { color: '#5cb87a', percentage: 60 },
      { color: '#1989fa', percentage: 80 },
      { color: '#6f7ad3', percentage: 100 },
    ]

    const expShortcut = async () => {
      const arr = [...this.shortcutList];
      const str = JSON.stringify(arr, null, 2);
      await writeText(str);
      ElMessage.success("已复制到剪贴板");
    };

    const linkOpen = async (url) => {
      await open(url);
    };

    const changeTheme = (e) => {
      systemConf.value.theme = "theme-" + e;
      updateSystemConf();
    };
    
    const closeDialog = () => {
      settingInfo.checkPasswordDialog = false
      settingInfo.changePasswordDialog = false
      settingInfo.configDefaultParseUrlDialog = false
      // if (this.show.proxyDialog) {
      //   this.show.proxyDialog = false
      //   this.setting.proxy.type = 'none'
      //   await this.updateSettingEvent()
      //   this.$message.info('取消使用代理')
      //   zy.proxy()
      // }
      settingInfo.inputPassword = ''
    }
    
    const changePasswordEvent = () => {
      if (systemConf.value.encryptedPassword) {
        settingInfo.action = 'ChangePassword'
        settingInfo.checkPasswordDialog = true
      } else {
        settingInfo.changePasswordDialog = true
      }
    }

    const checkPasswordEvent = () => {
      let pwdSuccess = systemConf.value.encryptedPassword ? false : true;
      if (!pwdSuccess) {
        pwdSuccess = bcrypt.compareSync(settingInfo.inputPassword, systemConf.value.encryptedPassword)
      }
      if (pwdSuccess) {
        closeDialog()
        if (settingInfo.action === 'EditSites') {
          view.value = 'EditSites'
        } else if (settingInfo.action === 'ChangePassword') {
          settingInfo.changePasswordDialog = true
        } else if (settingInfo.action === 'CleanDB') {
          this.clearDB()
        }
      } else {
        ElMessage.error('您输入的密码错误，请重试')
      }
    }
    
    const confirmedChangePasswordEvent = () => {
      const salt = bcrypt.genSaltSync(10);
      const hash = bcrypt.hashSync(settingInfo.inputPassword, salt);
      systemConf.value.encryptedPassword = hash;
      updateSystemConf();
      refreshSystemConf();
      closeDialog()
    }
    
    const editSitesEvent = () => {
      if (systemConf.value.encryptedPassword) {
        settingInfo.action = 'EditSites'
        settingInfo.checkPasswordDialog = true
      } else {
        view.value = 'EditSites'
      }
    }

    const addRootClassFilter = () => {
      systemConf.value.rootClassFilter.push(settingInfo.rootClass);
      settingInfo.rootClass = "";
      updateSystemConf();
    }

    const removeRootClassFilter = (o) => {
      systemConf.value.rootClassFilter = _.remove(systemConf.value.rootClassFilter, function(n) {
        return n != o;
      });
      updateSystemConf();
    }

    const addR18ClassFilter = () => {
      systemConf.value.r18ClassFilter.push(settingInfo.r18Class);
      settingInfo.rootClass = "";
      updateSystemConf();
    }

    const removeR18ClassFilter = (o) => {
      systemConf.value.r18ClassFilter = _.remove(systemConf.value.r18ClassFilter, function(n) {
        return n != o;
      });
      updateSystemConf();
    }

    const clearDBEvent = () => {
      if (systemConf.value.encryptedPassword) {
        settingInfo.action = 'CleanDB'
        settingInfo.checkPasswordDialog = true
      } else {
        clearDB()
      }
    }
    
    const clearDB = () => {
      db.delete().then(res => {
        ElMessage.success('重置成功')
        const win = remote.getCurrentWindow()
        win.destroy()
      })
    }

    const openUpdate = () => {
      updateInfo.showUpdate = true
    }

    const closeUpdate = () => {
      updateInfo.showUpdate = false
    }

    const startUpdate = () => {
      installUpdate().then(
        () => {
          ElMessage.success("更新成功");
        },
        (e) => {
          ElMessage.error(e.toString());
        }
      )
    }

    const percentage = computed(() => updateInfo.downloaded/updateInfo.total)

    const updateBody = computed(() => marked.parse(updateInfo.body))

    onBeforeMount(() => {
      getSystemConf();
      getPlayerConf();
    });

    onMounted(() => {
      checkUpdate().then(
        (update) => {
          if (update.shouldUpdate) {
            updateInfo.showUpdate = true;
            updateInfo.body = update.manifest.body;
          } else {
            updateInfo.body = "##已经是最新的版本";
          }
        },
        (e) => {
          updateInfo.body = e.toString();
          ElMessage.error(e.toString());
        }
      );
      if (unlisten.value === 0) {
        unlisten.value = listen("tauri://update-download-progress", (e) => {
          if (eventId.value === 0) {
            eventId.value = e.id;
          }
          if (e.id === eventId.value) {
            updateInfo.total = e.payload.contentLength;
            updateInfo.downloaded += e.payload.chunkLength;
          }
        });
      }
    });

    return {
      systemConf,
      settingInfo,
      playerConf,
      updateSystemConf,
      expShortcut,
      linkOpen,
      changeTheme,
      updatePlayerConf,
      closeDialog,
      changePasswordEvent,
      checkPasswordEvent,
      editSitesEvent,
      addRootClassFilter,
      Close,
      removeRootClassFilter,
      addR18ClassFilter,
      removeR18ClassFilter,
      clearDBEvent,
      confirmedChangePasswordEvent,
      updateInfo,
      percentage,
      colors,
      openUpdate,
      closeUpdate,
      startUpdate,
      updateBody,
    };
  },
});
// export default {
//   data () {
//     return {
//       shortcutList: [],
//       show: {
//         proxy: false,
//         proxyDialog: false,
//       },
//       proxy: {
//         type: '',
//         scheme: '',
//         url: '',
//         port: ''
//       },
//     }
//   },
//   methods: {
//     getSetting () {
//       setting.find().then(res => {
//         this.d = res
//         this.setting = this.d
//         if (!this.setting.defaultParseURL) this.configDefaultParseURL()
//       })
//     },
//     getShortcut () {
//       shortcut.all().then(res => {
//         this.shortcutList = res
//       })
//     },
//     async clearCache () {
//       const win = remote.getCurrentWindow()
//       const ses = win.webContents.session
//       const size = await ses.getCacheSize() / 1024 / 1024
//       const mb = size.toFixed(2)
//       await ses.clearCache()
//       this.$message.success(`清除缓存成功, 共清理 ${mb} MB`)
//     },
//     async resetDefaultParseURL () {
//       this.setting.defaultParseURL = 'https://jx.bpba.cc/?v='
//     },
//     async configDefaultParseURL () {
//       if (!this.setting.defaultParseURL) await this.resetDefaultParseURL()
//       this.d.defaultParseURL = this.setting.defaultParseURL?.trim()
//       this.show.configDefaultParseUrlDialog = false
//       this.updateSettingEvent()
//     },
//     resetShortcut () {
//       shortcut.clear().then(shortcut.add(defaultShortcuts)).then(res => {
//         this.getShortcut()
//         this.$message.success('快捷键已重置')
//         this.d.shortcutModified = true
//         this.updateSettingEvent()
//       })
//     },
//     async changeProxyType (e) {
//       this.d.proxy.type = e
//       if (e === 'manual') {
//         this.show.proxyDialog = true
//         this.proxy.scheme = this.setting.proxy.scheme
//         this.proxy.url = this.setting.proxy.url
//         this.proxy.port = this.setting.proxy.port
//       }
//       await this.updateSettingEvent()
//       this.show.proxy = false
//       zy.proxy()
//     },
//     async proxyConfirm () {
//       this.d.proxy.scheme = this.proxy.scheme
//       this.d.proxy.url = this.proxy.url
//       this.d.proxy.port = this.proxy.port
//       await this.updateSettingEvent()
//       this.show.proxyDialog = false
//       zy.proxy()
//       this.$message.info('开始使用代理')
//     },
//   },
//   created () {
//     this.getSetting()
//     this.getShortcut()
//     this.checkUpdate()
//   }
// }
</script>
<style lang="scss" scoped>
.setting {
  height: calc(100% - 60px);
  width: 100%;
  border-radius: 5px;
  .setting-box {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }
  .logo {
    margin-top: 10px;
    text-align: center;
    img {
      width: 120px;
      height: auto;
    }
  }
  .info {
    width: 100%;
    margin-top: 20px;
    text-align: center;
    a {
      text-decoration: none;
      margin: 0 10px;
      font-size: 14px;
      cursor: pointer;
    }
  }
  .setting-item {
    padding: 20px;
    margin-top: 20px;
    .setting-item-box {
      margin-top: 10px;
      .vop-select {
        margin-right: 20px;
      }
      .vop-array-input {
        margin-top: 10Px;
        .vop-input{
          width: 50%;
          margin-left: 10Px;
        }
        .tags {
          display: flex;
          flex-wrap: wrap;
          padding: 4px 0;
          .tag {
            color: rgba(0, 0, 0, 0.25);
            background-color: #F0F2F5;
            opacity: 1;
            padding: 3px 8px;
            margin: 0px 10px 4px 0px;
            border: 1px solid #DCDFE6;
            border-radius: 4px;
            .icon {
              cursor: pointer;
            }
          }
        }
      }
    }
  }

  .theme {
    padding-left: 20px;
    margin-top: 20px;
    .theme-box {
      display: flex;
      flex-wrap: wrap;
      justify-content: flex-start;
      margin-top: 10px;
      .theme-item {
        width: 200px;
        height: 180px;
        margin-right: 20px;
        cursor: pointer;
        border-radius: 2px;
        .theme-image {
          width: 180px;
          margin: 10px auto;
          img {
            width: 100%;
          }
        }
        .theme-name {
          width: 100%;
          text-align: center;
        }
      }
    }
  }
  .clearDB {
    margin-top: 20px;
    margin-bottom: 20px;
    .clearBtn {
      margin-left: 20px;
      color: red;
      cursor: pointer;
      border: 1px solid #ff000088;
      display: inline-block;
      width: 160px;
      height: 32px;
      font-size: 14px;
      text-align: center;
      line-height: 32px;
    }
    .clearTips {
      margin: 10px 0 0 20px;
      font-size: 12px;
      color: #ff000088;
    }
  }
  .Tips {
    margin: 20px;
    font-size: 12px;
    color: #ff000066;
  }
  .update {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(7, 17, 27, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    .wrapper {
      background-color: #fff;
      padding: 20px 50px 40px;
      border-radius: 4px;
      max-width: 500px;
      max-height: 90%;
      overflow: auto;
      .footer {
        display: flex;
        justify-content: flex-end;
      }
    }
  }
}
</style>
