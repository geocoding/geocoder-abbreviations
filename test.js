const fs = require('fs');
const path = require('path');
const tape = require('tape');

const config = require('./index');

tape((t) => {
    fs.readdir(__dirname + '/tokens/', (err, files) => {
        t.error(err);

        files.forEach((file) => {
            let country = path.basename(file, '.json');
            let cc = config()

            if (cc.tokens) {
                if (!Array.isArray(cc.tokens))
                    cc.tokens = [ cc.tokens ];
                cc.tokens.forEach((token) => {
                    t.ok(config.tokens(token).length > 0, 'valid tokens found for ' + country + ', ' + token);
                });
            }
            else
                t.ok('no tokens specified for ' + country);
        });
        t.end();
    });
});
