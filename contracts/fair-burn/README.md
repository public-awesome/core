# Stargaze Fair Burn

The Stargaze Fair Burn contract is responsible for handlintg fees paid by other contracts. Fees can be paid in any denom. The Fair Burn contract performs the following logic:

- If the funds transferred are in STARS, then a percentage of the funds are burned, and the remaining funds are sent either to the treasury, or a specified recipient address.
- If the funds transferred are not in STARS, then a percentage of the funds are sent to the treasury, and the remaining funds are sent either to the treasury, or a specified recipient address.
- In both cases there is a deductible amount of fees that must be paid inbefore funds can be distributed to the treasury or recipient address. The deductible amount is used to ensure some STARS are burned on each message. For example, with a 10% fee percentage, and a payment of 9 STARS, 0 STARS would be burned (9 \* .1 = 0.9 = 0). A deductible amount of 1 STARS would ensure that at least 1 STARS is burned.
