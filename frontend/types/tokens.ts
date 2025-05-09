import type {
  Details,
  TransactionF,
} from ".dfx/ic/canisters/backend/backend.did"

export interface ICRC1Info {
  canisters: {
    //Related canister id
    governance: string
    index: string
    ledger: string
    root: string
    swap: string
  }
  decimals: number
  fee: number
  meta: {
    description: string
    logo: string
    name: string
    url: string
  }
  name: string
  symbol: string
}

export interface Currency {
  decimals: number
  symbol: string
}

export interface IRCR1Price {
  id: number
  low: number //最低价
  high: number
  close: number //收盘价
  open: number
  timestamp: number
}

export interface LedgerICPTransaction {
  allowance: null
  amount: string
  block_hash: string
  block_height: string
  created_at: number
  expected_allowance: null
  expires_at: null
  fee: string
  from_account_identifier: string
  icrc1_memo: string
  memo: string
  parent_hash: string
  spender_account_identifier: null
  to_account_identifier: string
  transaction_hash: string
  transfer_type: string
}

export type InferredTransaction = TransactionF

export type SyncedTransaction = {
  id: bigint
  tag: Array<string>
  hash: string
  memo: string
  t_type: string
  comment: string
  address: string
  timestamp: number
  details: Details
  manual: boolean
}

export type ICRC1BalanceResult = {
  symbol: string
  logo: string
  balance: number
  price: number
  value: number
  error?: Error
}
