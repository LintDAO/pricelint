<template>
  <div class="color-mask">
    <div class="container login-container">
      <div class="description row">
        <div class="col-12 col-md-6">
          <h4 style="margin-top: 0; margin-bottom: 10px">
            Welcome to TallyLint🧾
          </h4>
          <div class="q-mb-xl">
            <div>
              <span class="text-body1">
                TallyLint is a 100% decentralized on-chain app that enables ICP
                users to track the value of every transaction with precision.
              </span>
              <br />
              <span class="text-body1">
                Offering private and secure services through smart contracts,
                TallyLint makes asset management effortless and efficient.
              </span>
            </div>
            <q-btn
              color="primary"
              class="login-button"
              @click="onLogin()"
              :loading="loading"
              no-caps
            >
              Launch app
            </q-btn>
          </div>
          <div class="logo">
            <img alt="logo" src="@/assets/on-chain.svg" />
          </div>
        </div>
        <div class="col-12 col-md-6">
          <q-img src="@/assets/images/intro.png"></q-img>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth"
import { setCurrentIdentity } from "@/api/canister_pool"
import { useUserStore } from "@/stores/user"
import { checkDomain } from "@/utils/common"
import { onMounted, ref } from "vue"
import { useRouter } from "vue-router"

const router = useRouter()
const userStore = useUserStore()
onMounted(() => {
  // getOKInfo()
  checkDomain()
})
// 与 II 认证相关的信息
const signedIn = ref(false) // 是否登录

const loading = ref(false)

const onLogin = async () => {
  const auth = await initAuth()
  loading.value = true
  //TODO 先不使用登录缓存，有点问题
  // if (!auth.info) {
  //检查用户是否已登录，未登录就登录
  signIn(auth.client) // 理论上有链接对象才会进入这个方法
    .then((ii) => {
      signedIn.value = true
      auth.info = ii
      loginSuccess(ii)
    })
    .catch((e) => {
      console.error("e", e)
    })
    .finally(() => {
      loading.value = false
    })
  // } else {
  //   //存在auth.info，说明用户已登录，不需要再登录
  //   loginSuccess(auth.info)
  // }
}

const enableTwitterAds = () => {
  // 调用 Twitter 广告跟踪事件
  //@ts-ignore
  window.twq("event", "tw-opr1q-opr2m", {})
}

const loginSuccess = (ii: IdentityInfo) => {
  // 保存登录状态到actor，方便调用
  setCurrentIdentity(ii.identity, ii.principal)
  // 保存 principal 到状态
  userStore.setPrincipal(ii.principal).then(() => {
    enableTwitterAds()
    //直接跳转到应用中，在应用里获取userInfo，加快速度。
    router.push({
      path: "/app",
    })
  })
}
</script>

<style lang="scss" scoped>
.color-mask {
  overflow: hidden;
  position: relative;
  &::before {
    content: "";
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 50%; // 你可以根据需要调整大小
    height: 200px; // 半圆的高度是圆的半径
    background: radial-gradient(
      circle,
      rgba(192, 217, 255, 0.5),
      rgba(240, 185, 229, 0.5)
    );
    border-radius: 100px;
    filter: blur(300px); // 模糊效果
  }
}
.login-container {
  padding-top: 100px;
  padding-bottom: 250px;

  .description > div {
    position: relative;
  }
  .login-button {
    margin-top: 50px;
  }
  .logo {
    position: absolute;
    bottom: 0;
    left: 0;
  }
}
</style>
