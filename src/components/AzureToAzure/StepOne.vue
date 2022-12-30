<template>
    <h6 class="mt-4 mb-1">Source</h6>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">PAT</span>
        <input type="text" class="form-control" v-model.trim="azureToAzureStore.srcPat">
    </div>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">Organization Name</span>
        <input type="text" class="form-control" v-model.trim="azureToAzureStore.srcOrgName">
    </div>


    <h6 class="mt-4 mb-1">Destination</h6>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">PAT</span>
        <input type="text" class="form-control" v-model.trim="azureToAzureStore.destPat">
    </div>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">Organization Name</span>
        <input type="text" class="form-control" v-model.trim="azureToAzureStore.destOrgName">
    </div>

</template>

<script setup>
import { useAzureToAzureStore } from "../../stores/AzureToAzureStore";

const azureToAzureStore = useAzureToAzureStore();
</script>