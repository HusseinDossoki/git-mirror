<template>

  <h6 class="mt-4 mb-1">Source</h6>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">PAT</span>
    <input type="text" class="form-control" v-model.trim="srcPat">
  </div>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">Repo Git URL</span>
    <input type="text" class="form-control" v-model.trim="srcUrl">
  </div>


  <h6 class="mt-4 mb-1">Destination</h6>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">PAT</span>
    <input type="text" class="form-control" v-model.trim="destPat">
  </div>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">Repo Git URL</span>
    <input type="text" class="form-control" v-model.trim="destUrl">
  </div>


  <div class="alert alert-danger mt-3" v-if="submitted && error">
    {{ error }}
  </div>

  <div class="alert alert-success mt-3" v-if="submitted && !error">
    Repository is cloned successfully ✅
  </div>

  <div class="d-grid gap-2 col-6 mx-auto mt-3">
    <button class="btn btn-primary" type="button" @click="submit()" :disabled="!isValidForm || submitting">
      <span v-if="submitting" class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
      {{ submitting? 'Cloning...': 'Clone' }}
    </button>
  </div>

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