var fs = require('fs');
String.prototype.hex2bin = function () {
    var i = 0, l = this.length - 1, bytes = []
    for (i; i < l; i += 2)
        bytes.push(parseInt(this.substr(i, 2), 16))
    return bytes
}
var hex = fs.readFileSync("./pw_lock_mainnet").toString().hex2bin();


console.log(hex)


fs.appendFile('./pw_anyone_can_pay', Buffer.from(hex), function (err) {
    if (err) {
        throw new Error(err);
    } else {
        // console.log(typedArray.length);
    }
});