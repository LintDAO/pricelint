// 注意和types.ts中的ApiUserInfo统一格式
export class UserInfo {
  // id = 0; //用户id
  owner = ""; // 用户principal，唯一
  name = ""; // 用户自己设置的用户名
  create_at = 0; //注册时间
}

export interface UserInfoElement {
  owner?: string;
  name?: string;
}
