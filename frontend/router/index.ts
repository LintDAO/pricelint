import Home from "@/views/home/Home.vue"
import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router"
import errors from "./modules/errors"

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  // {
  //   path: "/market",
  //   component: MarketLayout, // 只是一个布局容器
  //   children: [
  //     { path: "", name: "Market", component: Market }, // /market
  //     { path: "post", name: "Post", component: PostPredict }, // /market/post
  //   ],
  // }
  ...errors,
  {
    path: "/:catchAll(.*)",
    redirect: "/error/404",
  },
]

const router = createRouter({
  history: createWebHistory("/"),
  routes,
})

export default router
