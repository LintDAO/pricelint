import type { Principal } from "@dfinity/principal/lib/cjs";

// 后端的错误
export type ApiError = {

};

// 后端的返回结果
export type ApiResult<T> = {
    Ok?: T;
    Err?: ApiError;
};
// 后端的返回结果 分页
export type ApiResultByPage<T> = {
    Ok?: {
        data: T[];
        page_num: bigint; // 当前页码
        page_size: bigint; // 页面大小
        total_count: bigint; // 总数
    };
    Err?: ApiError;
};

export type ApiUserInfo = {
    // id: bigint; //id
    owner: Principal | string; // 用户principal，唯一。从后端接受时是principal格式，再通过toString显示成字符串格式。
    name: string;
    created_at: bigint;
};
