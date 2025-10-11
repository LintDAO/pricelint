<template>
  <div class="navbar-container" :class="{ 'navbar-scrolled': isScrolled }">
    <q-toolbar class="q-px-lg">
      <!-- Left Side - Logo -->
      <div class="navbar-logo">
        <q-btn flat no-caps class="logo-btn q-pa-sm" @click="scrollToTop">
          <q-avatar size="40px" class="q-mr-sm logo-avatar">
            <q-img src="@/assets/favicon.svg" alt="BTC" />
          </q-avatar>
          <div class="logo-text">
            <div class="text-h6 text-weight-bold text-white">CryptoAI</div>
            <div
              class="text-caption text-white"
              style="opacity: 0.8; line-height: 1"
            >
              PriceLint
            </div>
          </div>
        </q-btn>
      </div>

      <!-- Center - Navigation Links (Desktop) -->
      <q-space />
      <!-- <div class="navbar-links gt-sm">
        <q-btn
          flat
          no-caps
          class="nav-link q-px-md"
          @click="scrollToPredictions"
        >
          <q-icon name="trending_up" class="q-mr-xs" size="18px" />
          <span>Predictions</span>
        </q-btn>
        <q-btn flat no-caps class="nav-link q-px-md" @click="scrollToFeatures">
          <q-icon name="auto_awesome" class="q-mr-xs" size="18px" />
          <span>Features</span>
        </q-btn>
      </div>
      <q-space /> -->

      <!-- Right Side - CTA Button -->
      <div class="navbar-cta">
        <q-btn
          unelevated
          rounded
          no-caps
          class="cta-button q-px-lg q-py-sm"
          style="background: rgba(255, 255, 255, 0.9); color: #667eea"
          @click="onLogin()"
          :loading="loading"
        >
          <q-icon name="rocket_launch" class="q-mr-sm" size="18px" />
          <span class="text-weight-bold">Start Predict</span>
          <q-tooltip class="bg-white text-grey-9">
            Begin your AI-powered predictions
          </q-tooltip>
        </q-btn>

        <!-- Status Indicator -->
        <q-chip
          class="q-ml-md status-chip gt-xs"
          :ripple="false"
          style="background: rgba(76, 175, 80, 0.2)"
        >
          <q-avatar>
            <div class="status-dot"></div>
          </q-avatar>
          <span class="text-white text-caption">Live</span>
        </q-chip>
      </div>
    </q-toolbar>
  </div>
</template>

<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth";
import { setCurrentIdentity } from "@/api/canister_pool";
import { useUserStore } from "@/stores/user";
import { onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userStore = useUserStore();
const isScrolled = ref(false);
const showMobileMenu = ref(false);
// 与 II 认证相关的信息
const signedIn = ref(false); // 是否登录

const loading = ref(false);
const onLogin = async () => {
  loading.value = true;
  const auth = await initAuth();
  //TODO 先不使用登录缓存，有点问题
  // if (!auth.info) {
  //检查用户是否已登录，未登录就登录
  signIn(auth.client) // 理论上有链接对象才会进入这个方法
    .then((ii) => {
      signedIn.value = true;
      auth.info = ii;
      loginSuccess(ii);
    })
    .catch((e) => {
      console.error("e", e);
      loading.value = false;
    })
    .finally(() => {
      // loading.value = false;
    });
  // } else {
  //   //存在auth.info，说明用户已登录，不需要再登录
  //   loginSuccess(auth.info)
  // }
};

const loginSuccess = (ii: IdentityInfo) => {
  // 保存登录状态到actor，方便调用
  setCurrentIdentity(ii.identity, ii.principal);
  // 保存 principal 到状态
  userStore.setPrincipal(ii.principal).then(() => {
    loading.value = false;
    //直接跳转到应用中，在应用里获取userInfo，加快速度。
    router.push({
      path: "/app",
    });
  });
};

const handleScroll = () => {
  isScrolled.value = window.scrollY > 50;
};

const scrollToTop = () => {
  window.scrollTo({ top: 0, behavior: "smooth" });
};

onMounted(() => {
  window.addEventListener("scroll", handleScroll);
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});
</script>

<style scoped>
.navbar-container {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  backdrop-filter: blur(20px);
  background: rgba(102, 126, 234, 0.1);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.navbar-scrolled {
  background: rgba(102, 126, 234, 0.95);
  backdrop-filter: blur(20px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.navbar-logo .logo-btn {
  transition: all 0.3s ease;
  border-radius: 12px;
}

.navbar-logo .logo-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-1px);
}

.logo-avatar {
  background: linear-gradient(45deg, #667eea, #764ba2);
  transition: all 0.3s ease;
}

.navbar-logo .logo-btn:hover .logo-avatar {
  transform: rotate(5deg) scale(1.05);
}

.navbar-links {
  display: flex;
  gap: 8px;
}

.nav-link {
  color: white;
  transition: all 0.3s ease;
  border-radius: 8px;
  position: relative;
  overflow: hidden;
}

.nav-link::before {
  content: "";
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.1);
  transition: left 0.3s ease;
}

.nav-link:hover::before {
  left: 0;
}

.nav-link:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-1px);
}

.cta-button {
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
}

.cta-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15);
  background: rgba(255, 255, 255, 1) !important;
}

.status-chip {
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.status-dot {
  width: 8px;
  height: 8px;
  background: #4caf50;
  border-radius: 50%;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(76, 175, 80, 0.7);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(76, 175, 80, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(76, 175, 80, 0);
  }
}

.mobile-menu-card {
  position: relative;
  height: 100vh;
}

.mobile-menu-card::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grain" width="100" height="100" patternUnits="userSpaceOnUse"><circle cx="25" cy="25" r="1" fill="white" opacity="0.05"/><circle cx="75" cy="75" r="1" fill="white" opacity="0.05"/></pattern></defs><rect width="100" height="100" fill="url(%23grain)"/></svg>');
  pointer-events: none;
}

.mobile-nav-link {
  color: white;
  width: 100%;
  justify-content: flex-start;
  border-radius: 12px;
  transition: all 0.3s ease;
}

.mobile-nav-link:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateX(8px);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .logo-text .text-h6 {
    font-size: 1rem;
  }

  .logo-text .text-caption {
    font-size: 0.7rem;
  }
}

/* 确保页面内容不被固定导航栏遮挡 */
body {
  padding-top: 70px;
}
</style>
