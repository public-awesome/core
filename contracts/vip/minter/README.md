# Stargaze VIP Minter

This contract is responsible for writing to the Stargaze VIP Collection contract.

Upon instantiation, a collection contract is also instantiated via `instantiate2`. The collection address is derived from the minter contract address, the collection contract code hash, and a salt. It is outputted as an attribute when the contract is instantiated and saved in `Config`.
