/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Uint128, InstantiateMsg, ExecuteMsg, QueryMsg, String, Boolean, Uint64, ArrayOfUint128 } from "./minter.types";
export interface MinterReadOnlyInterface {
  contractAddress: string;
  collection: () => Promise<String>;
  isPaused: () => Promise<Boolean>;
  tokenUpdateHeight: ({
    tokenId
  }: {
    tokenId: number;
  }) => Promise<Uint64>;
  tier: ({
    address
  }: {
    address: string;
  }) => Promise<Uint64>;
  tiers: () => Promise<ArrayOfUint128>;
}
export class MinterQueryClient implements MinterReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.collection = this.collection.bind(this);
    this.isPaused = this.isPaused.bind(this);
    this.tokenUpdateHeight = this.tokenUpdateHeight.bind(this);
    this.tier = this.tier.bind(this);
    this.tiers = this.tiers.bind(this);
  }

  collection = async (): Promise<String> => {
    return this.client.queryContractSmart(this.contractAddress, {
      collection: {}
    });
  };
  isPaused = async (): Promise<Boolean> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_paused: {}
    });
  };
  tokenUpdateHeight = async ({
    tokenId
  }: {
    tokenId: number;
  }): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      token_update_height: {
        token_id: tokenId
      }
    });
  };
  tier = async ({
    address
  }: {
    address: string;
  }): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tier: {
        address
      }
    });
  };
  tiers = async (): Promise<ArrayOfUint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tiers: {}
    });
  };
}
export interface MinterInterface extends MinterReadOnlyInterface {
  contractAddress: string;
  sender: string;
  mint: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  update: ({
    tokenId
  }: {
    tokenId: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  pause: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  resume: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateTiers: ({
    tiers
  }: {
    tiers: Uint128[];
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateBaseURI: ({
    baseUri
  }: {
    baseUri: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class MinterClient extends MinterQueryClient implements MinterInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.mint = this.mint.bind(this);
    this.update = this.update.bind(this);
    this.pause = this.pause.bind(this);
    this.resume = this.resume.bind(this);
    this.updateTiers = this.updateTiers.bind(this);
    this.updateBaseURI = this.updateBaseURI.bind(this);
  }

  mint = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      mint: {}
    }, fee, memo, _funds);
  };
  update = async ({
    tokenId
  }: {
    tokenId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update: {
        token_id: tokenId
      }
    }, fee, memo, _funds);
  };
  pause = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      pause: {}
    }, fee, memo, _funds);
  };
  resume = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      resume: {}
    }, fee, memo, _funds);
  };
  updateTiers = async ({
    tiers
  }: {
    tiers: Uint128[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_tiers: {
        tiers
      }
    }, fee, memo, _funds);
  };
  updateBaseURI = async ({
    baseUri
  }: {
    baseUri: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_base_u_r_i: {
        base_uri: baseUri
      }
    }, fee, memo, _funds);
  };
}