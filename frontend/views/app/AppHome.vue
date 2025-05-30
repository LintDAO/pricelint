<template>
  <q-layout view="lHh lpR lFr" class="home-container">
    <q-header bordered class="open-header text-dark">
      <q-toolbar>
        <q-btn dense flat round icon="menu" @click="toggleLeftDrawer" />
        <q-toolbar-title>
          <q-avatar>
            <img src="@/assets/dfinity.svg" />
          </q-avatar>
          IC PriceLint
        </q-toolbar-title>
        <q-btn color="primary" label="Logout" @click="onLogOut()" />
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      show-if-above
      :breakpoint="769"
    >
      <q-img
        class="absolute-top"
        src="@/assets/material.png"
        style="height: 150px"
      >
        <div class="absolute-bottom bg-transparent">
          <q-avatar size="56px" class="q-mb-sm" :style="{ backgroundColor }">
            {{ showAvatar }}
          </q-avatar>
          <div class="text-weight-bold">
            {{ showUser }}
            <q-icon
              name="content_copy"
              class="cursor-pointer"
              @click="copyPid()"
            />
          </div>
          <div>@user</div>
        </div>
      </q-img>
      <q-scroll-area
        style="
          height: calc(100% - 150px);
          margin-top: 150px;
          border-right: 1px solid #ddd;
        "
      >
        <q-list>
          <q-item
            v-for="(item, index) in menuItems"
            :key="index"
            clickable
            v-ripple="true"
            :disable="item.disable"
            :to="item.route"
            :active="item.route === $route.path"
          >
            <q-item-section avatar>
              <q-icon :name="item.icon" />
            </q-item-section>
            <q-item-section>
              {{ item.label }}
            </q-item-section>
          </q-item>
        </q-list>
        <div class="bottom-icon q-pa-md">
          <a :href="DISCORD_URL" target="_Blank">
            <q-icon class="cursor-pointer" size="md" name="discord" />
          </a>
        </div>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <div class="q-pa-md q-gutter-md">
        <router-view />
      </div>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import { initAuth, signOut } from "@/api/auth";
import { clearCurrentIdentity, setCurrentIdentity } from "@/api/canister_pool";
import { DISCORD_URL } from "@/api/constants/docs";
import { getUserAutoRegister } from "@/api/user";
import { goHome } from "@/router/routers";
import { useUserStore } from "@/stores/user";
import {
  extractColorByName,
  showAvatarName,
  showUsername,
} from "@/utils/avatars";
import { showMessageError, showMessageSuccess } from "@/utils/message";
import { copyToClipboard } from "quasar";
import { computed, onMounted, provide, ref } from "vue";
import { useRouter } from "vue-router";

const userStore = useUserStore();
const router = useRouter();

const menuItems = [
  { icon: "drafts", label: "Dashboard", route: "/app" },
  { icon: "wallet", label: "Canisters", route: "/app/canisters" },
  { icon: "settings", label: "Settings", route: "/app/settings" },
];
const leftDrawerOpen = ref(false);
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

const toggleLeftDrawer = () => {
  leftDrawerOpen.value = !leftDrawerOpen.value;
};
const showAvatar = computed<string>(() => {
  const m = showAvatarName(username.value, principal.value);
  return m ? m : "A";
});
// 根据名字，定义头像颜色
const backgroundColor = computed<string>(() => {
  return extractColorByName(username.value);
});
// 根据名字，定义用户名
const showUser = computed<string>(() => {
  return showUsername(username.value, principal.value);
});
</script>

<style lang="scss">
.home-container {
  min-height: 100vh;
  .q-header {
    background-color: #0000001a;
    -webkit-backdrop-filter: blur(7px);
    backdrop-filter: blur(7px);
  }
  .bottom-icon {
    position: absolute;
    bottom: 0;
    a {
      color: grey !important;
    }
  }
}
</style>
