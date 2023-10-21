import { instantiate2Address } from '@cosmjs/cosmwasm-stargate/build/instantiate2'
import { fromHex } from '@cosmjs/encoding'
import chainConfig from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP } from '../setup/context'
import { readChecksumFile } from '../utils/file'
import _ from 'lodash'
import path from 'path'

describe('LoyaltyProgramInstantiation', () => {
  let context: Context

  beforeAll(async () => {
    context = new Context()
    await context.initialize(true)
  })

  test('is initialized', async () => {
    expect(context.getContractAddress(CONTRACT_MAP.VIP_MINTER)).toBeTruthy()
    expect(context.getContractAddress(CONTRACT_MAP.VIP_COLLECTION)).toBeTruthy()
  })

  test('loyalty program collection address is correct', async () => {
    const vipMinter = context.getContractAddress(CONTRACT_MAP.VIP_MINTER)
    const vipCollection = context.getContractAddress(CONTRACT_MAP.VIP_COLLECTION)

    const checksumFilePath = path.join(chainConfig.artifacts_path, 'checksums.txt')
    const checksum = await readChecksumFile(checksumFilePath, 'stargaze_vip_collection.wasm')
    const checksumUint8Array = fromHex(checksum)
    const salt = new TextEncoder().encode('vip_collection1')
    const address2 = instantiate2Address(checksumUint8Array, vipMinter, salt, 'stars')

    expect(address2).toBe(vipCollection)
  })
})
