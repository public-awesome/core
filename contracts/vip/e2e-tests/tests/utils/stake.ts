
export interface APIPaginationData {
    readonly next_key: string | null;
    readonly total: string;
  }

export interface StakedTokensAPIResponse {
readonly delegation_responses: {
    readonly delegation: {
    readonly delegator_address: `stars${string}`;
    readonly validator_address: `starsvaloper${string}`;
    readonly shares: string;
    };
    readonly balance: {
    readonly denom: 'ustars';
    readonly amount: string;
    };
}[];
readonly pagination: APIPaginationData;
}
