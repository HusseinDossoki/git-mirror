<template>
    <StepOne v-if="azureToAzureStore.currentStep == 1" />
    <StepTwo v-if="azureToAzureStore.currentStep == 2" />
    <StepThree v-if="azureToAzureStore.currentStep == 3" />

    <div class="alert alert-danger mt-3" v-if="azureToAzureStore.errors.length > 0">
        <ul class="my-0">
            <li v-for="err in azureToAzureStore.errors">{{ err }}</li>
        </ul>
    </div>

    <div class="row mt-4">
        <div class="text-center col-12">
            <button class="btn btn-secondary mx-3"
                v-if="azureToAzureStore.currentStep == 2 || azureToAzureStore.currentStep == 3"
                @click="azureToAzureStore.prev()"
                :disabled="azureToAzureStore.stillInrogress">Previous</button>
            <button class="btn btn-primary mx-3"
                v-if="azureToAzureStore.currentStep == 1 || azureToAzureStore.currentStep == 2"
                @click="azureToAzureStore.next()" :disabled="!azureToAzureStore.isCurrentStepValid">next
                <span v-if="azureToAzureStore.currentStep == 2">
                    ({{ azureToAzureStore?.selectedProjectsCount }} projects)</span>
            </button>
            <button class="btn btn-success mx-3" v-if="azureToAzureStore.currentStep == 3"
                @click="azureToAzureStore.submit()"
                :disabled="azureToAzureStore.stillInrogress">Submit</button>
        </div>
    </div>
    <Spinner :msg="'loading...'" :show="azureToAzureStore.loading" />
</template>

<script setup>
import StepOne from './StepOne.vue';
import StepTwo from './StepTwo.vue';
import StepThree from './StepThree.vue';
import Spinner from '../Spinner.vue';
import { useAzureToAzureStore } from "../../stores/AzureToAzureStore";

const azureToAzureStore = useAzureToAzureStore();

</script>