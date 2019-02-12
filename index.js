module.exports = config;

const fs = require('fs');
const path = require('path');
const UnionFind = require('union-find');

/**
 * config() Return a Language's tokens or if not specified all tokens available
 *
 * @param {String} lang [optional] ISO 639-1 Code - If not specified return object of all codes
 * @param {Boolean} singletons [optional] whether to include single-entry abbreviation list items. These are not used for substitution but can be useful for string comparison. Defaults to false.
 * @param {Boolean} advanced [optional] whether to opt into a more complex representation of the tokens than a list of lists, which can represent per-replacement configuration, etc.
 *
 * @return {Array|Object} Return an array for a single lang tokens or an object map of all tokens by ISO code
 */
function config(lang, singletons, advanced) {
    singletons = !!singletons;
    advanced = !!advanced;

    if (lang && (typeof lang !== 'string' || lang.length != 2)) throw Error('optional lang param must be string containing 2 letter ISO 639-1 Code');

    if (lang) {
        if (!fs.statSync(path.resolve(__dirname, `./tokens/${lang}.json`))) {
            if (!fs.statSync(path.resolve(__dirname, `./tokens/${lang}.json`))) {
                return [];
            } else {
                let tokenjs = require(`./tokens/${lang}`);

                return prepare(tokenjs(), singletons, advanced);
            }
        } else {
            let tokenjson = require(`./tokens/${lang}.json`);

            return prepare(tokenjson, singletons, advanced);
        }
    }

    const tokens = {};

    fs.readdirSync(path.resolve(__dirname, './tokens/')).forEach((token) => {
        if (token.match(/\.json$/)) {
            let json = require(`./tokens/${token}`);

            tokens[token.replace(/\.json/, '')] = prepare(json, singletons, advanced);
        } else if (token.match(/\.js$/)) {
            let js = require(`./tokens/${token.replace('\.js$', '')}`);

            tokens[token.replace(/\.js/, '')] = prepare(js(), singletons, advanced);
        } else {
            return;
        }
    });

    return tokens;
}

function prepare(data, singletons, advanced) {
    if (singletons) data = removeSingletons(data);
    if (!advanced) data = simplify(data);
    return data;
}

function simplify(data) {
    // the advanced representation differs from the old-school list of lists in a couple of ways:
    // * the same token may occur in more than one group
    // * groups contain additional configuration options
    // to get the old-style form, we want to extract just the tokens, and also merge the groups

    // only bother if the data is shaped the way we expect:
    if (!data.length || !data[0].tokens) return data;

    let tokens = new Set();
    for (let group of data) {
        // groups with skipDiacriticStripping or skipBoundaries enabled don't make sense without
        // those config values, so skip if we're stripping config
        if (group.skipDiacriticStripping || group.skipBoundaries) continue;

        for (let token of group.tokens) {
            tokens.add(token);
        }
    }
    tokens = Array.from(tokens).sort();
    let invTokens = new Map();
    tokens.forEach((v, i) => { invTokens.set(v, i); });

    let uf = new UnionFind(tokens.length);
    for (let group of data) {
        let idx1 = invTokens.get(group.tokens[0]);
        for (let token of group.tokens.slice(1)) {
            let idx2 = invTokens.get(token);
            uf.link(idx1, idx2);
        }
    }

    let out = [];

    let groups = Array.from(new Set(uf.roots)).sort((a, b) => a - b);
    let invGroups = new Map();
    groups.forEach((v, i) => { invGroups.set(v, i); });

    for (let g = 0; g < groups.length; g++) out[g] = [];
    for (let i = 0; i < tokens.length; i++) {
        out[invGroups.get(uf.roots[i])].push(tokens[i]);
    }
    out.forEach((arr) => { arr.sort((a, b) => a.length - b.length) });

    return out;
}

function removeSingletons(tokens) {
    if (!(tokens instanceof Array)) return tokens;

    return tokens.filter((token) => {
        return
            (token instanceof Array && token.length > 1) ||
            (token.tokens instanceof Array && token.tokens.length > 1);
    });
}
