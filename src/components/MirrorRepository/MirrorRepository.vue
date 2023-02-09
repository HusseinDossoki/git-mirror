<template>

  <h6 class="text-lg font-bold dark:text-white">Source</h6>


  <div class="flex">
    <span
      class="inline-flex items-center px-3 text-sm text-gray-900 bg-gray-200 border border-r-0 border-gray-300 rounded-l-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
      Personal
      Access Token
    </span>
    <input type="text" id="website-admin"
      class="rounded-none rounded-r-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      placeholder="" v-model.trim="srcPat">
  </div>

  <div class="flex mt-2">
    <span
      class="inline-flex items-center px-3 text-sm text-gray-900 bg-gray-200 border border-r-0 border-gray-300 rounded-l-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
      Git
      URL
    </span>
    <input type="text" id="website-admin"
      class="rounded-none rounded-r-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      placeholder="" v-model.trim="srcUrl">
  </div>


  <h6 class="text-lg font-bold dark:text-white mt-4">Destination</h6>

  <div class="flex">
    <span
      class="inline-flex items-center px-3 text-sm text-gray-900 bg-gray-200 border border-r-0 border-gray-300 rounded-l-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
      Personal
      Access Token
    </span>
    <input type="text" id="website-admin"
      class="rounded-none rounded-r-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      placeholder="" v-model.trim="destPat">
  </div>

  <div class="flex mt-2">
    <span
      class="inline-flex items-center px-3 text-sm text-gray-900 bg-gray-200 border border-r-0 border-gray-300 rounded-l-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
      Git
      URL
    </span>
    <input type="text" id="website-admin"
      class="rounded-none rounded-r-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      placeholder="" v-model.trim="destUrl">
  </div>

  <div v-if="submitted && error"
    class="mt-4 flex p-4 mb-4 text-sm text-red-800 border border-red-300 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400 dark:border-red-800"
    role="alert">
    <svg aria-hidden="true" class="flex-shrink-0 inline w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd"
        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
        clip-rule="evenodd"></path>
    </svg>
    <div>
      {{ error }}
    </div>
  </div>

  <div v-if="submitted && !error"
    class="mt-4 flex p-4 mb-4 text-sm text-green-800 border border-green-300 rounded-lg bg-green-50 dark:bg-gray-800 dark:text-green-400 dark:border-green-800"
    role="alert">
    <svg aria-hidden="true" class="flex-shrink-0 inline w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd"
        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
        clip-rule="evenodd"></path>
    </svg>
    <div>
      Repository is cloned successfully
    </div>
  </div>

  <button type="button" @click="submit()" :disabled="!isValidForm || submitting"
    class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center mr-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
    <svg aria-hidden="true" class="w-5 h-5 mr-2 -ml-1" fill="currentColor" viewBox="0 0 20 20"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M3 1a1 1 0 000 2h1.22l.305 1.222a.997.997 0 00.01.042l1.358 5.43-.893.892C3.74 11.846 4.632 14 6.414 14H15a1 1 0 000-2H6.414l1-1H14a1 1 0 00.894-.553l3-6A1 1 0 0017 3H6.28l-.31-1.243A1 1 0 005 1H3zM16 16.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM6.5 18a1.5 1.5 0 100-3 1.5 1.5 0 000 3z">
      </path>
    </svg>
    {{ submitting? 'Cloning...': 'Clone' }}
  </button>

  <Spinner :msg="'Cloning the Repository'" :show="submitting" />

</template>

<script setup>
import Spinner from "../Spinner.vue";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const error = ref(null);
const submitting = ref(false);
const submitted = ref(false);

const srcPat = ref(null);
const srcUrl = ref(null);
const destPat = ref(null);
const destUrl = ref(null);

const isValidForm = computed(() => {
  return srcUrl.value && destUrl.value;
})

function submit() {
  submitted.value = false;
  submitting.value = true;
  invoke("mirror_repository",
    {
      srcPat: srcPat.value,
      srcRepoUrl: srcUrl.value,
      destPat: destPat.value,
      destRepoUrl: destUrl.value,
    })
    .then(res => {
      submitting.value = false;
      submitted.value = true;
      error.value = null;
    })
    .catch(err => {
      error.value = err;
      submitting.value = false;
      submitted.value = true;
    });
}
</script>

<style scoped>

</style>