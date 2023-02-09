<template>


	<ol
		class="mb-5 mt-5 flex items-center w-full text-sm font-medium text-center text-gray-500 dark:text-gray-400 sm:text-base">
		<li
			class="flex md:w-full items-center text-blue-600 dark:text-blue-500 sm:after:content-[''] after:w-full after:h-1 after:border-b after:border-gray-200 after:border-1 after:hidden sm:after:inline-block after:mx-6 xl:after:mx-10 dark:after:border-gray-700">
			<span
				class="flex items-center after:content-['/'] sm:after:hidden after:mx-2 after:font-light after:text-gray-200 dark:after:text-gray-500">
				<svg aria-hidden="true" class="w-4 h-4 mr-2 sm:w-5 sm:h-5" fill="currentColor" viewBox="0 0 20 20"
					xmlns="http://www.w3.org/2000/svg">
					<path fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
						clip-rule="evenodd"></path>
				</svg>
				<span style="width: max-content;">Git Credential</span>
			</span>
		</li>
		<li
			class="flex md:w-full items-center after:content-[''] after:w-full after:h-1 after:border-b after:border-gray-200 after:border-1 after:hidden sm:after:inline-block after:mx-6 xl:after:mx-10 dark:after:border-gray-700">
			<span
				class="flex items-center after:content-['/'] sm:after:hidden after:mx-2 after:font-light after:text-gray-200 dark:after:text-gray-500">
				<span class="mr-2">2</span>
				Projects
			</span>
		</li>
		<li class="flex items-center">
			<span class="mr-2">3</span>
			Submit
		</li>
	</ol>


	<StepOne v-if="azureToAzureStore.currentStep == 1" />
	<StepTwo v-if="azureToAzureStore.currentStep == 2" />
	<StepThree v-if="azureToAzureStore.currentStep == 3" />

	<div class="alert alert-danger mt-3" v-if="azureToAzureStore.errors.length > 0">
		<ul class="my-0">
			<li v-for="err in azureToAzureStore.errors">{{ err }}</li>
		</ul>
	</div>

	<div class="text-center mt-3">
		<button type="button" v-if="azureToAzureStore.currentStep == 2 || azureToAzureStore.currentStep == 3"
			@click="azureToAzureStore.prev()" :disabled="azureToAzureStore.stillInrogress"
			:class="{ 'bg-blue-400 dark:bg-blue-500 cursor-not-allowed': azureToAzureStore.stillInrogress, 'hover:bg-blue-800': !azureToAzureStore.stillInrogress }"
			class="mx-3 text-white bg-blue-700  focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
			Previous
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--! Font Awesome Pro 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l128 128c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.3 288 480 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-370.7 0 73.4-73.4c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-128 128z"/></svg>
		</button>

		<button type="button" v-if="azureToAzureStore.currentStep == 1 || azureToAzureStore.currentStep == 2"
			@click="azureToAzureStore.next()" :disabled="!azureToAzureStore.isCurrentStepValid"
			:class="{ 'bg-blue-400 dark:bg-blue-500 cursor-not-allowed': !azureToAzureStore.isCurrentStepValid, 'hover:bg-blue-800': azureToAzureStore.isCurrentStepValid }"
			class="mx-3 text-white bg-blue-700  focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
			Next <span v-if="azureToAzureStore.currentStep == 2">
				({{ azureToAzureStore?.selectedProjectsCount }} projects)</span>
			<svg aria-hidden="true" class="w-5 h-5 ml-2 -mr-1" fill="currentColor" viewBox="0 0 20 20"
				xmlns="http://www.w3.org/2000/svg">
				<path fill-rule="evenodd"
					d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
					clip-rule="evenodd"></path>
			</svg>
		</button>
	</div>

	<div class="row mt-4">
		<div class="text-center col-12">

			<button class="btn btn-secondary mx-3"
				v-if="azureToAzureStore.currentStep == 2 || azureToAzureStore.currentStep == 3"
				@click="azureToAzureStore.prev()" :disabled="azureToAzureStore.stillInrogress">Previous</button>
			<button class="btn btn-primary mx-3"
				v-if="azureToAzureStore.currentStep == 1 || azureToAzureStore.currentStep == 2"
				@click="azureToAzureStore.next()" :disabled="!azureToAzureStore.isCurrentStepValid">next
				<span v-if="azureToAzureStore.currentStep == 2">
					({{ azureToAzureStore?.selectedProjectsCount }} projects)</span>
			</button>
			<button class="btn btn-success mx-3" v-if="azureToAzureStore.currentStep == 3" @click="azureToAzureStore.submit()"
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

<style scoped>
ol li {
	width: 33.3% !important;
}
</style>