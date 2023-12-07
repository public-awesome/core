import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import chainConfig from '../../configs/chain_config.json'
import testAccounts from '../../configs/test_accounts.json'
import { getSigningClient } from '../utils/client'
import { InstantiateMsg as RoyaltyRegistryInstantiateMsg } from '@stargazezone/core-types/src/RoyaltyRegistry.types'
import { InstantiateMsg as VendingFactoryInstantiateMsg } from '@stargazezone/launchpad/src/VendingFactory.types'
import fs from 'fs'
import _ from 'lodash'
import path from 'path'

export const CONTRACT_MAP = {
  // core artifacts
  FAIR_BURN: 'stargaze_fair_burn',
  ROYALTY_REGISTRY: 'stargaze_royalty_registry',

  // launchpad artifacts
  VENDING_MINTER: 'vending_minter',
  VENDING_FACTORY: 'vending_factory',
  SG721_BASE: 'sg721_base',
}

export type TestUser = {
  name: string
  address: string
  client: SigningCosmWasmClient
}

export type TestUserMap = { [name: string]: TestUser }

export default class Context {
  codeIds: { [key: string]: number } = {}
  contracts: { [key: string]: string[] } = {}
  testCachePath: string = path.join(__dirname, '../../tmp/test_cache.json')
  testUserMap: TestUserMap = {}

  initializeTestUsers = async () => {
    for (let i = 0; i < testAccounts.length; i++) {
      const mnemonic = testAccounts[i].mnemonic
      const signingClient = await getSigningClient(mnemonic)
      const testAccount = testAccounts[i]
      this.testUserMap[testAccount.name] = {
        name: testAccount.name,
        address: testAccounts[i].address,
        client: signingClient.client,
      }
    }
  }

  writeContext = () => {
    const dir = path.dirname(this.testCachePath)

    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true })
    }

    fs.writeFileSync(
      this.testCachePath,
      JSON.stringify({
        codeIds: this.codeIds,
        contracts: this.contracts,
      }),
    )
  }

  hydrateContext = async () => {
    let testCache = JSON.parse(fs.readFileSync(this.testCachePath, 'utf8'))
    this.codeIds = testCache.codeIds
    this.contracts = testCache.contracts
  }

  getCodeId = (codeIdKey: string) => {
    return this.codeIds[codeIdKey]
  }

  getContractAddress = (contractKey: string, index: number = 0) => {
    try {
      return this.contracts[contractKey][index]
    } catch {
      console.log(`error ${contractKey} ${index} ${JSON.stringify(this.contracts)}}`)
    }
    return this.contracts[contractKey][index]
  }

  pushContractAddress = (contractKey: string, contractAddress: string) => {
    this.contracts[contractKey] = _.extend([], this.contracts[contractKey], [contractAddress])
  }

  uploadContracts = async () => {
    let { client, address: sender } = this.testUserMap['user1']

    let fileNames = fs.readdirSync(chainConfig.artifacts_path)
    let wasmFileNames = _.filter(fileNames, (fileName) => _.endsWith(fileName, '.wasm'))

    for (const idx in wasmFileNames) {
      let wasmFileName = wasmFileNames[idx]
      let wasmFilePath = path.join(chainConfig.artifacts_path, wasmFileName)
      let wasmFile = fs.readFileSync(wasmFilePath, { encoding: null })
      let uploadResult = await client.upload(sender, wasmFile, 'auto')
      let codeIdKey = wasmFileName.replace('-aarch64', '').replace('.wasm', '')
      this.codeIds[codeIdKey] = uploadResult.codeId
      console.log(`Uploaded ${codeIdKey} contract with codeId ${uploadResult.codeId}`)
    }
  }

  instantiateContract = async (client: SigningCosmWasmClient, sender: string, contractKey: string, msg: any) => {
    let instantiateResult = await client.instantiate(sender, this.codeIds[contractKey], msg, contractKey, 'auto')
    this.pushContractAddress(contractKey, instantiateResult.contractAddress)
    console.log(`Instantiated ${contractKey} contract with address ${instantiateResult.contractAddress}`)
    return instantiateResult
  }

  instantiateContracts = async () => {
    let { client, address: sender } = this.testUserMap['user1']
    let { address: feeManager } = this.testUserMap['user5']

    // Instantiate stargaze_fair_burn
    let instantiateFairBurnResult = await this.instantiateContract(client, sender, CONTRACT_MAP.FAIR_BURN, {
      fee_bps: 5000,
      fee_manager: feeManager,
    })

    // Instantiate stargaze_royalty_registry
    let royaltyRegistryInstantiateMsg: RoyaltyRegistryInstantiateMsg = {
      config: {
        max_share_delta: '0.10',
        update_wait_period: 12,
      },
    }
    await this.instantiateContract(client, sender, CONTRACT_MAP.ROYALTY_REGISTRY, royaltyRegistryInstantiateMsg)

    // Instantiate vendinge_factory
    let vendingFactoryInstantiateMsg: VendingFactoryInstantiateMsg = {
      params: {
        allowed_sg721_code_ids: [this.codeIds[CONTRACT_MAP.SG721_BASE]],
        code_id: this.codeIds[CONTRACT_MAP.VENDING_MINTER],
        creation_fee: { amount: '1000000', denom: 'ustars' },
        frozen: false,
        max_trading_offset_secs: 60 * 60,
        min_mint_price: { amount: '1000000', denom: 'ustars' },
        mint_fee_bps: 200,
        extension: {
          airdrop_mint_fee_bps: 200,
          airdrop_mint_price: { amount: '1000000', denom: 'ustars' },
          max_per_address_limit: 10_000,
          max_token_limit: 10_000,
          shuffle_fee: { amount: '1000000', denom: 'ustars' },
        },
      },
    }
    await this.instantiateContract(client, sender, CONTRACT_MAP.VENDING_FACTORY, vendingFactoryInstantiateMsg)
  }
}
