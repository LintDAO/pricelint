import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";
import { copyToClipboard } from "quasar";
import { showMessageError, showMessageSuccess } from "./message";

/**
 * 将带精度的值转换为实际代币值。
 * 例如：100000000 e8s (8位精度) 转换为 1 ICP。
 * @param amount 带精度的值（字符串或数字，例如 100000000）
 * @param decimals 精度（小数位数，例如 8）
 * @returns 实际代币值（例如 1）
 */
export function fromTokenAmount(
  amount: string | number,
  decimals: number
): number {
  const parsedAmount = typeof amount === "string" ? parseFloat(amount) : amount;
  return parsedAmount / Math.pow(10, decimals);
}

/**
 * 将实际代币值转换为带精度的值。
 * 例如：1 ICP 转换为 100000000 e8s (8位精度)。
 * @param amount 实际代币值（例如 1）
 * @param decimals 精度（小数位数，例如 8）
 * @returns 带精度的值（例如 100000000）
 */
export function toTokenAmount(
  amount: string | number,
  decimals: number
): bigint {
  const parsedAmount = typeof amount === "string" ? parseFloat(amount) : amount;
  return BigInt(parsedAmount * Math.pow(10, decimals));
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

export function copyText(text: string) {
  copyToClipboard(text)
    .then(() => {
      showMessageSuccess(`copy ${text} success`);
    })
    .catch(() => {
      showMessageError("copy failed");
    });
}
