import { denom } from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP } from '../setup/context'
import { getClient } from '../utils/client'
import { contracts } from '@stargazezone/core-types'
import _ from 'lodash'

const { FairBurnClient, FairBurnQueryClient } = contracts.FairBurn

describe('FairBurn', () => {
  const creatorName = 'user1'
  const burnerName = 'user2'
  const developerName = 'user3'

  let context: Context
  let fairBurnAddress: string

  beforeAll(async () => {
    context = new Context()
    await context.initializeTestUsers()
    await context.hydrateContext()

    fairBurnAddress = context.getContractAddress(CONTRACT_MAP.FAIR_BURN)
  })

  test('execute fair burn message', async () => {
    const burner = context.testUserMap[burnerName]
    const developer = context.testUserMap[developerName]

    const queryClient = await getClient()

    const burnerFairBurnSigningClient = new FairBurnClient(burner.client, burner.address, fairBurnAddress)
    const burnerFairBurnQueryClient = new FairBurnQueryClient(burner.client, fairBurnAddress)

    const fairBurnConfig = await burnerFairBurnQueryClient.config()

    const burnerBalanceBefore = await queryClient.getBalance(burner.address, denom)
    const developerBalanceBefore = await queryClient.getBalance(developer.address, denom)

    let fairBurnCoin = { amount: '1000000', denom }
    let response = await burnerFairBurnSigningClient.fairBurn({ recipient: developer.address }, 'auto', 'fair-burn', [
      fairBurnCoin,
    ])
    expect(response).toBeTruthy()

    const burnerBalanceAfter = await queryClient.getBalance(burner.address, denom)
    const developerBalanceAfter = await queryClient.getBalance(developer.address, denom)

    // Greater than or equal because of gas fees
    expect(parseInt(burnerBalanceBefore.amount, 10) - parseInt(fairBurnCoin.amount, 10)).toBeGreaterThanOrEqual(
      parseInt(burnerBalanceAfter.amount, 10),
    )

    const developerPayout = parseInt(fairBurnCoin.amount, 10) * parseFloat(fairBurnConfig.fee_percent)
    expect(parseInt(developerBalanceBefore.amount, 10) + developerPayout).toEqual(
      parseInt(developerBalanceAfter.amount, 10),
    )
  })
})
