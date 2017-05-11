const fs = require('fs');
const path = require('path');
const tape = require('tape');

const config = require('./index');

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
    fs.readdir(__dirname + '/tokens/', (err, files) => {
        t.error(err);

        files.forEach((file) => {
            if (!file.match(/\.json/)) return;

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

    Object.keys(tokens).forEach((token) => {
        t.equals(token.length, 2, 'token is length 2');
        t.ok(Array.isArray(tokens[token]), 'token refs array');
        t.ok(tokens[token].length > 0, 'array has length');
    });

    t.end();
}, 'return all countries')
