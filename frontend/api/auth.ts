import { deleteUserInfoStorage } from "@/utils/storage"
import type { Identity } from "@dfinity/agent"
import { AuthClient } from "@dfinity/auth-client"

let client: AuthClient | null = null

export class AuthInfo {
  client: AuthClient
  info?: IdentityInfo

  constructor(client: AuthClient, info?: IdentityInfo) {
    this.client = client
    this.info = info
  }
}

export class IdentityInfo {
  identity: Identity
  principal: string

  constructor(identity: Identity, principal: string) {
    this.identity = identity
    this.principal = principal
  }
}

// 初始化环境
// 提供后续链接的 client 对象 得到客户端对象表明已经准备好链接了
// 通过客户端对象判断是否已经登录，如果登录记录登录信息
export async function initAuth(): Promise<AuthInfo> {
  if (null == client) {
    client = await AuthClient.create({
      idleOptions: {
        // idleTimeout:1000 * 20, // 设置闲置超时时间
        disableIdle: true, // 设置为true禁用检测闲置行为
        disableDefaultIdleCallback: true, // 禁用默认空闲行为 - 防止调用注销和重新加载窗口
        // onIdle() {
        //     // 检测到闲置时的回调，默认为登出并且刷新页面，假如有此方法则会替代原来的方法
        // },
      },
    }) // 创建链接对象;
  }
  // 链接对象已经准备好
  // 取得当前登录信息
  const isAuthenticated = await client.isAuthenticated()
  // console.log("isAuthenticated", isAuthenticated)
  if (isAuthenticated) {
    // 如果已经登录，取得信息
    const identity = client.getIdentity()
    const principal = identity.getPrincipal().toString()

    // console.log('got identity by init auth', identity);
    // console.log('got principal by init auth', principal);

    return new AuthInfo(client, {
      identity: identity,
      principal: principal,
    })
  }

  return new AuthInfo(client)
}

// 登录动作
export async function signIn(client: AuthClient): Promise<IdentityInfo> {
  const days = BigInt(1)
  const hours = BigInt(24)
  const nanoseconds = BigInt(3600000000000)
  const result: IdentityInfo = await new Promise((resolve, reject) => {
    // 进行登录
    client.login({
      derivationOrigin:
        process.env.mode === "production"
          ? `https://${process.env.ASSETS_CANISTER_ID}.icp0.io`
          : undefined,
      identityProvider: "https://identity.ic0.app", // 用线上的 II 认证，本地没法搭建II认证
      onSuccess: () => {
        // 登录成功后取出用户信息
        const identity = client.getIdentity()
        const principal = identity.getPrincipal().toString()
        resolve(new IdentityInfo(identity, principal))
      },
      onError: (err) => {
        console.error("auth II have a error ", err)
        reject(err)
      },
      // Maximum authorization expiration is 8 days
      maxTimeToLive: days * hours * nanoseconds,
    })
  })
  // 持续打印II身份有效时间。
  // const authClient = await AuthClient.create()
  // if (await authClient.isAuthenticated()) {
  //     let identity = authClient.getIdentity();
  //     const nextExpiration = identity.getDelegation().delegations
  //         .map(d => d.delegation.expiration)
  //         .reduce((current, next) => next < current ? next : current);
  //     setInterval(function () {
  //         const expirationDuration = nextExpiration - BigInt(Date.now()) * BigInt(1000_000);
  //         console.log("ii time", expirationDuration)
  //     }, 1000);
  // }

  // console.log('got identity by sign in', result.identity);
  console.log("got principal by sign in", result.principal)
  return result
}

// 登出动作
export async function signOut(client: AuthClient): Promise<void> {
  if (client) {
    const isAuthenticated = await client.isAuthenticated()
    if (isAuthenticated) {
      // 如果已经登录，则同时移除登录缓存
      const principal = client.getIdentity().getPrincipal().toString()
      deleteUserInfoStorage(principal)
      //TODO  登出时刷新页面应该就足够了，消除所有存储感觉不太合理
      // localStorage.clear()
      // sessionStorage.clear()
    }
    await client.logout()
  }
}
