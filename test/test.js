const fs = require('fs');
const path = require('path');
const tape = require('tape');

const config = require('..');

const tokenize = require('./util').tokenize;

tape((t) => {
    t.throws(() => {
        config([]);
    }, /optional lang param must be string/);

    t.throws(() => {
        config('adsg');
    }, /optional lang param must be string/);

    t.throws(() => {
        config(2355);
    }, /optional lang param must be string/);

    t.throws(() => {
        config({});
    }, /optional lang param must be string/);

    t.end();
}, 'invalid lang input');

tape((t) => {
    fs.readdir(__dirname + '/../tokens/', (err, files) => {
        t.error(err);

        files.forEach((file) => {
            if (!file.match(/\.json/) || file.match(/global/)) return;

            let lang = path.basename(file, '.json');
            let tokens = config(lang);

            if (!Array.isArray(tokens)) t.fail(`${lang} tokens must be an array`);
            if (!tokens.length > 0) t.fail(`${lang} tokens must have at least 1 token`);
        });
        t.end();
    });
}, 'each simple-form lang can be accessed');

tape((t) => {
    fs.readdir(__dirname + '/../tokens/', (err, files) => {
        t.error(err);

        files.forEach((file) => {
            if (!file.match(/\.json/) || file.match(/global/)) return;

            let lang = path.basename(file, '.json');
            let tokens = config(lang, true, true);

            if (!Array.isArray(tokens)) t.fail(`${lang} tokens must be an array`);
            if (!tokens.length > 0) t.fail(`${lang} tokens must have at least 1 token`);

            const props = {
                tokens: { type: 'array', required: true },
                full: { type: 'string', required: true },
                canonical: { type: 'string', required: true },

                note: { type: 'string', required: false },
                onlyCountries: { type: 'array', required: false },
                onlyLayers: { type: 'array', required: false, allowed: [ 'address' ] },
                onlyUseWhile: { type: 'array', required: false, allowed: [ 'processing', 'indexing', 'querying' ] },
                preferFull: { type: 'boolean', required: false },
                regex: { type: 'boolean', required: false },
                reduceRelevance: { type: 'boolean', required: false },
                skipBoundaries: { type: 'boolean', required: false },
                skipDiacriticStripping: { type: 'boolean', required: false },
                spanBoundaries: { type: 'number', required: false },
                type: { type: 'string', required: false, allowed: [ 'box', 'cardinal', 'number', 'ordinal', 'unit', 'way', 'determiner' ] }
            }

            for (const group of tokens) {
                for (const key of Object.keys(props)) {
                    const attributes = props[key];
                    const actualType = typeof group[key];
                    if (actualType === 'undefined') {
                        if (attributes.required) t.fail(`${lang} group ${JSON.stringify(group)} is missing property ${key}`);
                        continue;
                    }

                    const typeMatch = attributes.type === 'array' ? Array.isArray(group[key]) : actualType === attributes.type;
                    if (!typeMatch) t.fail(`${lang} group ${JSON.stringify(group)} property ${key} should be type ${attributes.type}; found ${actualType}`);

                    if (attributes.allowed) {
                        const toCheck = attributes.type === 'array' ? group[key] : [group[key]];
                        for (const item of toCheck) {
                            if (attributes.allowed.indexOf(item) === -1) {
                                t.fail(`${lang} group ${JSON.stringify(group)} property ${key} should be one of ${JSON.stringify(attributes.allowed)}; found ${item}`);
                            }
                        }
                    }
                }
                for (const key of Object.keys(group)) {
                    if (typeof props[key] !== 'object') {
                        t.fail(`${lang} group ${JSON.stringify(group)} has unexpected property ${key}`);
                    }
                }

                const boundariesLengths = new Set();
                for (const phrase of group.tokens) {
                    // carmen does a very naive split if we don't specify how to split (just splits on spaces)
                    // so make sure that if we don't specify, it comes out right
                    const boundaries = tokenize(phrase).tokens.length - 1;
                    const splitOnSpaces = phrase.split(' ').length - 1;
                    if (phrase != group.canonical) {
                        if (boundaries > 0 && !group.regex) {
                            if (group.spanBoundaries) {
                                // all phrases except the canonical one must have the same number of tokens as the group's spanBoundaries property
                                t.equals(boundaries, group.spanBoundaries, `correct token count in ${phrase}`);
                            } else {
                                // if no spanBoundaries property is specific, Carmen does a simple space-split, but that needs to produce a sane number of tokens
                                t.equals(boundaries, splitOnSpaces, `real tokenization and simple space split must match for phrase ${phrase}`);
                            }
                        }
                        boundariesLengths.add(boundaries);
                    }
                }
                if (group.tokens.length > 1 && boundariesLengths.size !== 1) {
                    t.fail(`all phrases except the canonical one must span the same number of boundaries: ${JSON.stringify(group.tokens)}`);
                }
            }
        });
        t.end();
    });
}, 'each complex-form lang can be accessed');

tape((t) => {
    let tokens = config();

    t.equals(typeof tokens.global, 'object');

    for (let token in tokens.global) {
        t.ok(new RegExp(token))
    }

    Object.keys(tokens).forEach((token) => {
        if (token === 'global') return;

        t.equals(token.length, 2, 'token is length 2');
        t.ok(Array.isArray(tokens[token]), 'token refs array');
        t.ok(tokens[token].length > 0, 'array has length');
    });

    t.end();
}, 'return all countries')

tape((t) => {
    if (process.env.UPDATE) {
        fs.writeFileSync(__dirname + '/fixtures/et-no-singletons.json', JSON.stringify(config('et')));
        fs.writeFileSync(__dirname + '/fixtures/et-singletons.json', JSON.stringify(config('et', true)));
        fs.writeFileSync(__dirname + '/fixtures/de-simple.json', JSON.stringify(config('de', false)));
        fs.writeFileSync(__dirname + '/fixtures/de-advanced.json', JSON.stringify(config('de', false, true)));
        t.fail('updated fixtures');
    } else {
        t.deepEquals(config('et'), require(__dirname + '/fixtures/et-no-singletons.json', 'singletons off'));
        t.deepEquals(config('et', true), require(__dirname + '/fixtures/et-singletons.json', 'singletons on'));
        t.deepEquals(config('de', false), require(__dirname + '/fixtures/de-simple.json', 'de simple (tests combining)'));
        t.deepEquals(config('de', false, true), require(__dirname + '/fixtures/de-advanced.json', 'de advanced'));
    }
    t.end();
}, 'singletons');
