import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import { useCoreStore } from './index';

export const useIptvStore = defineStore('iptv', {
    state: () => {
      return {
        init: {
          channelGroupListInit: false,
        },
        channelGroupList: [],
        channelList: [],
        channelGroupMap: {},
        channelMap: {},
        channelGroupTree: [],
        channelGroupFilter: [],
      }
    },
    actions:{
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
          channelGroup["channels"] = channels;
          channelGroupName = channelGroup.channel_group_name;
          this.channelGroupMap[channelGroup.channel_name + channelGroupName] = channelGroup;

          // 获取表格分组名称筛选
          if (!channelGroupFilterMap[channelGroupName]) {
            channelGroupFilterMap[channelGroup.channel_group_name] = "1";
            this.channelGroupFilter.push({ text: channelGroupName, value: channelGroupName })
          }
          
          for (let channel of channelGroup.channels) {
            channel["name"] = channelGroup["channel_name"];
            channel["channelGroupId"] = channelGroup["id"];
            this.channelList.push(channel);
            this.channelMap[channel.url] = channel;
          }
        }
        const channelGroups = [...new Set(channelGroupList.map(iptv => iptv.channel_group_name))]
        channelGroups.forEach(cg => {
          const doc = {
            label: cg,
            children: channelGroupList.filter(x => x.channel_group_name === cg).map(i => { return { label: i.channel_name, channel_group_name: i } })
          }
          this.channelGroupTree.push(doc)
        })
        this.channelGroupList = channelGroupList;
      },
    },
    getters:{
      currentChannel() { 
        const core = useCoreStore();
        return this.channelGroupList.filter(item => item.id == core.playInfo.iptv.channelGroupId)[0];
      },
    }
  })