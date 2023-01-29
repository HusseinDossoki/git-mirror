<template>
  <div class="p-3">
    <section v-if="selectedOption">
      <a href="javascript:void(0)" @click="goToHome()">Home > {{ selectedOption?.title }}</a>
    </section>

    <Home v-if="!selectedOption" @onOptionChanges="(val) => selectedOption = val" />

    <div class="mt-4" v-if="selectedOption">
      <MirrorRepository v-if="selectedOption.id == 1" />
      <AzureToAzure v-if="selectedOption.id == 2" />
      <GithubToAzure v-if="selectedOption.id == 3" />
    </div>
  </div>
  <AppFooter />
</template>

<script setup>
import AppFooter from './components/AppFooter.vue';
import { ref } from "vue";
import Home from './components/Home.vue';
import MirrorRepository from './components/MirrorRepository/MirrorRepository.vue';
import AzureToAzure from './components/AzureToAzure/AzureToAzure.vue';
import GithubToAzure from './components/GithubToAzure/GithubToAzure.vue';
import { useAzureToAzureStore } from "./stores/AzureToAzureStore";
const selectedOption = ref(null);

const azureToAzureStore = useAzureToAzureStore();

function goToHome() {
  if(azureToAzureStore.stillInrogress) return;
  selectedOption.value = null;
  azureToAzureStore.$reset();
}
</script>

<style scoped>

</style>
