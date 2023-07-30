import DPlayer from 'dplayerplus';
import flvjs from 'flv.js';
import Hls from 'hls.js';

export class MoviesPlayer {
    videoName = "";
    playerType = "";
    playing = false;
    dpConfig = {
        container: null,
        isLive: false,
        autoplay: false,
        volume: 0.6,
        lang: "zh-cn",
        video: {
            type: "customHls", 
            customType: {},
        },
        highlight: [],
    };
    dp;
    
    constructor(playerType, playerInfo, playerConf) {
        this.dpConfig.container = document.getElementById('dpplayer'),
        this.playerType = playerType;
        this.dpConfig.volume = playerConf.volume;
        this.dpConfig.isLive = playerInfo.isLive;
        switch(playerType) {
            case 'mp4':
                this.dpConfig.video.type = "mp4";
                this.dpConfig.video.customType = {};
                break
            case 'flv':
                this.dpConfig.video.type = "customFlv";
                this.dpConfig.video.customType = {
                    customFlv: function (video, player) {
                    const flvPlayer = flvjs.createPlayer({
                        type: 'flv',
                        url: video.src,
                    });
                    flvPlayer.attachMediaElement(video);
                    flvPlayer.load();
                    }
                } 
                break
            default:
                this.dpConfig.video.type = "customHls";
                this.dpConfig.video.customType = {
                    customHls: function (video, player) {
                        const hls = new Hls({
                            debug: false,
                            p2pConfig: {
                              live: false,
                            },
                        });
                        hls.loadSource(video.src)
                        hls.attachMedia(video)
                    }
                }
                break;
        }
        this.dp = new DPlayer(this.dpConfig);
    }

    durationchange() {
        if (this.dp) {
            const event = new Event("durationchange");
            this.dp.container.querySelector('.dplayer-video-current').dispatchEvent(event);
        }
    }

    duration() {
        return this.dp ? this.dp.video ? this.dp.video.duration ? this.dp.video.duration: 0.0 : 0.0 : 0.0;
    }

    currentTime() {
        return this.dp ? this.dp.video ? this.dp.video.currentTime : 0.0 : 0.0;
    }
    
    playerExist() {
        return this.dp
    }

    videoExist() {
        return this.dp && this.dp.video
    }

    destroy() {
        if (this.dp) {
            this.dp.destroy();
            this.dp = null;
        }
    }
}

export const getUrlType = (url) => {
    const regex = /\.(m3u8|flv|mp4)(\?|$)/m;
    const match = url.match(regex);
    if (match) {
        return match[1];
    }
}

export const getPlayerType = (movieUrl) => {
    let videoType = "customHls";
    if (movieUrl) {
        switch (getUrlType(movieUrl)) {
            case "flv":
                videoType = "customFlv";
                break;
            case "mp4":
                videoType = "mp4";
                break;
            default:
                videoType = "customHls";
                break;
        }
    }
    return videoType;
}

export const getIsVipMovies = (movieUrl) => {
    const vipWebsites = ['iqiyi', 'tenxun']
    vipWebsites.forEach(v => {
        if (movieUrl.indexOf(v) != -1) {
            return true
        }
    })
    return false
}