import validator from 'validator';

export class M3uGenerate {
  extm3u = '#EXTM3U\n';
  m3uData = "";
  extinf = "#EXTINF:";

  appendM3u(duration, title, url) {
      this.m3uData += this.extinf + duration + " " + title + "\n";
      this.m3uData += url + "\n";
  }

  toString() {
      return this.extm3u + this.m3uData
  }
}

export class M3uParse {
    extm3u = '#EXTM3U\n';
    m3uData = "";
    extinf = "#EXTINF:";
    header = {};
    items = [];

    parse(content) {
        let lines = content.split('\n').map(this.parseLine);      
        let firstLine = lines.find(l => l.index === 0);
        if (!firstLine || !/^#EXTM3U/.test(firstLine.raw)) throw new Error('播放列表无效');
        
        this.header = this.parseHeader(firstLine);
        let i = 0
        const items = {}
        for (let line of lines) {
          if (line.index === 0) continue
          const string = line.raw.toString().trim()
          if (string.startsWith('#EXTINF:')) {
            const EXTINF = string
            items[i] = {
              name: this.getName(EXTINF),
              tvg: {
                id: this.getAttribute(EXTINF, 'tvg-id'),
                name: this.getAttribute(EXTINF, 'tvg-name'),
                logo: this.getAttribute(EXTINF, 'tvg-logo'),
                url: this.getAttribute(EXTINF, 'tvg-url'),
                rec: this.getAttribute(EXTINF, 'tvg-rec'),
              },
              group: {
                title: this.getAttribute(EXTINF, 'group-title'),
              },
              http: {
                referrer: '',
                'user-agent': this.getAttribute(EXTINF, 'user-agent')
              },
              url: undefined,
              raw: line.raw,
              line: line.index + 1,
              catchup: {
                type: this.getAttribute(EXTINF, 'catchup'),
                days: this.getAttribute(EXTINF, 'catchup-days'),
                source: this.getAttribute(EXTINF, 'catchup-source')
              },
              timeshift: this.getAttribute(EXTINF, 'timeshift')
            }
          } else if (string.startsWith('#EXTVLCOPT:')) {
            if (!items[i]) continue
            const EXTVLCOPT = string
            items[i].http.referrer = this.getOption(EXTVLCOPT, 'http-referrer') || items[i].http.referrer
            items[i].http['user-agent'] =
              this.getOption(EXTVLCOPT, 'http-user-agent') || items[i].http['user-agent']
            items[i].raw += `\r\n${line.raw}`
          } else if (string.startsWith('#EXTGRP:')) {
            if (!items[i]) continue
            const EXTGRP = string
            items[i].group.title = this.getValue(EXTGRP) || items[i].group.title
            items[i].raw += `\r\n${line.raw}`
          } else {
            if (!items[i]) continue
            const url = this.getURL(string)
            const user_agent = this.getParameter(string, 'user-agent')
            const referrer = this.getParameter(string, 'referer')
            if (url && (validator.isURL(url))) {
              items[i].url = url
              items[i].http['user-agent'] = user_agent || items[i].http['user-agent']
              items[i].http.referrer = referrer || items[i].http.referrer
              items[i].raw += `\r\n${line.raw}`
              i++
            } else {
              if (!items[i]) continue
              items[i].raw += `\r\n${line.raw}`
            }
          }
        }
      
        this.items = Object.values(items);
    }

    parseLine(line, index) {
        return {
            index,
            raw: line
        }
    }

    parseHeader(line) {
        const supportedAttrs = ['x-tvg-url', 'url-tvg']
        
        let attrs = {}
        for (let attrName of supportedAttrs) {
            const tvgUrl = line.raw.getAttribute(attrName)
            if (tvgUrl) {
            attrs[attrName] = tvgUrl
            }
        }
        
        return {
            attrs,
            raw: line.raw
        }
    }
    
    getAttribute (oldStr, name) {
        let regex = new RegExp(name + '="(.*?)"', 'gi')
        let match = regex.exec(oldStr)
    
        return match && match[1] ? match[1] : ''
    }
    
    getName (oldStr) {
        let info = oldStr.replace(/\="(.*?)"/g, '')
        let parts = info.split(/,(.*)/)
    
        return parts[1] || ''
    }

    getOption (oldStr, name) {
        let regex = new RegExp(':' + name + '=(.*)', 'gi')
        let match = regex.exec(oldStr)
    
        return match && match[1] && typeof match[1] === 'string' ? match[1].replace(/\"/g, '') : ''
    }

    getValue (oldStr) {
        let regex = new RegExp(':(.*)', 'gi')
        let match = regex.exec(oldStr)
        
        return match && match[1] && typeof match[1] === 'string' ? match[1].replace(/\"/g, '') : ''
    }
      
    getURL (oldStr) {
        return oldStr.split('|')[0] || ''
    }

    getParameter (oldStr, name) {
        const params = oldStr.replace(/^(.*)\|/, '')
        const regex = new RegExp(name + '=(\\w[^&]*)', 'gi')
        const match = regex.exec(params)
        
        return match && match[1] ? match[1] : ''
    }
}

export const validityIptvUrl = (url) => {
  const supportFormats = "m3u8|flv";
  let videoType = "";
  const regex = /\.(m3u8|flv)(\?|$)/m;
  const match = url.match(regex);
  if (match) {
    videoType = match[1];
  }
  return supportFormats.indexOf(videoType) != -1;
}

export const determineGroup = (name) => {
  if (name.toLowerCase().includes('cctv') || name.toLowerCase().includes('cgtn')) {
    return '央视'
  } else if (name.includes('香港') || name.includes('澳门') || name.includes('台湾') || name.includes('凤凰') || name.includes('翡翠')) {
    return '港澳台'
  } else if (name.includes('卫视')) {
    return '卫视'
  } else if (name.includes('高清') || name.includes('蓝光') || name.includes('1080P')) {
    return '高清'
  } else {
    return '其他'
  }
}
