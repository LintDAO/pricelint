import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";

//根据精度计算对应的代币数值
export function currencyCalculate(amount: string, decimals: number): number {
  const floatValue = parseInt(amount) / Math.pow(10, decimals);
  return floatValue;
}

//将 principal id 转换为 account id
// * account id无法转换为 principal id
export function p2a(principal: string): string {
  const principalId = Principal.fromText(principal);
  const identity = AccountIdentifier.fromPrincipal({ principal: principalId });
  return identity.toHex();
}

// 识别字符串是否属于principal ID类型
export function isPrincipal(text: string): boolean {
  try {
    //只有是principalID才不会弹出
    Principal.fromText(text);
    return true;
  } catch (error) {
    //如果不是principalID就会报错弹出
    return false;
  }
}

// 检查用户是否使用的老域名访问，如果是老域名，则将其跳转到新域名。
export function checkDomain() {
  const oldDomain = "old";
  const newDomain = "new";

  if (window.location.hostname.includes(oldDomain)) {
    // 构建新的 URL，替换掉旧的域名
    const newUrl = window.location.href.replace(oldDomain, newDomain);
    window.location.replace(newUrl);
  }
}
