import qs from 'qs';
import {http} from "@tauri-apps/api";
import axios from 'axios';

axios.defaults.headers.post['Content-Type'] = 'application/x-www-form-urlencoded;charset=UTF-8'
axios.defaults.withCredentials = true

//axios 请求
export default {
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
    localGet(url, params, withTimestamp = false, timeout = 10) {
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
            axios.get(url, {
                headers: {
                    'Content-Type': 'application/text',
                },
                timeout: timeout,
                responseType: "text"
            }).then(res => {
                resolve(res.data)
            }).catch((error) => {
                reject(error)
            })
        })
    },
}