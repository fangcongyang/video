export default {
    trimAll(ele) {
        if (typeof ele === 'string') {
            return ele.split(/[\t\r\f\n\s]*/g).join('');
        } else {
            console.error("`${type of ele}` is not a string")
        }
    }
}