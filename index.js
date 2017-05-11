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
    if (country) return require(`./tokens/${token}.json`);

    const tokens = {};

    fs.readdirSync(path.resolve(__dirname, './tokens/')).forEach((token) => {
        if (!token.match(/\.json$/)) return;

        let json = require(`./tokens/${token}`);

        tokens[token.replace(/\.json/, '')] = json;
    });

    return tokens;
}
