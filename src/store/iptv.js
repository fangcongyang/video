import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import { _ as lodash } from 'lodash';

export const useIptvStore = defineStore('iptv', {
    state: () => {
      return {
        init: {
          channelGroupListInit: false,
        },
        channelId: 0,
        channelGroupId: 0,
        channelGroupList: [],
        channelList: [],
        channelGroupMap: {},
        channelMap: {},
        channelGroupTree: [],
        channelGroupFilter: [],
      }
    },
    actions:{
      getCurrentChannel() { 
        return this.channelGroupList.filter(item => item.id == this.channelGroupId)[0];
      },

      async getAllChannelGroup() { 
        if (!this.init.channelGroupListInit) {
          await this.refreshChannelGroupList();
          this.init.channelGroupListInit = true;
        }
      },

      async refreshChannelGroupList() {
        this.channelList = [];
        this.channelGroupTree = [];
        this.channelGroupMap = {};
        this.channelMap = {};
        this.channelGroupFilter = [];
        let channelGroupList = await invoke("select_channel_group", {});
        let channelGroupFilterMap = {}
        let channelGroupName;
        for (let channelGroup of channelGroupList) {
          let channels = JSON.parse(channelGroup.channels);
          channelGroup.hasChildren = channelGroup.hasChildren == '1';
          channelGroup["channels"] = channels;
          channelGroupName = channelGroup.channelGroup
          this.channelGroupMap[channelGroup.name + channelGroupName] = channelGroup;

          // 获取表格分组名称筛选
          if (!channelGroupFilterMap[channelGroupName]) {
            channelGroupFilterMap[channelGroup.channelGroup] = "1";
            this.channelGroupFilter.push({ text: channelGroupName, value: channelGroupName })
          }
          
          for (let channel of channelGroup.channels) {
            channel["name"] = channelGroup["name"];
            channel["channelGroupId"] = channelGroup["id"];
            this.channelList.push(channel);
            this.channelMap[channel.url] = channel;
          }
        }
        const channelGroups = [...new Set(channelGroupList.map(iptv => iptv.channelGroup))]
        channelGroups.forEach(cg => {
          const doc = {
            label: cg,
            children: channelGroupList.filter(x => x.channelGroup === cg).map(i => { return { label: i.name, channelGroup: i } })
          }
          this.channelGroupTree.push(doc)
        })
        this.channelGroupList = channelGroupList;
      },
    },
    getters:{
      getSourcePlayList() {
        return lodash.find(this.channelGroupList, { id: this.channelGroupId }).channels;
      },
    }
  })