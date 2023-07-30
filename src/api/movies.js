import fetch from './fetch';
import { XMLParser } from 'fast-xml-parser';
import { load } from 'cheerio';

const parsetConfig = { // XML 转 JSON 配置
  trimValues: true,
  textNodeName: '_t',
  ignoreAttributes: false,
  attributeNamePrefix: '_',
  parseAttributeValue: true
};

const parser = new XMLParser(parsetConfig);

class SiteClassXmlParser {
    doParserClass(resolve, reject, data) {
        const json = parser.parse(data)
        const jsondata = json?.rss === undefined ? json : json.rss
        if (!jsondata?.class || !jsondata?.list) reject("解析xml数据为空")
        const arr = []
        if (jsondata.class) {
            // 有些网站返回的分类名里会含有一串包含在{}内的字符串,移除掉
            const regex = /\{.*\}/i
            for (const i of jsondata.class.ty) {
                const j = {
                    id: i._id,
                    name: i._t.replace(regex, '')
                }
                arr.push(j)
            }
        }
        const doc = {
            classList: arr,
            page: jsondata.list._page,
            pageCount: jsondata.list._pagecount,
            pageSize: jsondata.list._pagesize,
            recordCount: jsondata.list._recordcount
        }
        resolve(doc)
    }

    doParserVideo(resolve, reject, data) {
        const json = parser.parse(data)
        const jsondata = json.rss === undefined ? json : json.rss
        const videoList = jsondata.list.video
        if (videoList && videoList.length) {
          resolve(videoList)
        } else {
          resolve([])
        }
    }

    doParserVideoDetail(resolve, reject, data) {
        const json = parser.parse(data)
        const jsondata = json?.rss === undefined ? json : json.rss
        const videoList = jsondata?.list?.video
        if (!videoList) resolve()
        // Parse video lists
        let fullList = []
        let index = 0
        const supportedFormats = ['m3u8', 'mp4']
        const dd = videoList.dl.dd
        const $ = load(videoList.des)
        videoList.des = $('p').first().children("span").text()
        const type = Object.prototype.toString.call(dd)
        if (type === '[object Array]') {
            for (const i of dd) {
                i._t = i._t.replace(/\$+/g, '$')
                const ext = Array.from(new Set(...i._t.split('#').map(e => e.includes('$') ? e.split('$')[1].match(/\.\w+?$/) : e.match(/\.\w+?$/)))).map(e => e.slice(1))
                if (ext.length && ext.length <= supportedFormats.length && ext.every(e => supportedFormats.includes(e))) {
                    if (ext.length === 1) {
                        i._flag = ext[0] + '-' + index
                    } else {
                        i._flag = index ? 'ZY支持-' + index : 'ZY支持'
                    }
                    index++
                }
                fullList.push({
                    flag: i._flag,
                    list: i._t.split('#').filter(e => e && (e.startsWith('http') || (e.split('$')[1] && e.split('$')[1].startsWith('http'))))
                })
            }
        } else {
            fullList.push({
                flag: dd._flag,
                list: dd._t.replace(/\$+/g, '$').split('#').filter(e => e && (e.startsWith('http') || (e.split('$')[1] && e.split('$')[1].startsWith('http'))))
            })
        }
        fullList.forEach(item => {
            if (item.list.every(e => e.includes('$') && /^\s*\d+\s*$/.test(e.split('$')[0]))) 
                item.list.sort((a, b) => { 
                    return a.split('$')[0] - b.split('$')[0] 
                })
        })
        if (fullList.length > 1) { // 将ZY支持的播放列表前置
            index = fullList.findIndex(e => supportedFormats.includes(e.flag) || e.flag.startsWith('ZY支持'))
            if (index !== -1) {
                const first = fullList.splice(index, 1)
                fullList = first.concat(fullList)
            }
        }
        videoList.fullList = fullList
        resolve(videoList)
    }
}

class SiteClassJsonParser {
    doParserClass(resolve, reject, data) {
        const json = JSON.parse(data)
        const arr = []
        if (json.class) {
            for (const i of json.class) {
                const j = {
                    id: i.type_id,
                    name: i.type_name
                }
                arr.push(j)
            }
        }
        const doc = {
            classList: arr,
            page: json.page,
            pageCount: json.pagecount,
            pageSize: json.limit,
            recordCount: json.total
        }
        resolve(doc)
    }

    doParserVideo(resolve, reject, data) {
        throw new Error('Method not implemented.');
    }

    doParserVideoDetail(resolve, reject, data) {
        throw new Error('Method not implemented.');
    }
}

export default  {
    
    /**
     * 获取资源分类 和 所有资源的总数, 分页等信息
     * @param {*} site 资源网信息
     * @returns
     */
    getSiteClass(site) {
        return new Promise((resolve, reject) => {
            fetch.get(site.api, null).then((data) => {
                let dataParser;
                switch (site.name) {
                    case "jyzyapi":
                        dataParser = new SiteClassJsonParser();
                        break;
                    default:
                        dataParser = new SiteClassXmlParser();
                        break;
                }
                dataParser.doParserClass(resolve, reject, data);
            }, (err) => {
                reject(err);
            })
        })
    },

    pageMovies(site, params) { 
        return new Promise((resolve, reject) => {
            params["ac"] = "videolist";
            fetch.get(site.api, params).then((res) => {
                const data = res.match(/<list [^>]*>/)[0] + '</list>' // 某些源站不含页码时获取到的数据parser无法解析
                const json = parser.parse(data)
                const jsondata = json.rss === undefined ? json : json.rss
                const pg = {
                  totalPageCount: jsondata.list._pagecount,
                  pageCount: jsondata.list._pagecount,
                  recordcount: jsondata.list._recordcount,
                  moviesList: []
                }
                resolve(pg)
            }, (err) => {
                reject(err);
            })
        })
    },

    listMovies(site, params) { 
        return new Promise((resolve, reject) => {
            params["ac"] = "videolist";
            fetch.get(site.api, params).then((data) => {
                let dataParser;
                switch (site.name) {
                    case "jyzyapi":
                        dataParser = new SiteClassJsonParser();
                        break;
                    default:
                        dataParser = new SiteClassXmlParser();
                        break;
                }
                dataParser.doParserVideo(resolve, reject, data);
            }, (err) => {
                reject(err);
            })
        })
    },

    /**
     * 获取资源详情
     * @param {*} site 资源网信息
     * @param {*} id 资源唯一标识符 id
     * @returns
     */
    detail(site, id) {
        return new Promise((resolve, reject) => {
            let params = {
                ac: "videolist",
                ids: id
            }
            fetch.get(site.api, params).then(data => {
                let dataParser;
                switch (site.name) {
                    case "jyzyapi":
                        dataParser = new SiteClassJsonParser();
                        break;
                    default:
                        dataParser = new SiteClassXmlParser();
                        break;
                }
                dataParser.doParserVideoDetail(resolve, reject, data);
            }).catch(err => {
                reject(err)
            })
        })
    },

    /**
     * 搜索资源
     * @param {*} key 资源网 key
     * @param {*} wd 搜索关键字
     * @returns
     */
    search(site, wd) {
        return new Promise(async (resolve, reject) => {
            let params = {
                wd: encodeURI(wd),
            }
            fetch.get(site.api, params, false, 3000).then(data => {
                const json = parser.parse(data)
                const jsondata = json?.rss === undefined ? json : json.rss
                if (json && jsondata && jsondata.list) {
                    let videoList = jsondata.list.video
                    if (Object.prototype.toString.call(videoList) === '[object Object]') videoList = [].concat(videoList)
                    videoList = videoList?.filter(e => e.name.toLowerCase().includes(wd.toLowerCase()))
                    if (videoList?.length) {
                        resolve(videoList)
                    } else {
                        resolve()
                    }
                } else {
                    resolve()
                }
            }).catch(err => {
                reject("搜索资源失败");
            })
        })
    },
    
    /**
     * 搜索资源详情
     * @param {*} site 资源网
     * @param {*} wd 搜索关键字
     * @returns
     */
    searchFirstDetail (site, wd) {
        return new Promise((resolve, reject) => {
            let params = {
                wd: encodeURI(wd)
            }
            fetch.get(site.api, params, false, 3000).then(data => {
                const json = parser.parse(data)
                const jsondata = json?.rss === undefined ? json : json.rss
                if (json && jsondata && jsondata.list) {
                    let videoList = jsondata.list.video
                    if (Object.prototype.toString.call(videoList) === '[object Object]') videoList = [].concat(videoList)
                    videoList = videoList?.filter(e => e.name.toLowerCase().includes(wd.toLowerCase()))
                    if (videoList?.length) {
                        this.detail(site, videoList[0].id).then(detailRes => {
                            resolve(detailRes)
                        })
                    } else {
                        resolve()
                    }
                } else {
                    resolve()
                }
            }).catch(err => {
                reject(err)
            })
        }).catch(err => {
            reject(err)
        })
    },

    /**
     * 下载资源
     * @param {*} site 资源网
     * @param {*} id 资源唯一标识符 id
     * @returns
     */
    download (site, id, videoFlag) {
        return new Promise((resolve, reject) => {
            let info = ''
            let downloadUrls = ''
            if (site.download) {
                let params = {
                    ac: "videolist",
                    ids: id,
                    ct: 1
                }
                fetch.get(site.download, params).then(data => {
                    const json = parser.parse(data)
                    const jsondata = json.rss === undefined ? json : json.rss
                    const videoList = jsondata.list.video
                    const dd = videoList.dl.dd
                    const type = Object.prototype.toString.call(dd)
                    if (type === '[object Array]') {
                        for (const i of dd) {
                        downloadUrls = i._t.replace(/\$+/g, '$').split('#').map(e => encodeURI(e.includes('$') ? e.split('$')[1] : e)).join('\n')
                        }
                    } else {
                        downloadUrls = dd._t.replace(/\$+/g, '$').split('#').map(e => encodeURI(e.includes('$') ? e.split('$')[1] : e)).join('\n')
                    }
                    if (downloadUrls) {
                        info = '调用下载接口获取到的链接已复制, 快去下载吧!'
                        resolve({ downloadUrls: downloadUrls, info: info })
                    } else {
                        throw new Error()
                    }
                }).catch((err) => {
                    err.info = '无法获取到下载链接，请通过播放页面点击“调试”按钮获取'
                    reject(err)
                })
            } else {
                this.detail(site, id).then(res => {
                    const dl = res.fullList.find((e, index) => e.flag + '-' + index === videoFlag) || res.fullList[0]
                    for (const i of dl.list) {
                        const url = encodeURI(i.includes('$') ? i.split('$')[1] : i)
                        downloadUrls += (url + '\n')
                    }
                    if (downloadUrls) {
                        info = '视频源链接已复制, 快去下载吧!'
                        resolve({ downloadUrls: downloadUrls, info: info })
                    } else {
                        throw new Error()
                    }
                }).catch((err) => {
                    err.info = '无法获取到下载链接，请通过播放页面点击“调试”按钮获取'
                    reject(err)
                })
            }
        })
    }
}
