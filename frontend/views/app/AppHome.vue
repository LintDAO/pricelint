<template>
  <q-layout view="lHh lpR lFr">
    <!-- Mobile Navbar (visible on smaller screens) -->
    <q-header reveal class="bg-white header" v-if="$q.screen.lt.md">
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleDrawer"
          class="q-mr-sm text-grey-9"
        />
        <q-space />
        <!-- User Avatar in Header (Mobile) -->
        <div
          class="connection-panel q-mr-md"
          style="color: black; width: 150px"
        >
          <div class="flex items-center justify-between full-width q-pa-sm">
            <!-- 左侧：Connected + 呼吸灯 -->
            <div class="flex items-center q-gutter-xs">
              <div class="breathing-dot"></div>
              <span class="text-caption">On</span>
            </div>

            <!-- 中间分隔线 -->
            <q-separator vertical style="height: 20px" />

            <!-- 右侧：Principal ID -->
            <div class="flex items-center q-gutter-xs">
              <span class="text-caption text-grey-7">{{ showUser }}</span>
              <q-icon
                name="content_copy"
                class="cursor-pointer"
                @click.stop="copyPid()"
              />
            </div>
          </div>
        </div>
        <q-icon
          name="keyboard_arrow_up"
          class="rotate-icon"
          style="color: black"
          :class="{ 'rotate-active': userMenu }"
        >
          <q-menu v-model="userMenu" anchor="bottom start" self="top start">
            <q-list>
              <template v-for="(item, index) in userMenuItems" :key="index">
                <q-item
                  clickable
                  v-close-popup
                  @click="item.click ? item.click() : null"
                >
                  <q-item-section avatar>
                    <q-icon :name="item.icon" />
                  </q-item-section>
                  <q-item-section>{{ item.label }}</q-item-section>
                </q-item>
                <!-- <q-separator v-if="item.separator" /> -->
              </template>
            </q-list>
          </q-menu>
        </q-icon>
      </q-toolbar>
    </q-header>

    <!-- Sidebar (Drawer) -->
    <q-drawer
      v-model="drawerOpen"
      show-if-above
      :width="256"
      class="bg-grey-11 drawer-container"
      :breakpoint="769"
    >
      <!-- Sidebar Body: Navigation Sections -->
      <q-list padding class="q-px-sm">
        <q-item
          class="q-my-xs q-px-md rounded-borders"
          style="transition: all 0.2s ease"
        >
          <q-item-section avatar>
            <q-avatar size="sm">
              <q-img src="@/assets/favicon.svg" alt="Logo" />
            </q-avatar>
          </q-item-section>
          <q-item-section>
            <span
              :style="{
                color: '#4b5563',
                fontWeight: '600',
              }"
            >
              PriceLint
            </span>
          </q-item-section>
        </q-item>
        <q-separator class="q-my-md" />
        <q-scroll-area style="height: calc(100vh - 160px)">
          <!-- First Section -->
          <q-item-label header class="text-caption text-grey-7"
            >Overview</q-item-label
          >
          <q-item
            v-for="(item, index) in menuItems"
            :key="index"
            clickable
            v-ripple
            :to="item.route"
            :active="item.route === $route.path"
            active-class="bg-grey-2"
            class="q-my-xs q-px-md rounded-borders"
            style="transition: all 0.2s ease"
          >
            <q-item-section avatar>
              <q-icon :name="item.icon" />
            </q-item-section>
            <q-item-section>
              <span
                :style="{
                  color: item.route === $route.path ? '#1f2937' : '#4b5563',
                  fontWeight: item.route === $route.path ? '600' : '500',
                }"
              >
                {{ item.label }}
              </span>
            </q-item-section>
            <div
              v-if="item.route === $route.path"
              class="active-indicator"
            ></div>
          </q-item>
        </q-scroll-area>
      </q-list>
    </q-drawer>

    <!-- Sidebar Footer: User Profile Dropdown -->
    <div class="absolute-bottom" v-if="drawerOpen && !$q.screen.lt.md">
      <q-separator />
      <div class="q-pa-md">
        <q-item class="rounded-borders">
          <q-item-section class="connection-panel">
            <div class="flex items-center justify-between full-width q-pa-sm">
              <!-- 左侧：Connected + 呼吸灯 -->
              <div class="flex items-center q-gutter-xs">
                <div class="breathing-dot"></div>
                <span class="text-caption">On</span>
              </div>

              <!-- 中间分隔线 -->
              <q-separator vertical style="height: 20px" />

              <!-- 右侧：Principal ID -->
              <div class="flex items-center q-gutter-xs">
                <span class="text-caption text-grey-7">{{ showUser }}</span>
                <q-icon
                  name="content_copy"
                  class="cursor-pointer"
                  @click.stop="copyPid()"
                />
              </div>
            </div>
          </q-item-section>
          <q-item-section side>
            <q-btn flat dense round>
              <q-icon
                name="keyboard_arrow_down"
                class="rotate-icon"
                :class="{ 'rotate-active': userMenu }"
              />
              <q-menu v-model="userMenu" anchor="top start" self="bottom start">
                <q-list>
                  <template v-for="(item, index) in userMenuItems" :key="index">
                    <q-item
                      clickable
                      v-close-popup
                      @click="item.click ? item.click() : null"
                    >
                      <q-item-section avatar>
                        <q-icon :name="item.icon" />
                      </q-item-section>
                      <q-item-section>{{ item.label }}</q-item-section>
                    </q-item>
                    <!-- <q-separator v-if="item.separator" /> -->
                  </template>
                </q-list>
              </q-menu>
            </q-btn>
          </q-item-section>
        </q-item>
      </div>
    </div>
    <!-- Main Content Area -->
    <q-page-container
      class="bg-grey-11"
      :class="{ 'q-pa-sm': !$q.screen.lt.md, 'q-pa-none': $q.screen.lt.md }"
    >
      <q-page
        class="bg-white"
        :style="{
          border: '1px solid #e5e7eb',
          borderRadius: $q.screen.lt.md ? '0' : '0.5rem',
          minHeight: $q.screen.lt.md ? '100vh' : 'calc(100vh - 16px)',
        }"
      >
        <!-- 面包屑占位 -->
        <div v-if="!showBreadcrumbs" style="height: 30px"></div>
        <q-breadcrumbs v-if="showBreadcrumbs" class="breadcrumb-content">
          <q-breadcrumbs-el
            v-for="(item, index) in breadcrumbItems"
            :key="index"
            :label="item.label"
            :to="item.to"
          />
          <!-- 面包屑按钮区域 -->
          <div v-if="showButtons" class="q-ml-md">
            <q-btn
              v-for="btn in buttons"
              :key="btn.label"
              :label="btn.label"
              :to="btn.to"
              flat
              no-caps
              dense
              class="text-grey-8 rounded-borders q-mr-sm q-px-sm q-py-xs"
              :ripple="false"
              :class="{
                'bg-grey-3': $route.path === btn.to,
                'text-grey-10': $route.path === btn.to,
              }"
              unelevated
            />
          </div>
        </q-breadcrumbs>

        <q-separator v-if="showBreadcrumbs" />

        <router-view class="container" />
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import { initAuth, signOut } from "@/api/auth";
import { clearCurrentIdentity, setCurrentIdentity } from "@/api/canister_pool";
import { getUserAutoRegister } from "@/api/user";
import { goHome } from "@/router/routers";
import { useUserStore } from "@/stores/user";
import {
  extractColorByName,
  showAvatarName,
  showUsername,
} from "@/utils/avatars";
import { copyText } from "@/utils/common";
import { showMessageError } from "@/utils/message";
import { useQuasar } from "quasar";
import { computed, onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const $q = useQuasar();
const userStore = useUserStore();
const router = useRouter();
const route = useRoute();

const menuItems = [
  { icon: "analytics", label: "Dashboard", route: "/app" },
  { icon: "memory", label: "Canisters", route: "/app/canisters" },
  // { icon: "settings", label: "Settings", route: "/app/settings" },
];

const drawerOpen = ref(false);
const userMenu = ref(false);
// 与 II 认证相关的信息
const clientReady = ref(false);
const signedIn = ref(false); // 是否登录

const principal = computed(() => userStore.principal);
const username = ref();

onMounted(() => {
  doInitAuth();
});
// 判断是否显示按钮（匹配 /app/canisters/:canisterId 及其子路由）
const showButtons = computed(() => {
  const regex = /^\/app\/canisters\/[^/]+(\/.*)?$/;
  return regex.test(route.path);
});

// 动态生成按钮
const buttons = computed(() => {
  if (!showButtons.value) return [];
  // 获取当前 canister ID
  const canisterId = route.params.canisterId as string | undefined;
  if (!canisterId) return [];

  return [
    {
      label: "Overview",
      to: `/app/canisters/${canisterId}`,
    },
    {
      label: "Insights",
      to: `/app/canisters/${canisterId}/insights`,
    },
    {
      label: "Settings",
      to: `/app/canisters/${canisterId}/edit`,
    },
  ];
});

// 显示面包屑的条件
const showBreadcrumbs = computed(() => {
  return route.path.startsWith("/app/canisters/");
});

// 通用面包屑生成逻辑，仅显示到 canister ID
const breadcrumbItems = computed((): { label: string; to: string }[] => {
  const parts = route.path.split("/").filter((part) => part);
  const items: { label: string; to: string }[] = [];
  let currentPath = "";

  // 只处理到 /app/canisters/:canisterid
  const maxParts = Math.min(parts.length, 3); // 限制为 app, canisters, :canisterid
  for (let i = 0; i < maxParts; i++) {
    currentPath += `/${parts[i]}`;
    const label = parts[i];
    // 最后一项（canister ID）不设置 to
    const to = i === maxParts - 1 ? "" : currentPath;
    items.push({ label, to });
  }

  return items;
});

const doInitAuth = () => {
  initAuth().then((ai) => {
    clientReady.value = true;
    if (ai.info) {
      setCurrentIdentity(ai.info.identity, ai.info.principal);
      // 保存 principal 到用户信息状态
      userStore.setPrincipal(ai.info.principal).then(() =>
        // 获取用户信息
        getUserInfoFromServices()
      );
    }
  });
};

//从后台获取用户信息，并且设置
const getUserInfoFromServices = () => {
  getUserAutoRegister()
    .then((info) => {
      if (info.Ok) {
        username.value = info.Ok.name;
      } else if (info.Err) {
        console.error("no information for unregister user: ", info);
      } else {
        throw new Error("info not ok & info not err");
      }
    })
    .catch((e) => {
      console.error("mounted get user info failed: ", e);
      showMessageError("mounted get user info failed: " + e);
      onLogOut();
    });
};

const copyPid = () => {
  copyText(principal.value);
};

const onLogOut = async () => {
  console.log("onLogout");
  const auth = await initAuth();
  signedIn.value = false;
  clearCurrentIdentity();
  await signOut(auth.client);

  goHome(router);
  // TODO 返回首页还要刷新页面，感觉不是很友好
  //返回首页后，刷新页面，防止出现缓存问题。
  // 如果不刷新页面，会导致A用户登出后，再登录B用户的账号，结果会读取A用户缓存的问题
  setTimeout(() => {
    window.location.reload();
  }, 500);
};

const toggleDrawer = () => {
  drawerOpen.value = !drawerOpen.value;
};
const showAvatar = computed<string>(() => {
  const m = showAvatarName("", principal.value);
  return m ? m : "A";
});
// 根据名字，定义头像颜色
const backgroundColor = computed<string>(() => {
  return extractColorByName(principal.value);
});
// 根据名字，定义用户名
const showUser = computed<string>(() => {
  return showUsername("", principal.value);
});

// 定义用户菜单项数据
const userMenuItems = [
  // {
  //   label: "Profile",
  //   to: "/profile",
  //   icon: "person",
  // },
  // {
  //   label: "Settings",
  //   to: "/app/settings",
  //   icon: "settings",
  //   separator: true, // 仅在此项后添加分隔线
  // },
  {
    label: "Logout",
    click: onLogOut,
    icon: "logout",
  },
];
</script>

<style lang="scss" scoped>
.connection-panel {
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  background: #fafafa;
}

.breathing-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #4caf50;
  animation: breathing 2s ease-in-out infinite;
}

@keyframes breathing {
  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.7;
    transform: scale(1.1);
  }
}
/* 自定义边框类 */
.header {
  border-bottom: 1px solid #e5e7eb; /* 浅灰色边框，模仿 Catalyst UI */
}
.active-indicator {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 40px;
  background-color: #667eea;
  border-radius: 2px 0 0 2px;
}
.drawer-container {
  .q-item__section--avatar {
    color: inherit;
    min-width: 36px;
    padding-right: 0;
  }
}

.absolute-bottom {
  position: fixed; /* Fix to viewport bottom */
  bottom: 0;
  left: 0;
  width: 256px; /* Match q-drawer width */
  background: inherit; /* Inherit bg-grey-1 from q-drawer */
  z-index: 5000; /* Ensure it stays above content */
}
.rotate-icon {
  transition: transform 0.3s ease;
}
.rotate-icon.rotate-active {
  transform: rotate(180deg);
}
.breadcrumb-content {
  padding: 14px 22px;
}
</style>
