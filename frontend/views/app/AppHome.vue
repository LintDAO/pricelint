<template>
  <q-layout view="lHh lpR lFr">
    <!-- Mobile Navbar (visible on smaller screens) -->
    <q-header elevated class="bg-primary text-white" v-if="$q.screen.lt.md">
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleDrawer"
        />
        <q-space />
        <q-btn flat label="Logout" @click="onLogOut()" />
      </q-toolbar>
    </q-header>

    <!-- Sidebar (Drawer) -->
    <q-drawer
      v-model="drawerOpen"
      show-if-above
      :width="256"
      class="bg-grey-11"
      :breakpoint="769"
    >
      <q-scroll-area class="fit">
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
                  fontWeight: '500',
                }"
              >
                PriceLint
              </span>
            </q-item-section>
          </q-item>
          <q-separator class="q-my-md" />
          <!-- First Section -->
          <q-item-label header class="text-caption text-grey-7"
            >Overview</q-item-label
          >
          <q-item
            v-for="(item, index) in menuItems.slice(0, 2)"
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
        </q-list>

        <!-- Sidebar Footer: User Profile Dropdown -->
        <div class="absolute-bottom">
          <q-separator />
          <q-item clickable v-ripple>
            <q-item-section avatar>
              <q-avatar
                :style="{ background: backgroundColor, color: 'white' }"
              >
                {{ showAvatar }}
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ username || showUser }}</q-item-label>
              <q-item-label caption>{{
                principal ? principal.slice(0, 10) + "..." : ""
              }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-btn
                flat
                dense
                round
                icon="keyboard_arrow_up"
                @click="userMenu = true"
              />
              <q-menu v-model="userMenu" anchor="top start" self="bottom start">
                <q-list style="min-width: 200px">
                  <q-item clickable v-close-popup to="/profile">
                    <q-item-section>Profile</q-item-section>
                  </q-item>
                  <q-item clickable v-close-popup to="/app/settings">
                    <q-item-section>Settings</q-item-section>
                  </q-item>
                  <q-separator />
                  <q-item clickable v-close-popup @click="onLogOut">
                    <q-item-section>Logout</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-item-section>
          </q-item>
        </div>
      </q-scroll-area>
    </q-drawer>

    <!-- Main Content Area -->
    <q-page-container
      class="bg-grey-11"
      :class="{ 'q-pa-sm': !$q.screen.lt.md, 'q-pa-none': $q.screen.lt.md }"
    >
      <q-page
        padding
        class="bg-white"
        :style="{
          border: '1px solid #e5e7eb',
          borderRadius: '0.5rem',
          minHeight: $q.screen.lt.md ? '100vh' : 'calc(100vh - 16px)',
        }"
      >
        <router-view />
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
import { showMessageError, showMessageSuccess } from "@/utils/message";
import { copyToClipboard, useQuasar } from "quasar";
import { computed, onMounted, ref } from "vue";
import { useRouter } from "vue-router";

const $q = useQuasar();
const userStore = useUserStore();
const router = useRouter();

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
  copyToClipboard(principal.value)
    .then(() => {
      showMessageSuccess(`copy ${principal.value} success`);
    })
    .catch(() => {
      showMessageError("copy failed");
    });
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
</script>

<style lang="scss" scoped>
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
</style>
