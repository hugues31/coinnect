TODO
====

- [ ] Add tests for Poloniex and Kraken
- [ ] Remove 'unsafe' unwrap() and replace with error handling -> Result<>
- [ ] Implement two-factor auth for supported exchanges
- [ ] Add Option type for optional parameter (Pair for return_balances for example : if Pair is specified, get balance for this Pair, if Pair is None, get all balances (if possible))
- [ ] Add links to the documentation (Kraken use external links for example)
- [ ] Remove .clone() for params in Kraken & Poloniex
