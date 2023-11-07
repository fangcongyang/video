import { getUrlType } from '@/business/play';
import { Parser as M3u8Parser } from 'm3u8-parser';
import fetch from '@/api/fetch';
import { load } from 'cheerio';
import { _ } from "lodash";

const onlineSearchUrl = "https://www.foodieguide.com/iptvsearch/";

export default {
  /**
   * iptv在线搜索
   * 
   */
  iptvOnlineSearch(search) {
    return new Promise(async (resolve, reject) => {
      let allIptvUrl = [];
      let searchPromise = [];
      for (let i = 1; i < 4; i++) {
        searchPromise.push(fetch.get(onlineSearchUrl, {s: search, page: i}));
      }
      await Promise.all(searchPromise).then((datas) => {
        datas.map(data => {
          const $ = load(data)
          $('div.tables').each(function () {
              $(this).find("div.result").each(function () {
                const iptvName = $(this).find('div.channel>a>div').first().text()
                const iptvUrl = _.trim($(this).find('div.m3u8>table>tbody>tr>td').eq(1).text())
                if (iptvUrl) {
                  allIptvUrl.push({iptvName: iptvName, iptvUrl: iptvUrl})
                }
              });
          })
        })
      }).catch(err => {
        reject(err);
      })
      let allPromise = [];
      allIptvUrl.forEach(iptv => allPromise.push(this.checkChannelObject(iptv)))
      Promise.all(allPromise).then((validIptvUrl) => {
        resolve(_.filter(validIptvUrl, (item) => {
          return item.iptvName !== "";
        }));
      }).catch(err => {
        console.log(err);
      });
    })
  },

  checkChannelObject (iptv) {
    return new Promise((resolve, reject) => 
      this.checkChannel(iptv.iptvUrl).then(isValid => {
        if (isValid) {
          resolve(iptv);
        } else {
          resolve({iptvName: ""});
        }
      })
    )
  },

  /**
   * 检查直播源
   * @param {*} channel 直播频道 url
   * @returns boolean
   */
  checkChannel (url) {
    return new Promise((resolve, reject) => {
      const movieType = getUrlType(url);
      if (movieType === 'flv') {
        const MAX_CONTENT_LENGTH = 2000 // axios配置maxContentLength不生效，先用request凑合
        let receivedLength = 0
        let options = { uri: url, gzip: true, timeout: 10000 }
        const req = request.get(options)
          .on('data', (str) => {
            receivedLength += str.length
            if (receivedLength > MAX_CONTENT_LENGTH) {
              resolve(true) // 应该用FLVDemuxer.probe来检测，先凑合
              req.abort()
            }
          })
          .on('error', function (err) {
            resolve(false)
            console.log(err)
          })
          .on('end', () => { resolve(false) })
      } else if (movieType === 'm3u8') {
        fetch.get(url, null, false, 2000).then(manifest => {
          const parser = new M3u8Parser()
          parser.push(manifest)
          parser.end()
          const parsedManifest = parser.manifest
          if (parsedManifest.segments.length) {
            resolve(true)
          } else {
            resolve(false)
          }
        }).catch(e => {
          resolve(false)
        })
      } else {
        resolve(false)
      }
    })
  },
}