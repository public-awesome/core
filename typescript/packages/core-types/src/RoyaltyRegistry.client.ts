/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.30.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Decimal, InstantiateMsg, Config, ExecuteMsg, QueryMsg, QueryBoundForString, QueryOptionsForString, NullableRoyaltyDefault, Addr, Timestamp, Uint64, RoyaltyDefault, RoyaltyEntry, NullableRoyaltyProtocol, RoyaltyProtocol, RoyaltyPaymentResponse, ArrayOfRoyaltyProtocol } from "./RoyaltyRegistry.types";
export interface RoyaltyRegistryReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  collectionRoyaltyDefault: ({
    collection
  }: {
    collection: string;
  }) => Promise<NullableRoyaltyDefault>;
  collectionRoyaltyProtocol: ({
    collection,
    protocol
  }: {
    collection: string;
    protocol: string;
  }) => Promise<NullableRoyaltyProtocol>;
  royaltyProtocolByCollection: ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions?: QueryOptionsForString;
  }) => Promise<ArrayOfRoyaltyProtocol>;
  royaltyPayment: ({
    collection,
    protocol
  }: {
    collection: string;
    protocol?: string;
  }) => Promise<RoyaltyPaymentResponse>;
}
export class RoyaltyRegistryQueryClient implements RoyaltyRegistryReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.collectionRoyaltyDefault = this.collectionRoyaltyDefault.bind(this);
    this.collectionRoyaltyProtocol = this.collectionRoyaltyProtocol.bind(this);
    this.royaltyProtocolByCollection = this.royaltyProtocolByCollection.bind(this);
    this.royaltyPayment = this.royaltyPayment.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  collectionRoyaltyDefault = async ({
    collection
  }: {
    collection: string;
  }): Promise<NullableRoyaltyDefault> => {
    return this.client.queryContractSmart(this.contractAddress, {
      collection_royalty_default: {
        collection
      }
    });
  };
  collectionRoyaltyProtocol = async ({
    collection,
    protocol
  }: {
    collection: string;
    protocol: string;
  }): Promise<NullableRoyaltyProtocol> => {
    return this.client.queryContractSmart(this.contractAddress, {
      collection_royalty_protocol: {
        collection,
        protocol
      }
    });
  };
  royaltyProtocolByCollection = async ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions?: QueryOptionsForString;
  }): Promise<ArrayOfRoyaltyProtocol> => {
    return this.client.queryContractSmart(this.contractAddress, {
      royalty_protocol_by_collection: {
        collection,
        query_options: queryOptions
      }
    });
  };
  royaltyPayment = async ({
    collection,
    protocol
  }: {
    collection: string;
    protocol?: string;
  }): Promise<RoyaltyPaymentResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      royalty_payment: {
        collection,
        protocol
      }
    });
  };
}
export interface RoyaltyRegistryInterface extends RoyaltyRegistryReadOnlyInterface {
  contractAddress: string;
  sender: string;
  initializeCollectionRoyalty: ({
    collection
  }: {
    collection: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setCollectionRoyaltyDefault: ({
    collection,
    recipient,
    share
  }: {
    collection: string;
    recipient: string;
    share: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateCollectionRoyaltyDefault: ({
    collection,
    decrement,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    recipient?: string;
    shareDelta?: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  setCollectionRoyaltyProtocol: ({
    collection,
    protocol,
    recipient,
    share
  }: {
    collection: string;
    protocol: string;
    recipient: string;
    share: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateCollectionRoyaltyProtocol: ({
    collection,
    decrement,
    protocol,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    protocol: string;
    recipient?: string;
    shareDelta?: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class RoyaltyRegistryClient extends RoyaltyRegistryQueryClient implements RoyaltyRegistryInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.initializeCollectionRoyalty = this.initializeCollectionRoyalty.bind(this);
    this.setCollectionRoyaltyDefault = this.setCollectionRoyaltyDefault.bind(this);
    this.updateCollectionRoyaltyDefault = this.updateCollectionRoyaltyDefault.bind(this);
    this.setCollectionRoyaltyProtocol = this.setCollectionRoyaltyProtocol.bind(this);
    this.updateCollectionRoyaltyProtocol = this.updateCollectionRoyaltyProtocol.bind(this);
  }

  initializeCollectionRoyalty = async ({
    collection
  }: {
    collection: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      initialize_collection_royalty: {
        collection
      }
    }, fee, memo, _funds);
  };
  setCollectionRoyaltyDefault = async ({
    collection,
    recipient,
    share
  }: {
    collection: string;
    recipient: string;
    share: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_collection_royalty_default: {
        collection,
        recipient,
        share
      }
    }, fee, memo, _funds);
  };
  updateCollectionRoyaltyDefault = async ({
    collection,
    decrement,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    recipient?: string;
    shareDelta?: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_collection_royalty_default: {
        collection,
        decrement,
        recipient,
        share_delta: shareDelta
      }
    }, fee, memo, _funds);
  };
  setCollectionRoyaltyProtocol = async ({
    collection,
    protocol,
    recipient,
    share
  }: {
    collection: string;
    protocol: string;
    recipient: string;
    share: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_collection_royalty_protocol: {
        collection,
        protocol,
        recipient,
        share
      }
    }, fee, memo, _funds);
  };
  updateCollectionRoyaltyProtocol = async ({
    collection,
    decrement,
    protocol,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    protocol: string;
    recipient?: string;
    shareDelta?: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_collection_royalty_protocol: {
        collection,
        decrement,
        protocol,
        recipient,
        share_delta: shareDelta
      }
    }, fee, memo, _funds);
  };
}