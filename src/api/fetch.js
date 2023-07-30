import qs from 'qs';
import {http} from "@tauri-apps/api";

//axios 请求
export default {
    postJson(url, params) {
        return new Promise((resolve, reject) => {
            axios.post(url,  qs.stringify(params))
                .then(response => {
                    if (response) {
                        resolve(response)
                    } else {
                        reject('请求超时')
                    }
                   
                }, err => {
                    reject(err)
                })
                .catch((error) => {
                    reject(error)
                })
        })
    },
    postFormData(url, params) {
        return new Promise((resolve, reject) => {
            axios.post(url,  params)
                .then(response => {
                    if(response){
                        resolve(response)
                    }
                    else{
                        reject('请求超时')
                    }
                   
                }, err => {
                    reject(err)
                })
                .catch((error) => {
                    reject(error)
                })
        })
    },
    get(url, params, withTimestamp = false, timeout = 10) {
        return new Promise((resolve, reject) => {
            if (withTimestamp) {
                let date = new Date().getTime()
                if (params) {
                    params['t'] = date
                } else {
                    params = {}
                    params['t'] = date
                }
            }
            let pp = qs.stringify(params);
            if (pp) url += '?' + pp;
            http.fetch(url, {
                headers: {
                    "User-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.58",
                    'Content-Type': 'application/text',
                },
                method: 'GET',
                timeout: timeout,
                responseType: http.ResponseType.Text
            }).then(res => {
                resolve(res.data)
            }).catch((error) => {
                reject(error)
            })
        })
    },
    upload(url, params){
        return new Promise((resolve, reject) => {
            axios.post(url,  params)
                .then(response => {
                    if(response){
                        resolve(response.data)
                    }
                    else{
                        reject('请求超时')
                    }                 
                }, err => {
                    reject(err)
                })
                .catch((error) => {
                    reject(error)
                })
        })
    }
}