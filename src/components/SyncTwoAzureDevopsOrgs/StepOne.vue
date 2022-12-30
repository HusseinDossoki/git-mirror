<template>
    <h6 class="mt-4 mb-1">Source</h6>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">PAT</span>
        <input type="text" class="form-control" v-model.trim="azureDevopsStore.srcPat">
    </div>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">Organization Name</span>
        <input type="text" class="form-control" v-model.trim="azureDevopsStore.srcOrgName">
    </div>


    <h6 class="mt-4 mb-1">Destination</h6>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">PAT</span>
        <input type="text" class="form-control" v-model.trim="azureDevopsStore.destPat">
    </div>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">Organization Name</span>
        <input type="text" class="form-control" v-model.trim="azureDevopsStore.destOrgName">
    </div>

</template>

<script setup>
import { useAzureDevopsStore } from "../../stores/AzureDevopsStore";

const azureDevopsStore = useAzureDevopsStore();
</script>