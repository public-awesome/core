import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { toUtf8 } from '@cosmjs/encoding'
import { denom } from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP, TestUser } from '../setup/context'
import { getQueryClient } from '../utils/client'
import { ArrayOfUint128, ExecuteMsg as MinterExecuteMsg } from '../types/minter.types'
import { ExecuteMsg as CollectionExecuteMsg } from '../types/collection.types'
import { MinterQueryClient } from '../types/minter.client'
import { CollectionQueryClient } from '../types/collection.client'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import _ from 'lodash'
import { coin } from '@cosmjs/proto-signing'

describe('Mint Loyalty Program Tokens', () => {
  const userOne = 'user1'
  const userTwo = 'user2'

  let context: Context
  let queryClient: CosmWasmClient
  let minterQueryClient: MinterQueryClient
  let minterAddress: string
  let tiers: ArrayOfUint128
  let testUserOne: TestUser

  beforeAll(async () => {
    context = new Context()
    await context.initialize(true)
    minterAddress = context.getContractAddress(CONTRACT_MAP.VIP_MINTER)

    queryClient = await getQueryClient()

    minterQueryClient = new MinterQueryClient(queryClient, minterAddress)
    tiers = await minterQueryClient.tiers()
    testUserOne = context.getTestUser(userOne)
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
})
