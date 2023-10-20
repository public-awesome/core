import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import chainConfig from '../../configs/chain_config.json'
import testAccounts from '../../configs/test_accounts.json'
import { getSigningClient } from '../utils/client'
import assert from 'assert'
import fs from 'fs'
import _ from 'lodash'
import path from 'path'

export const CONTRACT_MAP = {
  // loyalty program artifacts
  VIP_MINTER: 'stargaze_vip_minter',
  VIP_COLLECTION: 'stargaze_vip_collection',
}

export type TestUser = {
  name: string
  address: string
  client: SigningCosmWasmClient
}

export type TestUserMap = { [name: string]: TestUser }

export default class Context {
  private codeIds: { [key: string]: number } = {}
  private contracts: { [key: string]: string[] } = {}
  private testCachePath: string = path.join(__dirname, '../../tmp/test_cache.json')
  private testUserMap: TestUserMap = {}

  private initializeTestUsers = async () => {
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

  private hydrateContext = async () => {
    let testCache = JSON.parse(fs.readFileSync(this.testCachePath, 'utf8'))
    this.codeIds = testCache.codeIds
    this.contracts = testCache.contracts
  }

  private uploadContracts = async () => {
    let { client, address: sender } = this.getTestUser('user1')

    let fileNames = fs.readdirSync(chainConfig.artifacts_path)
    let wasmFileNames = _.filter(fileNames, (fileName) => _.endsWith(fileName, '.wasm'))

    for (const idx in wasmFileNames) {
      let wasmFileName = wasmFileNames[idx]
      if (!_.includes(_.values(CONTRACT_MAP), wasmFileName.replace('.wasm', ''))) {
        continue
      }
      let wasmFilePath = path.join(chainConfig.artifacts_path, wasmFileName)
      let wasmFile = fs.readFileSync(wasmFilePath, { encoding: null })
      let uploadResult = await client.upload(sender, wasmFile, 'auto')
      let codeIdKey = wasmFileName.replace('-aarch64', '').replace('.wasm', '')
      this.codeIds[codeIdKey] = uploadResult.codeId
      console.log(`Uploaded ${codeIdKey} contract with codeId ${uploadResult.codeId}`)
    }
  }

  private instantiateLoyaltyProgramContracts = async () => {
    let { client, address: sender } = this.getTestUser('user1')

    // Instantiate Loyalty Program Minter
    let vipMinterInstantiateMessage = {
      collection_code_id: this.codeIds[CONTRACT_MAP.VIP_COLLECTION],
      tiers: ["10000000000","2000000000","3000000000"],
      base_uri: "ipfs://test",
    }

    let instantiateResult = await client.instantiate(sender, this.codeIds[CONTRACT_MAP.VIP_MINTER], vipMinterInstantiateMessage, CONTRACT_MAP.VIP_MINTER, 'auto')
    
    _.forEach(instantiateResult.events, (event) => {
          if (event.type === 'instantiate') {
            let codeId = parseInt(event.attributes[1].value, 10)
            let contractKey = this.getContractKeyByCodeId(codeId)
            assert(contractKey, 'contract address not found in wasm event attributes')
            console.log(`Instantiated ${contractKey} contract with address ${event.attributes[0].value}`)
            this.addContractAddress(contractKey, event.attributes[0].value)
          }
        })
  }

  private writeContext = () => {
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

  initialize = async (hydrate: boolean) => {
    await this.initializeTestUsers()

    if (hydrate) {
      await this.hydrateContext()
    } else {
      await this.uploadContracts()
      await this.instantiateLoyaltyProgramContracts()
      this.writeContext()
    }
  }

  getTestUser = (userName: string) => {
    return this.testUserMap[userName]
  }

  getCodeId = (codeIdKey: string) => {
    return this.codeIds[codeIdKey]
  }

  getContractKeyByCodeId = (codeId: number) => {
    return _.findKey(this.codeIds, (value, key) => value === codeId)
  }

  getContractAddress = (contractKey: string, index: number = 0) => {
    try {
      return this.contracts[contractKey][index]
    } catch {
      console.log(`error ${contractKey} ${index} ${JSON.stringify(this.contracts)}}`)
    }
    return this.contracts[contractKey][index]
  }

  addContractAddress = (contractKey: string, contractAddress: string) => {
    this.contracts[contractKey] = _.extend([], this.contracts[contractKey], [contractAddress])
  }
}