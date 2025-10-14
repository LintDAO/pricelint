import { initAuth } from "@/api/auth";
import { setCurrentIdentity } from "@/api/canister_pool";
import App from "@/views/app/AppHome.vue";
import CanisterDetail from "@/views/app/modules/canister/CanisterDetail.vue";
import CanisterEdit from "@/views/app/modules/canister/CanisterEdit.vue";
import Canisters from "@/views/app/modules/canister/Canisters.vue";
import CanisterInsights from "@/views/app/modules/canister/insights/CanisterInsights.vue";
import CanisterInsightsTable from "@/views/app/modules/canister/insights/Table.vue";
import DashBoard from "@/views/app/modules/Dashboard.vue";
import Home from "@/views/home/Home.vue";
import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import errors from "./modules/errors";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/app",
    // name: 'App',
    component: App,
    beforeEnter: async (to, from, next) => {
      //校验权限，提前准备好canister的连接，以免出现调用canister时没有认证用户的情况
      try {
        const ai = await initAuth();
        if (ai.info) {
          setCurrentIdentity(ai.info.identity, ai.info.principal);
        }
        next();
      } catch (error) {
        // 处理错误情况
        console.error("beforeEnter Err:", error);
        // 没有登录的情况下，重定向到登录页面
        next("/");
      }
    },
    children: [
      { name: "App", path: "", component: DashBoard },
      { name: "Canisters", path: "canisters", component: Canisters },
      {
        name: "CanisterDetail",
        path: "canisters/:canisterId/overview",
        component: CanisterDetail,
        meta: { label: "Overview" },
      },
      {
        name: "CanisterInsights",
        path: "canisters/:canisterId/insights",
        component: CanisterInsights,
        redirect: { name: "CanisterInsightsTable" },
        children: [
          {
            path: "table", // 子路径，匹配 canisters/:canisterId/insights/table
            component: CanisterInsightsTable, // 渲染 CanisterInsights
            name: "CanisterInsightsTable",
          },
        ],
        meta: {
          label: "Insights",
          sidebar: [{ label: "Table", to: "table", icon: "table_chart" }],
        },
      },
      {
        name: "CanisterEdit",
        path: "canisters/:canisterId/edit",
        component: CanisterEdit,
        meta: {
          label: "Settings",
          sidebar: [{ label: "General", to: "general", icon: "home" }],
        },
      },
    ],
  },
  ...errors,
  {
    path: "/:catchAll(.*)",
    redirect: "/error/404",
  },
];

const router = createRouter({
  history: createWebHistory("/"),
  routes,
});

export default router;
