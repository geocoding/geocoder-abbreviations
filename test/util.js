'use strict';

module.exports.tokenize = tokenize;

function removeEmoji(str) {
    return str.replace(/([#0-9]\u20E3)|[\xA9\xAE\u203C\u2047-\u2049\u2122\u2139\u3030\u303D\u3297\u3299][\uFE00-\uFEFF]?|[\u2190-\u21FF][\uFE00-\uFEFF]?|[\u2300-\u23FF][\uFE00-\uFEFF]?|[\u2460-\u24FF][\uFE00-\uFEFF]?|[\u25A0-\u25FF][\uFE00-\uFEFF]?|[\u2600-\u27BF][\uFE00-\uFEFF]?|[\u2900-\u297F][\uFE00-\uFEFF]?|[\u2B00-\u2BF0][\uFE00-\uFEFF]?|(?:\uD83C[\uDC00-\uDFFF]|\uD83D[\uDC00-\uDEFF])[\uFE00-\uFEFF]?/g, '');
}

// Split queries based on other ascii and unicode punctuation.
const WORD_SEPARATOR = [
    // Equivalient to \u0020\f\n\r\t\v\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF
    '\\s',

    // \u2000 - \u206F "General Punctuation"
    '\u2000-\u206F',

    // \u2E00 - \u2E7F "Supplemental Punctuation"
    '\u2E00-\u2E7F',

    // The usual suspects from \u0020 - \u007F "Basic Latin"
    // !"#$%&'()*+-./,
    '\u0021-\u002F',
    // :;<=>?@
    '\u003A-\u0040',
    // [\]^_`
    '\u005B-\u0060',
    // {|}~
    '\u007B-\u007E',

    // Similar symbols from \uFF00 - \uFFEF "Halfwidth and Fullwidth Forms"
    '\uFF01-\uFF0F',
    '\uFF1A-\uFF20',
    '\uFF3B-\uFF40',
    '\uFF5B-\uFF65'

].join('');
module.exports.WORD_SEPARATOR = WORD_SEPARATOR;

function tokenize(query, lonlat) {
    if (lonlat) throw new Error('Unsupported usage of tokenize. Use asReverse instead');

    const tokens = [];
    const separators = [];

    const normalized = query
        .toLowerCase()
        // collapse apostraphes, periods, caret
        .replace(/[\u2018\u2019\u02BC\u02BB\uFF07'\.\^]/g, '')
        // If the query begins with a separators, tear it off.
        .replace(new RegExp(`^[${WORD_SEPARATOR}]+`, 'u'), '');

    const split = new RegExp(`([^${WORD_SEPARATOR}]+)([${WORD_SEPARATOR}]+|$)`, 'yu');
    let part;
    let tail;
    // eslint-disable-next-line no-cond-assign
    while (part = split.exec(normalized)) {
        let t = part[1].toString();
        const s = part[2].toString();

        if (tail) {
            if (tail.s === '-' || tail.s === '/') {
                const combined = `${tail.t}${tail.s}${t}`;
                // Allow numbers like 1-2, 1/2, 1a, 1-2a, 1/2a, 1/2-3b
                if  (/^(\d+)(-|\/)(\d+)((-|\/)(\d+))?[a-z]?$/.test(combined)) {
                    t = combined;
                } else {
                    tokens.push(tail.t);
                    separators.push(tail.s);
                }
            } else {
                tokens.push(tail.t);
                separators.push(tail.s);
            }
        }
        tail = false;

        if (t.length === 0) continue;
        if (removeEmoji(t).length === 0) continue;

        // \u4E00 - \u9FFF "CJK Unified Ideographs" characters are indexed
        // individually to support addresses being written from largest to
        // smallest geographical entity without delimiters. Adjacent numbers,
        // normal and full-width, are not split.
        const subtoken = t.split(/([\u4E00-\u9FFF])/u);
        if (subtoken.length > 1) {
            for (let l = 0; l < subtoken.length; l++) {
                if (subtoken[l].length > 0) {
                    tokens.push(subtoken[l]);
                    separators.push('');
                }
            }
            continue;
        }

        // In some cases we want to combine two tokens.
        if (s === '-' || s === '/') {
            tail = { t, s };
        } else {
            tokens.push(t);
            separators.push(s);
        }
    }

    if (tail) {
        tokens.push(tail.t);
        separators.push(tail.s);
    }

    const owner = new Array(tokens.length);
    for (let i = 0; i < owner.length; i++) owner[i] = i;

    return { tokens, separators, owner, lastWord: false };
}
