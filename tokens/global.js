module.exports = () => {
    let tokens = {};

    //Prepend Standard word boundaries + ^
    //Postpend Standard word boundaries + $
    let stdBoundaries = {
        "P\\.?\\ ?O\\.? Box [0-9]+": " ",
        "(.+)(strasse|str|straße)": " $1 str ",
        "(Apartment|Apt) [0-9]+": " ",
        "(Boatshed|Btsd) [0-9]+": " ",
        "(Building|Bldg) [0-9]+": " ",
        "(Bungalow|Bngw) [0-9]+": " ",
        "(Cabin|Cbin) [0-9]+": " ",
        "(Carspace|Cars) [0-9]+": " ",
        "(Cottage|Ctge) [0-9]+": " ",
        "(Duplex|Dupl) [0-9]+": " ",
        "(Factory|Fcty) [0-9]+": " ",
        "(Flat) [0-9]+": " ",
        "(Garage|Grge) [0-9]+": " ",
        "(Hall) [0-9]+": " ",
        "(House|Hse) [0-9]+": " ",
        "(Kiosk|Ksk) [0-9]+": " ",
        "(Lobby|Lbby) [0-9]+": " ",
        "(Loft) [0-9]+": " ",
        "(Marine Berth|Mbth) [0-9]+": " ",
        "(Office|Offc) [0-9]+": " ",
        "(Penthouse|Ph) [0-9]+": " ",
        "(Reserve|Resv) [0-9]+": " ",
        "(Room) [0-9]+": " ",
        "(Shed) [0-9]+": " ",
        "(Shop) [0-9]+": " ",
        "(Site) [0-9]+": " ",
        "(Store|Storeroom|Stor) [0-9]+": " ",
        "(Strata unit|Str) [0-9]+": " ",
        "(Substation|Subs) [0-9]+": " ",
        "Unit [0-9]+": " ",
        "(Villa|Vlla) [0-9]+": " ",
        "(Warehouse|Whse) [0-9]+": " ",
        "(Workshop|Wksh) [0-9]+": " ",
        "Suite [0-9]+": " ",
        "Suite [0-9]+-[0-9]+": " ",
        "Suite [0-9]+[a-z]": " ",
        "Suite [a-z]": " ",
        "STE [0-9]+": " ",
        "STE [a-z]": " ",
        "SE [0-9]+": " ",
        "SE [a-z]": " ",
        "Post Office": " Po ",
        "Railway Station": " Rs ",
        "Village Post Office": " Vpo ",
        "Camino hondo": " CH ",
        "Camino nuevo": " CN ",
        "Camino viejo": " CV ",
        "Gran Vía": " GV ",
        "Pgind": " PI ",
        "Polígono industrial": " PI ",
        "Punto kilométrico": " pk ",
        "Paseo marítimo": " Psmar ",
        "Boîte Postale": " BP ",
        "Centre Commercial": " Ccal ",
        "Route Européenne": " E ",
        "Route Nationale": " RN ",
        "Zone d'activité": " Za ",
        "N.T.": " NT ",
        "New Territories": " NT ",
        "Strada Comunale": " SC ",
        "Strada Provinciale": " SP ",
        "Strada Regionale": " SR ",
        "Strada Statale": " SS ",
        "Van De": " vd ",
        "Van Den": " vd ",
        "Van Der": " vd ",
        "Câmara Municipal": " CM "
    };

    const BOUNDARIES = "[\\s\\u2000-\\u206F\\u2E00-\\u2E7F\\\\'!\"#$%&()*+,\\-.\\/:;<=>?@\\[\\]^_`{|}~]"

    for (let find in stdBoundaries) {
        tokens[`(?:${BOUNDARIES}|^)${find}(?:${BOUNDARIES}|$)`] = stdBoundaries[find];
    }

    let custBoundaries = {
        "\\b1丁目\\b": " 一丁目 ",
        "\\b2丁目\\b": " 二丁目 ",
        "\\b3丁目\\b": " 三丁目 ",
        "\\b4丁目\\b": " 四丁目 ",
        "\\b5丁目\\b": " 五丁目 ",
        "\\b6丁目\\b": " 六丁目 ",
        "\\b7丁目\\b": " 七丁目 ",
        "\\b8丁目\\b": " 八丁目 ",
        "\\b9丁目\\b": " 九丁目 ",
        "\\b10丁目\\b": " 十丁目 ",
        "^([\\u1100-\\u11FF\\u2E80-\\u2EFF\\u3000-\\u303F\\u3040-\\u309F\\u30A0-\\u30FF\\u3100-\\u312F\\u3130-\\u318F\\u31C0-\\u31EF\\u31F0-\\u31FF\\u3200-\\u32FF\\u3300-\\u33FF\\u3400-\\u4DBF\\u4E00-\\u9FFF\\uF900-\\uFAFF\\uFE30-\\uFE4F\\s]+)(\\s*)([0-9]+)-([0-9]+)(.*)$": "$1$5 $3"
    }

    for (let find in custBoundaries) {
        tokens[find] = custBoundaries[find];
    }

    return tokens;
}
