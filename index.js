module.exports = config;

const fs = require('fs');
const path = require('path');

/**
 * config() Return a country's tokens or if not specified all tokens available
 *
 * @param {String} country [optional] ISO 639-1 Code - If not specified return object of all codes
 *
 * @return {Array|Object} Return an array for a single country or an object map of all tokens by ISO code
 */
function config(country) {
    if (country && (typeof country !== 'string' || country.length != 2)) throw Error('optional country param must be string containing 2 letter ISO 639-1 Code');

    if (country) {
        if (!fs.statSync(path.resolve(__dirname, './tokens/'))) return [];

        return require(`./tokens/${country}.json`);
    }

    const tokens = {};

    fs.readdirSync(path.resolve(__dirname, './tokens/')).forEach((token) => {
        if (!token.match(/\.json$/)) return;

        let json = require(`./tokens/${token}`);

        tokens[token.replace(/\.json/, '')] = json;
    });

    return tokens;
}
