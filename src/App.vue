<template>
  <div class="p-3">
    <section v-if="selectedOption">
      <a href="javascript:void(0)" @click="goToHome()">Home > {{ selectedOption?.title }}</a>
    </section>

    <Home v-if="!selectedOption" @onOptionChanges="(val) => selectedOption = val" />

    <div class="mt-4" v-if="selectedOption">
      <MirrorRepository v-if="selectedOption.id == 1" />
      <SyncTwoAzureDevopsOrgs v-if="selectedOption.id == 2" />
    </div>

  </div>
</template>

<script setup>
import { ref } from "vue";
import Home from './components/Home.vue';
import MirrorRepository from './components/MirrorRepository.vue';
import SyncTwoAzureDevopsOrgs from './components/SyncTwoAzureDevopsOrgs/SyncTwoAzureDevopsOrgs.vue';
import { useAzureDevopsStore } from "./stores/AzureDevopsStore";
const selectedOption = ref(null);

const azureDevopsStore = useAzureDevopsStore();

function goToHome() {
  if(azureDevopsStore.stillInrogress) return;
  selectedOption.value = null;
  azureDevopsStore.$reset();
}
</script>

<style scoped>

</style>
