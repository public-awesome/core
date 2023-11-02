import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { denom } from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP, TestUser } from '../setup/context'
import { getQueryClient } from '../utils/client'
import { ArrayOfUint128, ExecuteMsg as MinterExecuteMsg } from '../types/minter.types'
import { MinterQueryClient } from '../types/minter.client'
import { CollectionQueryClient } from '../types/collection.client'
import _ from 'lodash'
import { coin } from '@cosmjs/proto-signing'
import { StakedTokensAPIResponse } from '../utils/stake'

describe('Mint Loyalty Program Tokens', () => {
  // userOne is the minter contract owner
  const userOne = 'user1'
  const userTwo = 'user2'

  let context: Context
  let queryClient: CosmWasmClient
  let minterQueryClient: MinterQueryClient
  let collectionQueryClient: CollectionQueryClient
  let minterAddress: string
  let collectionAddress: string
  let tiers: ArrayOfUint128
  let testUserOne: TestUser
  let testUserTwo: TestUser

  beforeAll(async () => {
    context = new Context()
    await context.initialize(true)
    minterAddress = context.getContractAddress(CONTRACT_MAP.VIP_MINTER)
    collectionAddress = context.getContractAddress(CONTRACT_MAP.VIP_COLLECTION)

    queryClient = await getQueryClient()

    minterQueryClient = new MinterQueryClient(queryClient, minterAddress)
    collectionQueryClient = new CollectionQueryClient(queryClient, collectionAddress)
    tiers = await minterQueryClient.tiers()
    testUserOne = context.getTestUser(userOne)
    testUserTwo = context.getTestUser(userTwo)
  })

  test('Mint Initial Token', async () => {
    const mintMsg: MinterExecuteMsg = {
      mint: {},
    }

    const executionResult = await testUserOne.client.execute(
      testUserOne.address,
      minterAddress,
      mintMsg,
      'auto',
      'mint loyalty program token',
    )

    _.forEach(executionResult.events, (event) => {
      if (event.type === 'wasm') {
        const attributes = _.keyBy(event.attributes, 'key')
        const tokenID = attributes['token_id'].value
        expect(tokenID).toBe('1')
      }
    })

    let userOneTier = await minterQueryClient.tier({ address: testUserOne.address })
    expect(userOneTier).toBe(0)

    const tokenMetadata = await collectionQueryClient.nftInfo({ tokenId: "1" })
    const stakedTokensResponse: Response = await fetch(
      `https://rest.elgafar-1.stargaze-apis.com/cosmos/staking/v1beta1/delegations/${testUserOne.address}`,
    )
    const stakedTokens: StakedTokensAPIResponse = await stakedTokensResponse.json()
    const stakedAmount = stakedTokens.delegation_responses[0]?.balance?.amount
    expect(tokenMetadata.extension.staked_amount).toBe(stakedAmount)
  })

  test('Update Token', async () => {
    await testUserOne.client.delegateTokens(
      testUserOne.address,
      'starsvaloper1jt9w26mpxxjsk63mvd4m2ynj0af09cslura0ec',
      coin(1000, 'ustars'),
      'auto',
      'delegate tokens',
    )

    const updateMsg: MinterExecuteMsg = {
      update: {
        token_id: 1,
      }
    }

    await testUserOne.client.execute(
      testUserOne.address,
      minterAddress,
      updateMsg,
      'auto',
      'update token',
    )
    
    let userOneTier = await minterQueryClient.tier({ address: testUserOne.address })
    expect(userOneTier).toBe(1)

    const tokenMetadata = await collectionQueryClient.nftInfo({ tokenId: "1" })
    const stakedTokensResponse: Response = await fetch(
      `https://rest.elgafar-1.stargaze-apis.com/cosmos/staking/v1beta1/delegations/${testUserOne.address}`,
    )
    const stakedTokens: StakedTokensAPIResponse = await stakedTokensResponse.json()
    const stakedAmount = stakedTokens.delegation_responses[0]?.balance?.amount
    expect(tokenMetadata.extension.staked_amount).toBe(stakedAmount)
  })

  test('Fail to Mint Duplicate Tokens', async () => {
    const mintMsg: MinterExecuteMsg = {
      mint: {},
    }

    await expect(testUserOne.client.execute(
      testUserOne.address,
      minterAddress,
      mintMsg,
      'auto',
      'mint loyalty program token',
    )).rejects.toThrowError(/AlreadyMinted/)
  }
  )

  test('Update tiers', async () => {
    const stakedTokensResponse: Response = await fetch(
      `https://rest.elgafar-1.stargaze-apis.com/cosmos/staking/v1beta1/delegations/${testUserOne.address}`,
    )
    const stakedTokens: StakedTokensAPIResponse = await stakedTokensResponse.json()
    const stakedAmount = stakedTokens.delegation_responses[0]?.balance?.amount

    const scrambledUpdateTiersMsg: MinterExecuteMsg = {
        update_tiers: {
            tiers: [(Number(stakedAmount) - 100).toString(), (Number(stakedAmount) + 100).toString(), (Number(stakedAmount) - 500).toString()],
        },
    }
    
    const updateTiersMsg: MinterExecuteMsg = {
      update_tiers: {
        tiers: [(Number(stakedAmount) - 500).toString(), (Number(stakedAmount) - 100).toString(), (Number(stakedAmount) + 100).toString()],
      },
    }

    await expect(testUserTwo.client.execute(
      testUserTwo.address,
      minterAddress,
      updateTiersMsg,
      'auto',
      'update tiers',
    )).rejects.toThrowError(/Unauthorized/)

    await expect(testUserOne.client.execute(
      testUserOne.address,
      minterAddress,
      scrambledUpdateTiersMsg,
      'auto',
      'update tiers',
    )).rejects.toThrowError(/Tiers must be in ascending order/)

    await testUserOne.client.execute(
      testUserOne.address,
      minterAddress,
      updateTiersMsg,
      'auto',
      'update tiers',
    )

    const tiers = await minterQueryClient.tiers()
    expect(tiers).toEqual([(Number(stakedAmount) - 500).toString(), (Number(stakedAmount) - 100).toString(), (Number(stakedAmount) + 100).toString()])

    const updateMsg: MinterExecuteMsg = {
      update: {
        token_id: 1,
      }
    }
    // Any address can update tokens
    await testUserTwo.client.execute(
      testUserTwo.address,
      minterAddress,
      updateMsg,
      'auto',
      'update token',
    )
    
    let userOneTier = await minterQueryClient.tier({ address: testUserOne.address })
    expect(userOneTier).toBe(2)
  })
})
