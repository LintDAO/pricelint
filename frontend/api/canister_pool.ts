import type { Identity } from "@dfinity/agent"
import { Actor, HttpAgent } from "@dfinity/agent"
import {
  backend as anonymousActorBackend,
  idlFactory as idlFactoryBackend,
  canisterId as canisterIdBackend,
} from "canisters/backend"

const createActor = (canisterId: string, idlFactory: any, options: any) => {
  const agent = new HttpAgent({ ...options?.agentOptions })
  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options?.actorOptions,
  })
}

// console.error('init canister pool');

// 当前登录信息
let currentPrincipal = ""

// 缓存的 actor
const ACTOR_CACHE = {}

// 未登录的情况下也要初始化个匿名的
ACTOR_CACHE[""] = {
  backend: anonymousActorBackend,
}

// 4. 暴露设置方法
export function setCurrentIdentity(identity: Identity, principal: string) {
  currentPrincipal = principal
  // console.log('set current principal', principal);

  if (ACTOR_CACHE[currentPrincipal]) return // 已经有了

  // 如果是本地调试，用 https://identity.ic0.app 进行身份认证是无法通过签名的，所以本地调试统一用匿名账户
  if (process.env.network != "ic") {
    console.log("development mode use anonymous actor")
    ACTOR_CACHE[currentPrincipal] = ACTOR_CACHE[""]
    return
  }

  // 把所有用到的 actor 初始化
  ACTOR_CACHE[currentPrincipal] = {
    backend: createActor(canisterIdBackend as string, idlFactoryBackend, {
      agentOptions: { identity },
    }),
  }
}

export function getCurrentPrincipal(): string {
  return currentPrincipal
}

// 提供取消登录方法
export function clearCurrentIdentity() {
  currentPrincipal = ""
  // console.log('set current principal', '');
}

/**
 * A ready-to-use agent for the community canister
 * @type {import("@dfinity/agent").ActorSubclass<import("./../../../services/.dfx/local/canisters/backend/backend.did.js")._SERVICE>}
 */
export const getBackend = (principal?: string) => {
  return ACTOR_CACHE[principal ?? currentPrincipal].backend
}
