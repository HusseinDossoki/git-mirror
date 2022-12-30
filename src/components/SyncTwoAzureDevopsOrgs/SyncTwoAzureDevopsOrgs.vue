<template>
    <StepOne v-if="azureDevopsStore.currentStep == 1" />
    <StepTwo v-if="azureDevopsStore.currentStep == 2" />
    <StepThree v-if="azureDevopsStore.currentStep == 3" />

    <div class="alert alert-danger mt-3" v-if="azureDevopsStore.errors.length > 0">
        <ul class="my-0">
            <li v-for="err in azureDevopsStore.errors">{{ err }}</li>
        </ul>
    </div>

    <div class="row mt-4">
        <div class="text-center col-12">
            <button class="btn btn-secondary mx-3"
                v-if="azureDevopsStore.currentStep == 2 || azureDevopsStore.currentStep == 3"
                @click="azureDevopsStore.prev()"
                :disabled="azureDevopsStore.stillInrogress">Previous</button>
            <button class="btn btn-primary mx-3"
                v-if="azureDevopsStore.currentStep == 1 || azureDevopsStore.currentStep == 2"
                @click="azureDevopsStore.next()" :disabled="!azureDevopsStore.isCurrentStepValid">next
                <span v-if="azureDevopsStore.currentStep == 2">
                    ({{ azureDevopsStore?.selectedProjectsCount }} projects)</span>
            </button>
            <button class="btn btn-success mx-3" v-if="azureDevopsStore.currentStep == 3"
                @click="azureDevopsStore.submit()"
                :disabled="azureDevopsStore.stillInrogress">Submit</button>
        </div>
    </div>
    <Spinner :msg="'loading...'" :show="azureDevopsStore.loading" />
</template>

<script setup>
import StepOne from './StepOne.vue';
import StepTwo from './StepTwo.vue';
import StepThree from './StepThree.vue';
import Spinner from '../Spinner.vue';
import { useAzureDevopsStore } from "../../stores/AzureDevopsStore";

const azureDevopsStore = useAzureDevopsStore();

</script>