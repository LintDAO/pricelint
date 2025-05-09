// 统一用户信息的结构
import type {
  TransactionB,
  TransactionF,
} from ".dfx/ic/canisters/backend/backend.did"

// 注意和types.ts中的ApiUserInfo统一格式
export class UserInfo {
  // id = 0; //用户id
  owner = "" // 用户principal，唯一
  name = "" // 用户自己设置的用户名
  create_at = 0 //注册时间
}

export interface UserInfoElement {
  owner?: string
  name?: string
}

export interface WalletInfo {
  id: bigint
  address: string
  principal_id: string[] //opt
  from: string //'NNS' | 'Plug' | 'Stoic' | 'AstorMe'
  name: string
  now_transactions: number
  transactions: number
  last_transaction_time: number
  last_sync_time: number
}
export interface syncedWalletCache {
  principal_id: string[] //opt
  tokens: [
    {
      symbol: string
    },
  ]
}

export interface WalletHistory {
  price: number
  amount: number
  walletAmount: number
  timestamp: number
  walletValue: number
  t_type: string
}

export interface syncWalletParam {
  walletId: bigint
  history: TransactionF[]
}

export interface WalletTag {
  id: number
  address: string
  principal: string[]
  name: string
  from: string
  last_sync_time?: number
}

export interface NNSNeuron {
  neuronId: bigint
  address: string
  from: string
  maturity: number
  stakedMaturity: number
}

export interface WalletToken {
  symbol: string
  logo: string
  balance: number
  price: number
  value: number
}

export interface Token {
  symbol: string
  name: string
  ledgerCanisterId: string
}

export interface Wallet {
  address: string
  name: string
  tokens: WalletToken[]
}

export interface DailyBalance {
  [date: string]: {
    [symbol: string]: {
      amount: number
      value: number
      ledgerCanisterId: string
    }
  }
}

export interface HistoryQueryParams {
  from_time: number
  to_time: number
  sort_method: string[]
  wids: number[]
}

export type SyncedHistory = TransactionB[]

export interface UserConfig {
  base_currency: string
  time_zone: string
  tax_method: string
}

export interface TaxReportData {
  capital_gain_or_loss: number
  costs_expenses: number
  gifts_dotations_lost_coins: number
  income: number
  other_gain: number
}
