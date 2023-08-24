import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import Context, { CONTRACT_MAP } from '../setup/context'
import { Sg2ExecuteMsgForVendingMinterInitMsgExtension } from '../types/vendingMinter'
import { getClient, getSigningClient } from './client'
import { getExpirationString } from './datetime'
import assert from 'assert'
import _ from 'lodash'

export const createMinter = async (context: Context) => {
  const queryClient = await getClient()

  let vendingFactoryAddress = context.getContractAddress(CONTRACT_MAP.VENDING_FACTORY)
  let { params: factoryParams } = await queryClient.queryContractSmart(vendingFactoryAddress, {
    params: {},
  })

  const { client: signingClient, address: sender } = context.testUserMap['user1']
  let msg: Sg2ExecuteMsgForVendingMinterInitMsgExtension = {
    create_minter: {
      init_msg: {
        base_token_uri: 'ipfs://bafybeiek33kk3js27dhodwadtmrf3p6b64netr6t3xzi3sbfyxovxe36qe',
        payment_address: sender,
        start_time: getExpirationString(2),
        num_tokens: 10_000,
        mint_price: { amount: '1000000', denom: 'ustars' },
        per_address_limit: 100,
        whitelist: null,
      },
      collection_params: {
        code_id: context.getCodeId(CONTRACT_MAP.SG721_BASE),
        name: 'Test Collection',
        symbol: 'TC',
        info: {
          creator: sender,
          description: 'This is the collection description',
          image: 'ipfs://bafybeiek33kk3js27dhodwadtmrf3p6b64netr6t3xzi3sbfyxovxe36qe/1.png',
          start_trading_time: getExpirationString(0),
          royalty_info: {
            payment_address: sender,
            share: '0.05',
          },
        },
      },
    },
  }
  let executeResult = await signingClient.execute(
    sender,
    vendingFactoryAddress,
    msg,
    'auto',
    'instantiate-vending-minter',
    [factoryParams.creation_fee],
  )

  let instantiateEvents = _.filter(executeResult.events, (event) => {
    return event.type === 'instantiate'
  })

  let minterAddress = instantiateEvents[0].attributes[0].value
  let collectionAddress = instantiateEvents[1].attributes[0].value

  context.pushContractAddress(CONTRACT_MAP.VENDING_MINTER, minterAddress)
  context.pushContractAddress(CONTRACT_MAP.SG721_BASE, collectionAddress)

  return [minterAddress, collectionAddress]
}

export const mintNft = async (
  context: Context,
  signingClient: SigningCosmWasmClient,
  sender: string,
  recipientAddress: string,
): Promise<[string, string]> => {
  const queryClient = await getClient()

  let vendingFactoryAddress = context.getContractAddress(CONTRACT_MAP.VENDING_FACTORY)
  let { params: factoryParams } = await queryClient.queryContractSmart(vendingFactoryAddress, {
    params: {},
  })

  let vendingMinterAddress = context.getContractAddress(CONTRACT_MAP.VENDING_MINTER)
  let minterConfig = await queryClient.queryContractSmart(vendingMinterAddress, {
    config: {},
  })
  let collectionAddress = minterConfig.collection_address

  let mintMsg = { mint: { token_uri: 'ipfs://bafybeiek33kk3js27dhodwadtmrf3p6b64netr6t3xzi3sbfyxovxe36qe/1.png' } }

  let mintPrice = (factoryParams.mint_fee_bps * factoryParams.min_mint_price.amount) / 10000
  let mintExecuteResult = await signingClient.execute(sender, vendingMinterAddress, mintMsg, 'auto', 'mint-nft', [
    { amount: mintPrice.toString(), denom: factoryParams.min_mint_price.denom },
  ])

  let tokenId: string = ''
  for (const idx in mintExecuteResult.events) {
    {
      const event = mintExecuteResult.events[idx]
      let tokenIdAttribute = _.find(event.attributes, (attribute) => attribute.key === 'token_id')
      if (tokenIdAttribute) {
        tokenId = tokenIdAttribute.value
        break
      }
    }
  }
  assert(tokenId, 'token_id not found in wasm event attributes')

  if (sender !== recipientAddress) {
    // Transfer NFT to recipient
    let transferMsg = { transfer_nft: { recipient: recipientAddress, token_id: tokenId } }
    let transferExecuteResult = await signingClient.execute(
      sender,
      collectionAddress,
      transferMsg,
      'auto',
      'transfer-nft',
    )
  }

  return [collectionAddress, tokenId]
}

export const approveNft = async (
  signingClient: SigningCosmWasmClient,
  sender: string,
  collectionAddress: string,
  tokenId: string,
  approveAddress: string,
) => {
  let msg = { approve: { spender: approveAddress, token_id: tokenId } }
  let executeResult = await signingClient.execute(sender, collectionAddress, msg, 'auto', 'approve-nft')
  return executeResult
}
