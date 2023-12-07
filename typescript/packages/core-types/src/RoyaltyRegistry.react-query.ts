/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.30.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery } from "react-query";
import { Decimal, InstantiateMsg, Config, ExecuteMsg, QueryMsg, QueryBoundForString, QueryOptionsForString, NullableRoyaltyDefault, Addr, Timestamp, Uint64, RoyaltyDefault, RoyaltyEntry, NullableRoyaltyProtocol, RoyaltyProtocol, RoyaltyPaymentResponse, ArrayOfRoyaltyProtocol } from "./RoyaltyRegistry.types";
import { RoyaltyRegistryQueryClient } from "./RoyaltyRegistry.client";
export const royaltyRegistryQueryKeys = {
  contract: ([{
    contract: "royaltyRegistry"
  }] as const),
  address: (contractAddress: string | undefined) => ([{ ...royaltyRegistryQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...royaltyRegistryQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const),
  collectionRoyaltyDefault: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...royaltyRegistryQueryKeys.address(contractAddress)[0],
    method: "collection_royalty_default",
    args
  }] as const),
  collectionRoyaltyProtocol: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...royaltyRegistryQueryKeys.address(contractAddress)[0],
    method: "collection_royalty_protocol",
    args
  }] as const),
  royaltyProtocolByCollection: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...royaltyRegistryQueryKeys.address(contractAddress)[0],
    method: "royalty_protocol_by_collection",
    args
  }] as const),
  royaltyPayment: (contractAddress: string | undefined, args?: Record<string, unknown>) => ([{ ...royaltyRegistryQueryKeys.address(contractAddress)[0],
    method: "royalty_payment",
    args
  }] as const)
};
export const royaltyRegistryQueries = {
  config: <TData = Config,>({
    client,
    options
  }: RoyaltyRegistryConfigQuery<TData>): UseQueryOptions<Config, Error, TData> => ({
    queryKey: royaltyRegistryQueryKeys.config(client?.contractAddress),
    queryFn: () => client ? client.config() : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  collectionRoyaltyDefault: <TData = NullableRoyaltyDefault,>({
    client,
    args,
    options
  }: RoyaltyRegistryCollectionRoyaltyDefaultQuery<TData>): UseQueryOptions<NullableRoyaltyDefault, Error, TData> => ({
    queryKey: royaltyRegistryQueryKeys.collectionRoyaltyDefault(client?.contractAddress, args),
    queryFn: () => client ? client.collectionRoyaltyDefault({
      collection: args.collection
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  collectionRoyaltyProtocol: <TData = NullableRoyaltyProtocol,>({
    client,
    args,
    options
  }: RoyaltyRegistryCollectionRoyaltyProtocolQuery<TData>): UseQueryOptions<NullableRoyaltyProtocol, Error, TData> => ({
    queryKey: royaltyRegistryQueryKeys.collectionRoyaltyProtocol(client?.contractAddress, args),
    queryFn: () => client ? client.collectionRoyaltyProtocol({
      collection: args.collection,
      protocol: args.protocol
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  royaltyProtocolByCollection: <TData = ArrayOfRoyaltyProtocol,>({
    client,
    args,
    options
  }: RoyaltyRegistryRoyaltyProtocolByCollectionQuery<TData>): UseQueryOptions<ArrayOfRoyaltyProtocol, Error, TData> => ({
    queryKey: royaltyRegistryQueryKeys.royaltyProtocolByCollection(client?.contractAddress, args),
    queryFn: () => client ? client.royaltyProtocolByCollection({
      collection: args.collection,
      queryOptions: args.queryOptions
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  royaltyPayment: <TData = RoyaltyPaymentResponse,>({
    client,
    args,
    options
  }: RoyaltyRegistryRoyaltyPaymentQuery<TData>): UseQueryOptions<RoyaltyPaymentResponse, Error, TData> => ({
    queryKey: royaltyRegistryQueryKeys.royaltyPayment(client?.contractAddress, args),
    queryFn: () => client ? client.royaltyPayment({
      collection: args.collection,
      protocol: args.protocol
    }) : Promise.reject(new Error("Invalid client")),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  })
};
export interface RoyaltyRegistryReactQuery<TResponse, TData = TResponse> {
  client: RoyaltyRegistryQueryClient | undefined;
  options?: UseQueryOptions<TResponse, Error, TData>;
}
export interface RoyaltyRegistryRoyaltyPaymentQuery<TData> extends RoyaltyRegistryReactQuery<RoyaltyPaymentResponse, TData> {
  args: {
    collection: string;
    protocol?: string;
  };
}
export function useRoyaltyRegistryRoyaltyPaymentQuery<TData = RoyaltyPaymentResponse>({
  client,
  args,
  options
}: RoyaltyRegistryRoyaltyPaymentQuery<TData>) {
  return useQuery<RoyaltyPaymentResponse, Error, TData>(royaltyRegistryQueryKeys.royaltyPayment(client?.contractAddress, args), () => client ? client.royaltyPayment({
    collection: args.collection,
    protocol: args.protocol
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface RoyaltyRegistryRoyaltyProtocolByCollectionQuery<TData> extends RoyaltyRegistryReactQuery<ArrayOfRoyaltyProtocol, TData> {
  args: {
    collection: string;
    queryOptions?: QueryOptionsForString;
  };
}
export function useRoyaltyRegistryRoyaltyProtocolByCollectionQuery<TData = ArrayOfRoyaltyProtocol>({
  client,
  args,
  options
}: RoyaltyRegistryRoyaltyProtocolByCollectionQuery<TData>) {
  return useQuery<ArrayOfRoyaltyProtocol, Error, TData>(royaltyRegistryQueryKeys.royaltyProtocolByCollection(client?.contractAddress, args), () => client ? client.royaltyProtocolByCollection({
    collection: args.collection,
    queryOptions: args.queryOptions
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface RoyaltyRegistryCollectionRoyaltyProtocolQuery<TData> extends RoyaltyRegistryReactQuery<NullableRoyaltyProtocol, TData> {
  args: {
    collection: string;
    protocol: string;
  };
}
export function useRoyaltyRegistryCollectionRoyaltyProtocolQuery<TData = NullableRoyaltyProtocol>({
  client,
  args,
  options
}: RoyaltyRegistryCollectionRoyaltyProtocolQuery<TData>) {
  return useQuery<NullableRoyaltyProtocol, Error, TData>(royaltyRegistryQueryKeys.collectionRoyaltyProtocol(client?.contractAddress, args), () => client ? client.collectionRoyaltyProtocol({
    collection: args.collection,
    protocol: args.protocol
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface RoyaltyRegistryCollectionRoyaltyDefaultQuery<TData> extends RoyaltyRegistryReactQuery<NullableRoyaltyDefault, TData> {
  args: {
    collection: string;
  };
}
export function useRoyaltyRegistryCollectionRoyaltyDefaultQuery<TData = NullableRoyaltyDefault>({
  client,
  args,
  options
}: RoyaltyRegistryCollectionRoyaltyDefaultQuery<TData>) {
  return useQuery<NullableRoyaltyDefault, Error, TData>(royaltyRegistryQueryKeys.collectionRoyaltyDefault(client?.contractAddress, args), () => client ? client.collectionRoyaltyDefault({
    collection: args.collection
  }) : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}
export interface RoyaltyRegistryConfigQuery<TData> extends RoyaltyRegistryReactQuery<Config, TData> {}
export function useRoyaltyRegistryConfigQuery<TData = Config>({
  client,
  options
}: RoyaltyRegistryConfigQuery<TData>) {
  return useQuery<Config, Error, TData>(royaltyRegistryQueryKeys.config(client?.contractAddress), () => client ? client.config() : Promise.reject(new Error("Invalid client")), { ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  });
}