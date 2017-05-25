const fs = require('fs');
const path = require('path');
const tape = require('tape');

const config = require('..');

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
}, 'each lang can be accessed');

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
        t.fail('updated fixtures');
    } else {
        t.deepEquals(config('et'), require(__dirname + '/fixtures/et-no-singletons.json', 'singletons off'));
        t.deepEquals(config('et', true), require(__dirname + '/fixtures/et-singletons.json', 'singletons on'));
    }
    t.end();
}, 'singletons');
