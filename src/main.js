import { createApp } from "vue";
import "@/assets/scss/theme.scss";
import App from "./App.vue";
import { createPinia } from "pinia";
import ElementPlus from "element-plus";
import "virtual:svg-icons-register";
import "element-plus/theme-chalk/index.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import zhCn from "element-plus/dist/locale/zh-cn.mjs";
import VueLazyload from "vue-lazyload";
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import ContextMenu from '@imengyu/vue3-context-menu'

const pinia = createPinia();

const app = createApp(App);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app
  .use(ElementPlus, {
    locale: zhCn,
  })
  .use(pinia)
  .use(VueLazyload)
  .use(ContextMenu)
  .mount("#app");
