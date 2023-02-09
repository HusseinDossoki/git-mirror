<template>
  <div class="p-3 body-container">

    <!-- Breadcrumb -->
    <nav
      v-if="selectedOption"
      class="flex px-5 py-3 text-gray-700 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-800 dark:border-gray-700"
      aria-label="Breadcrumb">
      <ol class="inline-flex items-center space-x-1 md:space-x-3">
        <li class="inline-flex items-center">
          <a href="javascript:void(0)" @click="goToHome()"
            class="inline-flex items-center text-sm font-medium text-gray-700 hover:text-blue-600 dark:text-gray-400 dark:hover:text-white">
            <svg aria-hidden="true" class="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg">
              <path
                d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z">
              </path>
            </svg>
            Home
          </a>
        </li>
        <li aria-current="page">
          <div class="flex items-center">
            <svg aria-hidden="true" class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg">
              <path fill-rule="evenodd"
                d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                clip-rule="evenodd"></path>
            </svg>
            <span class="ml-1 text-sm font-medium text-gray-500 md:ml-2 dark:text-gray-400">{{
              selectedOption?.title
            }}</span>
          </div>
        </li>
      </ol>
    </nav>

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
  if (azureToAzureStore.stillInrogress) return;
  selectedOption.value = null;
  azureToAzureStore.$reset();
}
</script>

<style scoped>
.body-container {
  margin-bottom: 70px !important;
}
.breadcrumb-item a {
  text-decoration: none;
}
</style>
