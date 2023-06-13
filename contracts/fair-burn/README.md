# Stargaze Fair Burn

The Stargaze Fair Burn contract is responsible for handlintg fees paid by other contracts. Fees can be paid in any denom. The Fair Burn contract performs the following logic:

- If the funds transferred are in STARS, then a percentage of the funds are burned, and the remaining funds are sent either to the treasury, or a specified recipient address.
- If the funds transferred are not in STARS, then a percentage of the funds are sent to the treasury, and the remaining funds are sent either to the treasury, or a specified recipient address.
