<template>
  <section class="features-section">
    <!-- Section Header -->
    <div class="section-header">
      <h2 class="section-title">Core Features</h2>
      <p class="section-subtitle">
        Powerful capabilities designed for the future
      </p>
    </div>

    <!-- Features Grid -->
    <div class="features-grid">
      <q-card
        v-for="feature in features"
        :key="feature.id"
        flat
        bordered
        class="feature-card"
        :style="{ '--gradient': feature.gradient }"
      >
        <q-card-section class="card-content">
          <!-- Icon with gradient background -->
          <div class="icon-wrapper">
            <div class="icon-bg">
              <q-icon :name="feature.icon" size="2rem" class="feature-icon" />
            </div>
          </div>

          <!-- Title -->
          <h3 class="feature-title q-mt-md q-mb-sm">{{ feature.title }}</h3>

          <!-- Description -->
          <p class="feature-description q-ma-none">
            {{ feature.description }}
          </p>
        </q-card-section>
      </q-card>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface Feature {
  id: number;
  icon: string;
  title: string;
  description: string;
  gradient: string;
}

const features = ref<Feature[]>([
  {
    id: 0,
    icon: "memory",
    title: "On-Chain AI",
    description:
      "LSTM models run directly in Canisters using the ic-burn framework, enabling true chain-native inference without external dependencies.",
    gradient: "linear-gradient(135deg, #1976d2, #42a5f5)",
  },
  {
    id: 1,
    icon: "rocket_launch",
    title: "One-Click Deployment",
    description:
      "Deploy multiple Canisters effortlessly with pre-configured models or customize your own. Get up and running in minutes, not hours.",
    gradient: "linear-gradient(135deg, #388e3c, #66bb6a)",
  },
  {
    id: 2,
    icon: "timeline",
    title: "Real-Time Data Streams",
    description:
      "Data Farmer streams deliver predictive insights directly to traders, enabling informed decision-making with live market intelligence.",
    gradient: "linear-gradient(135deg, #7b1fa2, #ba68c8)",
  },
  {
    id: 3,
    icon: "groups",
    title: "Community-Driven",
    description:
      "Harness the power of crowdsourced intelligence. Our community continuously improves model accuracy through collaborative contributions.",
    gradient: "linear-gradient(135deg, #f57c00, #ffb74d)",
  },
]);
</script>

<style scoped lang="scss">
.features-section {
  position: relative;
  padding: 4rem 2rem;
  background: #f8f9fa; // Light neutral background instead of dark
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: auto;
}

.section-header {
  text-align: center;
  margin-bottom: 3rem;
  max-width: 600px;
}

.section-title {
  font-size: clamp(2rem, 5vw, 2.5rem);
  font-weight: 700;
  color: #1a1a1a;
  margin: 0 0 0.5rem 0;
  letter-spacing: -0.5px;
}

.section-subtitle {
  font-size: 1.1rem;
  color: #666666;
  margin: 0;
  line-height: 1.6;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
  width: 100%;
  max-width: 1200px;

  @media (max-width: 768px) {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  @media (min-width: 769px) and (max-width: 1024px) {
    grid-template-columns: repeat(2, 1fr);
  }

  @media (min-width: 1025px) {
    grid-template-columns: repeat(4, 1fr);
  }
}

.feature-card {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid #e8e8e8;
  background: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  height: 100%;

  &:hover {
    border-color: var(--gradient);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
    transform: translateY(-4px);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem;
}

.icon-wrapper {
  display: flex;
  justify-content: flex-start;
}

.icon-bg {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 56px;
  height: 56px;
  border-radius: 8px;
  background: var(--gradient);
  opacity: 0.15;
  transition: all 0.3s ease;
  position: relative;

  &::after {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: 8px;
    background: var(--gradient);
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .feature-card:hover & {
    opacity: 0.2;
  }
}

.feature-icon {
  color: #1a1a1a;
}

.feature-title {
  font-size: 1.15rem;
  font-weight: 600;
  color: #1a1a1a;
  line-height: 1.3;
}

.feature-description {
  font-size: 0.9rem;
  color: #666666;
  line-height: 1.6;
}
</style>
