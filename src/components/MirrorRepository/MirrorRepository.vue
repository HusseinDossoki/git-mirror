<template>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">Source Git URL</span>
    <input type="text" class="form-control" v-model.trim="srcUrl">
  </div>

  <div class="input-group input-group-sm flex-nowrap mt-2">
    <span class="input-group-text">Destiniation Git URL</span>
    <input type="text" class="form-control" v-model.trim="destUrl">
  </div>

  <div class="alert alert-info mt-3" role="alert">
    <h4 class="alert-heading">Note!</h4>
    <p>If your repo is private then you have to generate new PAT (Personal Access Token) and append it to the Git URL
    </p>
    <hr>
    <p class="mb-0">
      https://<strong>{PAT}</strong>@github.com/{your account or organization}/{repo}.git
    </p>
  </div>


  <div class="alert alert-success mt-3" v-if="submitted">
    Repository is cloned successfully ✅
  </div>

  <div class="d-grid gap-2 col-6 mx-auto mt-3">
    <button class="btn btn-primary" type="button" @click="submit()" :disabled="!isValidForm || submitting">
      <span v-if="submitting" class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
      {{ submitting ? 'Cloning...' : 'Clone' }}
    </button>
  </div>

  <Spinner :msg="'Cloning the Repository'" :show="submitting" />

</template>

<script setup>
import Spinner from "../Spinner.vue";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const result = ref(null);
const submitting = ref(false);
const submitted = ref(false);

const srcUrl = ref(null);
const destUrl = ref(null);

const isValidForm = computed(() => {
  return srcUrl.value && destUrl.value;
})

async function submit() {
  submitted.value = false;
  submitting.value = true;
  result.value = await invoke("mirror_repository",
    {
      srcRepoUrl: srcUrl.value,
      destRepoUrl: destUrl.value,
    });
  submitting.value = false;
  submitted.value = true;
}
</script>

<style scoped>

</style>