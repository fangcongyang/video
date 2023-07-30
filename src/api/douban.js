import fetch from './fetch';
import { load } from 'cheerio';

export default {
    /**
     * 获取豆瓣页面链接
     * @param {*} name 视频名称
     * @param {*} year 视频年份
     * @returns 豆瓣页面链接，如果没有搜到该视频，返回搜索页面链接
     */
    doubanLink (name, year) {
        return new Promise((resolve, reject) => {
            // 豆瓣搜索链接
            const nameToSearch = name.replace(/\s/g, '')
            let params = {
                q: nameToSearch
            };
            const doubanSearchLink = 'https://www.douban.com/search'
            fetch.get(doubanSearchLink, params).then(data => {
                const $ = load(data)
                // 查询所有搜索结果, 看名字和年代是否相符
                let link = ''
                $('div.result').each(function () {
                    const linkInDouban = $(this).find('div>div>h3>a').first()
                    const nameInDouban = linkInDouban.text().replace(/\s/g, '')
                    const subjectCast = $(this).find('span.subject-cast').text()
                    if (nameToSearch === nameInDouban && subjectCast && subjectCast.includes(year)) {
                        link = linkInDouban.attr('href')
                    }
                })
                if (link) {
                    resolve(link)
                } else {
                    // 如果没找到符合的链接，返回搜索页面
                    resolve(doubanSearchLink)
                }
            }).catch(err => {
                reject(err)
            })
        })
    },
    /**
       * 获取豆瓣评分
       * @param {*} name 视频名称
       * @param {*} year 视频年份
       * @returns 豆瓣评分
       */
    doubanRate (name, year) {
        return new Promise((resolve, reject) => {
          const nameToSearch = name.replace(/\s/g, '')
          this.doubanLink(nameToSearch, year).then(link => {
            if (link.includes('https://www.douban.com/search')) {
              resolve('暂无评分')
            } else {
                fetch.get(link, null).then(data => {
                    const parsedHtml = load(data)
                    const rating = parsedHtml('body').find('#interest_sectl').first().find('strong').first()
                    if (rating.text()) {
                    resolve(rating.text().replace(/\s/g, ''))
                    } else {
                    resolve('暂无评分')
                    }
                }).catch(err => {
                    reject(err)
                })
            }
          }).catch(err => {
            reject(err)
          })
        })
      },
    
    /**
     * 获取豆瓣相关视频推荐列表
     * @param {*} name 视频名称
     * @param {*} year 视频年份
     * @returns 豆瓣相关视频推荐列表
     */
    doubanRecommendations (name, year) {
        return new Promise((resolve, reject) => {
            const nameToSearch = name.replace(/\s/g, '')
            const recommendations = []
            this.doubanLink(nameToSearch, year).then(link => {
                if (link.includes('https://www.douban.com/search')) {
                    resolve(recommendations)
                } else {
                    fetch.get(link, null).then(data => {
                        const $ = load(data)
                        $('div.recommendations-bd').find('div>dl>dd>a').each(function (index, element) {
                        recommendations.push($(element).text())
                    })
                    resolve(recommendations)
                }).catch(err => {
                    reject(err)
                })
                }
            }).catch(err => {
                reject(err)
            })
        })
    },
};