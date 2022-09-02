tokens = [
    "autovía|autovia",
"autopista|auto|autop",
"avenida|av|avd|avda",
"alameda",
"baixada",
"camiño|camino",
"costa",
"encosta",
"estrada",
"glorieta",
"paseo",
"praza|pr|pza",
"praciña|pracina",
"prazuela|przla",
"rambla|rbla",
"ronda|rda",
"rotonda|rtda",
"rúa|rua|r|ru",
"rúas|ruas|rs",
"ruela",
"sendeiro",
"suba",
"subida|sbida",
"travesía|travesia|trav",
"vía|via",
"viela"
]

_tokens = open(
        f"/Users/ilissablech/Documents/mapbox-code/geocoder-abbreviations/tokens/{country}.json"
    )
    
full = []
for token in tokens:
    _tokens = token.split("|")
    t = {
        "tokens": _tokens,
        "full": max(_tokens, key = len) ,
        "canonical": min(_tokens, key = len),
        "type": "way"
    }
    full.append(t)
print(full)
