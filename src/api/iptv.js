import { getUrlType } from '@/business/play';
import { Parser as M3u8Parser } from 'm3u8-parser';
import fetch from '@/api/fetch';

export default {
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
        fetch.get(url, null, false, 2).then(manifest => {
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
      }
    })
  },
}