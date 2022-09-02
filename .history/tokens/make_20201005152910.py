import json

tokens = [
"autopista|auto|autop",
"autovia|autov",
"avinguda|av|avd|avda|avgda",
"camí|cami",
"carrer|carr|c /",
"carrera|cra|carra",
"carreró|cró|carrero|cro|carro|carr",
"carretera|ctra",
"cinturó|cint|cinturo",
"diagonal|diag",
"drecera|drec",
"eix",
"eix diagonal",
"entrada|entr",
"gran vía|gran via|gv|g v",
"passadís|pdís|passadis|pdis",
"passatge|ptge",
"passeig|pg",
"plaça|pl|placa|pça|pca|plç|plc",
"portal|ptal",
"rambla|rbla",
"ronda|rda",
"rotonda|rtda",
"sortida|sort",
"transversal|trval|trvsal",
"travessera|trav",
"travessia|trv|trav",
"via|v"
]


es_tokens = json.load(open(
        "/Users/ilissablech/Documents/mapbox-code/geocoder-abbreviations/tokens/es.json"
    ))
current_list = []
for es in es_tokens:
    current_list.extend(es['tokens'])

for token in tokens:
    _tokens = token.split("|")
    for _ in _tokens:
        if _ in current_list:
            print(_)
        t = {
            "tokens": _tokens,
            "full": max(_tokens, key = len) ,
            "canonical": min(_tokens, key = len),
            "type": "way"
        }
        full.append(t)
print(full)
