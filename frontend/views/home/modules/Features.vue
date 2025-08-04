<template>
  <section
    class="q-pa-xl"
    style="background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)"
  >
    <div class="row justify-center container">
      <!-- Header -->
      <div class="text-center header">
        <h2 class="text-h3 text-weight-bold q-mb-xl text-primary">
          Revolutionary
          <span class="text-secondary">AI Infrastructure</span>
        </h2>
        <p class="text-h6 text-grey-7 q-mb-none">
          Experience the future of decentralized AI with our cutting-edge
          platform that combines blockchain technology with advanced machine
          learning capabilities.
        </p>
      </div>

      <!-- Features Grid -->
      <div class="row q-col-gutter-lg features">
        <!-- On-Chain AI Feature -->
        <div class="col-12 col-md-6">
          <q-card
            class="feature-card q-pa-lg"
            flat
            bordered
            @mouseenter="onHover(0)"
            @mouseleave="onLeave(0)"
            :class="{ 'feature-card-hover': hoveredCard === 0 }"
          >
            <div class="row items-center q-mb-md">
              <q-avatar
                size="48px"
                class="q-mr-md"
                style="background: linear-gradient(45deg, #1976d2, #42a5f5)"
              >
                <q-icon name="memory" color="white" size="24px" />
              </q-avatar>
              <div class="text-h5 text-weight-bold text-grey-9">
                On-Chain AI
              </div>
            </div>
            <p class="text-body1 text-grey-7 q-mb-md">
              LSTM models run directly in Canisters using the ic-burn framework,
              enabling true chain-native inference without external
              dependencies.
            </p>
            <!-- <q-btn
              flat
              no-caps
              color="primary"
              icon-right="arrow_forward"
              class="q-pa-none"
            >
              Learn more
            </q-btn> -->
          </q-card>
        </div>

        <!-- One-Click Deployment Feature -->
        <div class="col-12 col-md-6">
          <q-card
            class="feature-card q-pa-lg"
            flat
            bordered
            @mouseenter="onHover(1)"
            @mouseleave="onLeave(1)"
            :class="{ 'feature-card-hover': hoveredCard === 1 }"
          >
            <div class="row items-center q-mb-md">
              <q-avatar
                size="48px"
                class="q-mr-md"
                style="background: linear-gradient(45deg, #388e3c, #66bb6a)"
              >
                <q-icon name="rocket_launch" color="white" size="24px" />
              </q-avatar>
              <div class="text-h5 text-weight-bold text-grey-9">
                One-Click Deployment
              </div>
            </div>
            <p class="text-body1 text-grey-7 q-mb-md">
              Deploy multiple Canisters effortlessly with pre-configured models
              or customize your own. Get up and running in minutes, not hours.
            </p>
            <!-- <q-btn
              flat
              no-caps
              color="positive"
              icon-right="arrow_forward"
              class="q-pa-none"
            >
              Start deploying
            </q-btn> -->
          </q-card>
        </div>

        <!-- Real-Time Data Streams Feature -->
        <div class="col-12 col-md-6">
          <q-card
            class="feature-card q-pa-lg"
            flat
            bordered
            @mouseenter="onHover(2)"
            @mouseleave="onLeave(2)"
            :class="{ 'feature-card-hover': hoveredCard === 2 }"
          >
            <div class="row items-center q-mb-md">
              <q-avatar
                size="48px"
                class="q-mr-md"
                style="background: linear-gradient(45deg, #7b1fa2, #ba68c8)"
              >
                <q-icon name="timeline" color="white" size="24px" />
              </q-avatar>
              <div class="text-h5 text-weight-bold text-grey-9">
                Real-Time Data Streams
              </div>
            </div>
            <p class="text-body1 text-grey-7 q-mb-md">
              Data Farmer streams deliver predictive insights directly to
              traders, enabling informed decision-making with live market
              intelligence.
            </p>
            <!-- <q-btn
              flat
              no-caps
              color="secondary"
              icon-right="arrow_forward"
              class="q-pa-none"
            >
              View streams
            </q-btn> -->
          </q-card>
        </div>

        <!-- Community-Driven Feature -->
        <div class="col-12 col-md-6">
          <q-card
            class="feature-card q-pa-lg"
            flat
            bordered
            @mouseenter="onHover(3)"
            @mouseleave="onLeave(3)"
            :class="{ 'feature-card-hover': hoveredCard === 3 }"
          >
            <div class="row items-center q-mb-md">
              <q-avatar
                size="48px"
                class="q-mr-md"
                style="background: linear-gradient(45deg, #f57c00, #ffb74d)"
              >
                <q-icon name="groups" color="white" size="24px" />
              </q-avatar>
              <div class="text-h5 text-weight-bold text-grey-9">
                Community-Driven
              </div>
            </div>
            <p class="text-body1 text-grey-7 q-mb-md">
              Harness the power of crowdsourced intelligence. Our community
              continuously improves model accuracy through collaborative
              contributions.
            </p>
            <!-- <q-btn
              flat
              no-caps
              color="orange"
              icon-right="arrow_forward"
              class="q-pa-none"
            >
              Join community
            </q-btn> -->
          </q-card>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from "vue";

const hoveredCard = ref<number | null>(null);

const onHover = (index: number) => {
  hoveredCard.value = index;
};

const onLeave = (index: number) => {
  hoveredCard.value = null;
};
</script>

<style scoped>
.header {
  margin-bottom: 88px;
}
.features {
  margin-bottom: 100px;
}
.feature-card {
  transition: all 0.3s ease;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  min-height: 230px;
}

.feature-card-hover {
  transform: translateY(-8px);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

.feature-card:hover {
  background: rgba(255, 255, 255, 0.95);
}
</style>
