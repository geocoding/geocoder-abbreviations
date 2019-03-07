module.exports = () => {
    const out = {};

    // this is all the characters we expect in Japanese text
    const JP = '\\u1100-\\u11FF\\u2E80-\\u2EFF\\u3000-\\u318F\\u31C0-\\u4DBF\\u4E00-\\u9FFF\\uF900-\\uFAFF\\uFE30-\\uFE4F';
    // this matches a string that can either start with a single Japanese character, or a thing that starts and ends
    // with a Japanese character and has zero or more Japanese characters, numbers of either Latin or Japanese flavor,
    // or whitespace in the middle. That whole mess is then followed optionally by space, and then number hyphen number,
    // followed by whatever. We transform it by keeping the whole Japanese chunk, dropping the second number, and moving
    // the first number to the end (effectively: deletes the building number, and moves the block number into the building-
    // number position)
    out[`^([${JP}](?:[${JP}0-9\\uFF10-\\uFF19\\s]*[${JP}])?)(?:\\s*)([0-9]+)-(?:[0-9]+)(.*)$`] = "$1$3 $2";

    return out;
}
