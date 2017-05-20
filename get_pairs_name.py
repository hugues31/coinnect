# This python3 script displays all pairs that can be used
# on Kraken, Poloniex and Bitstamp platform. The pairs can then be copied/pasted
# into Coinnect. This script does the conversion for Poloniex (see Pair doc).

import json
import ssl
import urllib.request

# ugly fix the ssl certificate bug
ssl._create_default_https_context = ssl._create_unverified_context


# ╔╗   ╦  ╔╦╗  ╔═╗  ╔╦╗  ╔═╗  ╔╦╗  ╔═╗
# ╠╩╗  ║   ║   ╚═╗   ║   ╠═╣  ║║║  ╠═╝
# ╚═╝  ╩   ╩   ╚═╝   ╩   ╩ ╩  ╩ ╩  ╩
raw_bitstamp_pairs = ["btcusd", "btceur", "eurusd", "xrpusd", "xrpeur",
"xrpbtc"]
standardized_bitstamp_pairs = ["BTC_USD", "BTC_EUR", "EUR_USD", "XRP_USD",
"XRP_EUR", "XRP_BTC"]

# ╦╔═  ╦═╗  ╔═╗  ╦╔═  ╔═╗  ╔╗╔
# ╠╩╗  ╠╦╝  ╠═╣  ╠╩╗  ║╣   ║║║
# ╩ ╩  ╩╚═  ╩ ╩  ╩ ╩  ╚═╝  ╝╚╝
url = "https://api.kraken.com/0/public/AssetPairs"
raw_kraken_pairs = list()
standardized_kraken_pairs = list()
with urllib.request.urlopen(url) as response:
    html = response.read().decode("utf-8")
    json_data = json.loads(html)
    for currency in json_data["result"]:
        raw_kraken_pairs.append(currency)
        quote = json_data["result"][currency]["quote"][1:] # remove the X or Z
        base = json_data["result"][currency]["base"]
        old_naming = ("XETH", "XXBT", "XETC", "XLTC", "XICN", "XREP", "XXDG",
        "XZEC", "XXLM", "XXMR", "XMLN", "XXRP")
        if base in old_naming:
            base = base[1:] # remove the X
        if base == "XBT":
            base = "BTC"
        if quote == "XBT":
            quote = "BTC"
        if json_data["result"][currency]["altname"][-2:] == ".d":
            quote += "_d"
        standardized_kraken_pairs.append(base + "_" + quote)


# ╔═╗  ╔═╗  ╦    ╔═╗  ╔╗╔  ╦  ╔═╗  ═╗ ╦
# ╠═╝  ║ ║  ║    ║ ║  ║║║  ║  ║╣   ╔╩╦╝
# ╩    ╚═╝  ╩═╝  ╚═╝  ╝╚╝  ╩  ╚═╝  ╩ ╚═
url =  "https://poloniex.com/public?command=returnTicker"

raw_poloniex_pairs = list()
with urllib.request.urlopen(url) as response:
    html = response.read().decode("utf-8")
    json_data = json.loads(html)
    for currency in json_data:
        raw_poloniex_pairs.append(currency)

# conversion
standardized_poloniex_pairs = list()
for pair in raw_poloniex_pairs:
    base, quote = pair.split('_', 1)
    standardized_poloniex_pairs.append(quote + "_" + base)

# Generate all possible pairs
exchanges = [standardized_bitstamp_pairs, standardized_kraken_pairs,
standardized_poloniex_pairs]

pairs = list()
for exchange in exchanges:
    for pair in exchange:
        if pair not in pairs:
            pairs.append(pair)
pairs = sorted(pairs)

print("SUPPORTED PAIRS")
print("===============")
for pair in pairs:
    print(pair + ",")

print("\n\n\n")
print("BITSTAMP PAIRS")
print("==============")
for std, raw in zip(standardized_bitstamp_pairs, raw_bitstamp_pairs):
    print("m.insert({std}, \"{raw}\");".format(std=std, raw=raw))

print("\n\n\n")
print("KRAKEN PAIRS")
print("============")
for std, raw in zip(standardized_kraken_pairs, raw_kraken_pairs):
    print("m.insert({std}, \"{raw}\");".format(std=std, raw=raw))

print("\n\n\n")
print("POLONIEX PAIRS")
print("==============")
for std, raw in zip(standardized_poloniex_pairs, raw_poloniex_pairs):
    print("m.insert({std}, \"{raw}\");".format(std=std, raw=raw))
