module.exports = config;

const fs = require('fs');
const path = require('path');

/**
 * config() Return a Language's tokens or if not specified all tokens available
 *
 * @param {String} lang [optional] ISO 639-1 Code - If not specified return object of all codes
 *
 * @return {Array|Object} Return an array for a single lang tokens or an object map of all tokens by ISO code
 */
function config(lang) {
    if (lang && (typeof lang !== 'string' || lang.length != 2)) throw Error('optional lang param must be string containing 2 letter ISO 639-1 Code');

    if (lang) {
        if (!fs.statSync(path.resolve(__dirname, './tokens/'))) return [];

        return require(`./tokens/${lang}.json`);
    }

    const tokens = {};

    fs.readdirSync(path.resolve(__dirname, './tokens/')).forEach((token) => {
        if (!token.match(/\.json$/)) return;

        let json = require(`./tokens/${token}`);

        tokens[token.replace(/\.json/, '')] = json;
    });

    return tokens;
}
