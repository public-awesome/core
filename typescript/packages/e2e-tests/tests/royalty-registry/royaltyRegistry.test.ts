import { denom } from '../../configs/chain_config.json'
import Context, { CONTRACT_MAP } from '../setup/context'
import { getClient } from '../utils/client'
import { approveNft, createMinter, mintNft } from '../utils/nft'
import { contracts } from '@stargazezone/core-types'
import { RoyaltyDefault } from '@stargazezone/core-types/lib/RoyaltyRegistry.types'
import { RoyaltyEntry } from '@stargazezone/core-types/lib/RoyaltyRegistry.types'
import _ from 'lodash'

const { RoyaltyRegistryClient, RoyaltyRegistryQueryClient } = contracts.RoyaltyRegistry

describe('RoyaltyRegistry', () => {
  const creatorName = 'user1'
  const nonCreatorName = 'user2'
  const recipientName = 'user3'

  let context: Context
  let royaltyRegistryAddress: string
  let minterAddress: string
  let collectionAddress: string

  beforeAll(async () => {
    context = new Context()
    await context.initializeTestUsers()
    await context.hydrateContext()
    ;[minterAddress, collectionAddress] = await createMinter(context)
    royaltyRegistryAddress = context.getContractAddress(CONTRACT_MAP.ROYALTY_REGISTRY)
  })

  test('initialize collection royalty', async () => {
    const nonCreator = context.testUserMap[nonCreatorName]

    let queryClient = await getClient()
    let contractResponse = await queryClient.getContract(collectionAddress)

    const royaltyRegistryQueryClient = new RoyaltyRegistryQueryClient(nonCreator.client, royaltyRegistryAddress)
    const royaltyRegistryClient = new RoyaltyRegistryClient(
      nonCreator.client,
      nonCreator.address,
      royaltyRegistryAddress,
    )

    let royaltyDefault = await royaltyRegistryQueryClient.collectionRoyaltyDefault({ collection: collectionAddress })

    if (royaltyDefault) {
      // Already initialized in prior test run
    } else {
      let response = await royaltyRegistryClient.initializeCollectionRoyalty(
        { collection: collectionAddress },
        'auto',
        'initialize-collection-royalty',
      )
      expect(response).toBeTruthy()

      royaltyDefault = await royaltyRegistryQueryClient.collectionRoyaltyDefault({ collection: collectionAddress })
      expect(royaltyDefault).toBeTruthy()
      expect(royaltyDefault?.collection).toEqual(collectionAddress)
      expect(royaltyDefault?.royalty_entry.recipient).toBeTruthy()
      expect(royaltyDefault?.royalty_entry.share).toBeTruthy()
      expect(royaltyDefault?.royalty_entry.updated).toBeNull()
    }
  })

  test('update collection royalty default', async () => {
    const creator = context.testUserMap[creatorName]
    const recipient = context.testUserMap[recipientName]

    const royaltyRegistryQueryClient = new RoyaltyRegistryQueryClient(creator.client, royaltyRegistryAddress)
    const royaltyRegistryClient = new RoyaltyRegistryClient(creator.client, creator.address, royaltyRegistryAddress)

    let royaltyDefaultBefore = (await royaltyRegistryQueryClient.collectionRoyaltyDefault({
      collection: collectionAddress,
    })) as RoyaltyDefault
    expect(royaltyDefaultBefore).toBeTruthy()
    expect(royaltyDefaultBefore.collection).toEqual(collectionAddress)

    let shareDelta = '0.005'
    let response = await royaltyRegistryClient.updateCollectionRoyaltyDefault(
      { collection: collectionAddress, decrement: false, recipient: recipient.address, shareDelta },
      'auto',
      'update-collection-royalty-default',
    )
    expect(response).toBeTruthy()

    let royaltyDefaultAfter = (await royaltyRegistryQueryClient.collectionRoyaltyDefault({
      collection: collectionAddress,
    })) as RoyaltyDefault
    expect(royaltyDefaultAfter).toBeTruthy()
    expect(royaltyDefaultAfter.collection).toEqual(collectionAddress)
    expect(royaltyDefaultAfter.royalty_entry.recipient).toEqual(recipient.address)
    expect(parseFloat(royaltyDefaultBefore.royalty_entry.share) + parseFloat(shareDelta)).toEqual(
      parseFloat(royaltyDefaultAfter.royalty_entry.share),
    )
    expect(royaltyDefaultAfter.royalty_entry.updated).toBeTruthy()
  })

  test('set collection royalty protocol', async () => {
    const creator = context.testUserMap[creatorName]
    const recipient = context.testUserMap[recipientName]

    // Dummy protocol address just for testing purposes
    const protocolAddress = 'stars1tu06v6gzcc0gugfrmxrfrj20f9y499ccrylkgx'

    const royaltyRegistryQueryClient = new RoyaltyRegistryQueryClient(creator.client, royaltyRegistryAddress)
    const royaltyRegistryClient = new RoyaltyRegistryClient(creator.client, creator.address, royaltyRegistryAddress)

    let royaltyDefaultBefore = (await royaltyRegistryQueryClient.collectionRoyaltyDefault({
      collection: collectionAddress,
    })) as RoyaltyDefault
    expect(royaltyDefaultBefore).toBeTruthy()
    expect(royaltyDefaultBefore.collection).toEqual(collectionAddress)

    let share = '0.07'
    let response = await royaltyRegistryClient.setCollectionRoyaltyProtocol(
      { collection: collectionAddress, protocol: protocolAddress, recipient: recipient.address, share },
      'auto',
      'set-collection-royalty-protocol',
    )
    expect(response).toBeTruthy()

    let royaltyProtocol = await royaltyRegistryQueryClient.collectionRoyaltyProtocol({
      collection: collectionAddress,
      protocol: protocolAddress,
    })
    expect(royaltyProtocol).toBeTruthy()
    expect(royaltyProtocol?.royalty_entry.recipient).toEqual(recipient.address)
    expect(royaltyProtocol?.royalty_entry.share).toEqual(share)
    expect(royaltyProtocol?.royalty_entry.updated).toBeTruthy()
  })
})
