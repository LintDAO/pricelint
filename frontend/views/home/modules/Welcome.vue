<template>
  <section
    class="hero-section q-pa-xl"
    style="
      min-height: 100vh;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    "
  >
    <div class="row justify-center items-center" style="min-height: 80vh">
      <div class="col-12 col-md-10 col-lg-8">
        <!-- Main Hero Content -->
        <div class="text-center q-mb-xl">
          <!-- Animated Badge -->
          <q-chip
            :ripple="false"
            class="q-my-lg hero-badge"
            style="
              background: rgba(255, 255, 255, 0.2);
              backdrop-filter: blur(10px);
            "
          >
            <q-avatar>
              <q-icon name="trending_up" color="white" />
            </q-avatar>
            <span class="text-white text-weight-medium"
              >AI-Powered Predictions</span
            >
          </q-chip>

          <!-- Main Title with Typing Effect -->
          <h1 class="hero-title text-white q-my-md">
            <div class="text-h2 text-weight-bold q-mb-sm">
              {{ displayedText }}
              <span
                class="cursor"
                v-show="showCursor"
                :class="{ blink: showCursor }"
                >|</span
              >
            </div>
            <div class="text-h4 text-weight-light" style="opacity: 0.9">
              We’ve Got You Covered
            </div>
          </h1>

          <!-- Subtitle -->
          <p
            class="text-h6 text-white"
            style="opacity: 0.8; max-width: 600px; margin: 48px auto"
          >
            Harness the power of on-chain AI and LSTM models to make informed
            trading decisions with real-time predictive analytics.
          </p>

          <!-- CTA Buttons -->
          <div class="row justify-center q-gutter-md q-mb-xl">
            <q-btn
              unelevated
              rounded
              size="lg"
              class="q-px-xl q-py-md cta-primary"
              style="background: rgba(255, 255, 255, 0.9); color: #667eea"
              @click="scrollToPredictions"
            >
              <q-icon name="analytics" class="q-mr-sm" />
              <span class="text-weight-bold">View Predictions</span>
            </q-btn>
            <!-- <q-btn
              outline
              rounded
              size="lg"
              color="white"
              class="q-px-xl q-py-md"
            >
              <q-icon name="play_arrow" class="q-mr-sm" />
              <span class="text-weight-medium">Watch Demo</span>
            </q-btn> -->
          </div>

          <!-- Stats Row -->
          <div class="row justify-center q-gutter-lg stats-container">
            <div class="col-auto text-center">
              <div class="text-h4 text-weight-bold text-white">
                {{ animatedAccuracy }}%
              </div>
              <div class="text-caption text-white" style="opacity: 0.8">
                Prediction Accuracy
              </div>
            </div>
            <div class="col-auto text-center">
              <div class="text-h4 text-weight-bold text-white">
                {{ animatedAssets }}+
              </div>
              <div class="text-caption text-white" style="opacity: 0.8">
                Crypto Assets
              </div>
            </div>
            <div class="col-auto text-center">
              <div class="text-h4 text-weight-bold text-white">
                {{ animatedPredictions }}+
              </div>
              <div class="text-caption text-white" style="opacity: 0.8">
                Daily Predictions
              </div>
            </div>
          </div>
        </div>

        <!-- Floating Cards Preview -->
        <div class="row justify-center q-gutter-md">
          <div class="col-auto">
            <q-card
              class="floating-card q-pa-md"
              style="
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(20px);
                border: 1px solid rgba(255, 255, 255, 0.2);
              "
            >
              <div class="row items-center q-gutter-sm">
                <q-avatar size="32px">
                  <q-img src="@/assets/icons/BTC.svg" alt="BTC" />
                </q-avatar>
                <div>
                  <div class="text-white text-weight-medium">BTC</div>
                  <div class="text-caption text-white" style="opacity: 0.8">
                    55% Accuracy
                  </div>
                </div>
                <q-icon name="trending_up" color="positive" size="20px" />
              </div>
            </q-card>
          </div>

          <div class="col-auto">
            <q-card
              class="floating-card q-pa-md"
              style="
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(20px);
                border: 1px solid rgba(255, 255, 255, 0.2);
              "
            >
              <div class="row items-center q-gutter-sm">
                <q-avatar
                  size="32px"
                  style="
                    background: linear-gradient(to right, #6b5b95, #4b0082);
                  "
                >
                  <q-img src="@/assets/dfinity.svg" alt="ICP" />
                </q-avatar>
                <div>
                  <div class="text-white text-weight-medium">ICP</div>
                  <div class="text-caption text-white" style="opacity: 0.8">
                    61% Accuracy
                  </div>
                </div>
                <q-icon name="trending_up" color="positive" size="20px" />
              </div>
            </q-card>
          </div>
        </div>
      </div>
    </div>

    <!-- Scroll Indicator -->
    <div class="text-center scroll-indicator">
      <q-btn
        flat
        round
        color="white"
        icon="keyboard_arrow_down"
        size="lg"
        @click="scrollToPredictions"
        class="animate-bounce"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

// Typing effect
const fullText = "Crypto Trends: Up or Down";
const displayedText = ref("");
const showCursor = ref(true);
let typingInterval: NodeJS.Timeout | null = null;
let cursorInterval: NodeJS.Timeout | null = null;

// Animated numbers
const animatedAccuracy = ref(0);
const animatedAssets = ref(0);
const animatedPredictions = ref(0);

const startTypingEffect = () => {
  let index = 0;
  typingInterval = setInterval(() => {
    if (index < fullText.length) {
      displayedText.value = fullText.slice(0, index + 1);
      index++;
    } else {
      if (typingInterval) clearInterval(typingInterval);
      if (cursorInterval) clearInterval(cursorInterval); // 停止光标闪烁
      showCursor.value = false; // 隐藏光标
    }
  }, 100);
};

const startCursorBlink = () => {
  cursorInterval = setInterval(() => {
    showCursor.value = !showCursor.value;
  }, 500);
};

const animateNumbers = () => {
  // Animate accuracy to xx %
  const accuracyInterval = setInterval(() => {
    if (animatedAccuracy.value < 63) {
      animatedAccuracy.value += 1;
    } else {
      clearInterval(accuracyInterval);
    }
  }, 30);

  // Animate assets to x +
  const assetsInterval = setInterval(() => {
    if (animatedAssets.value < 2) {
      animatedAssets.value += 1;
    } else {
      clearInterval(assetsInterval);
    }
  }, 40);

  // Animate predictions to 24 +
  const predictionsInterval = setInterval(() => {
    if (animatedPredictions.value < 24) {
      animatedPredictions.value += 2;
    } else {
      clearInterval(predictionsInterval);
    }
  }, 25);
};

const scrollToPredictions = () => {
  // 滚动到预测表格区域
  const element = document.querySelector(".predictions-table"); // 你需要给表格添加这个class
  if (element) {
    element.scrollIntoView({ behavior: "smooth" });
  } else {
    // 如果没有找到表格，就滚动一个屏幕高度
    window.scrollBy({ top: window.innerHeight, behavior: "smooth" });
  }
};

onMounted(() => {
  setTimeout(() => {
    startTypingEffect();
    startCursorBlink();
    setTimeout(() => {
      animateNumbers();
    }, 1000);
  }, 500);
});

onUnmounted(() => {
  if (typingInterval) clearInterval(typingInterval);
  if (cursorInterval) clearInterval(cursorInterval);
});
</script>

<style scoped>
.hero-section {
  position: relative;
  overflow: hidden;
}

.hero-section::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grain" width="100" height="100" patternUnits="userSpaceOnUse"><circle cx="25" cy="25" r="1" fill="white" opacity="0.1"/><circle cx="75" cy="75" r="1" fill="white" opacity="0.1"/><circle cx="50" cy="10" r="0.5" fill="white" opacity="0.1"/><circle cx="10" cy="60" r="0.5" fill="white" opacity="0.1"/><circle cx="90" cy="40" r="0.5" fill="white" opacity="0.1"/></pattern></defs><rect width="100" height="100" fill="url(%23grain)"/></svg>');
  pointer-events: none;
}

.hero-badge {
  animation: fadeInUp 0.8s ease-out;
}

.hero-title {
  animation: fadeInUp 0.8s ease-out 0.2s both;
}

.cursor {
  display: inline-block;
  color: white;
}

.cursor.blink {
  animation: blink 1s infinite;
}

@keyframes blink {
  0%,
  50% {
    opacity: 1;
  }
  51%,
  100% {
    opacity: 0;
  }
}

.cta-primary {
  transition: all 0.3s ease;
}

.cta-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
}

.stats-container {
  animation: fadeInUp 0.8s ease-out 0.6s both;
}

.floating-card {
  animation: float 3s ease-in-out infinite;
  border-radius: 12px;
}

.floating-card:nth-child(1) {
  animation-delay: 0s;
}

.floating-card:nth-child(2) {
  animation-delay: 1s;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-10px);
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.scroll-indicator {
  position: absolute;
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%);
}

.animate-bounce {
  animation: bounce 2s infinite;
}

@keyframes bounce {
  0%,
  20%,
  53%,
  80%,
  100% {
    transform: translate3d(0, 0, 0);
  }
  40%,
  43% {
    transform: translate3d(0, -15px, 0);
  }
  70% {
    transform: translate3d(0, -7px, 0);
  }
  90% {
    transform: translate3d(0, -2px, 0);
  }
}

@media (max-width: 768px) {
  .hero-title .text-h2 {
    font-size: 2rem;
  }

  .hero-title .text-h4 {
    font-size: 1.2rem;
  }

  .stats-container {
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
