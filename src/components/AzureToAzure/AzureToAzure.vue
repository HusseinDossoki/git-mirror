<template>

	<ol
		class="mb-5 mt-5 flex items-center w-full text-sm font-medium text-center text-gray-500 dark:text-gray-400 sm:text-base">
		<li :class="{ 'text-blue-600 dark:text-blue-500 active': azureToAzureStore.currentStep == 1 }"
			class="flex md:w-full items-center  sm:after:content-[''] after:w-full after:h-1 after:border-b after:border-gray-200 after:border-1 after:hidden sm:after:inline-block after:mx-6 xl:after:mx-10 dark:after:border-gray-700">
			<span
				class="flex items-center after:content-['/'] sm:after:hidden after:mx-2 after:font-light after:text-gray-200 dark:after:text-gray-500">
				<span
					class="title flex items-center after:content-['/'] sm:after:hidden after:mx-2 after:font-light after:text-gray-200 dark:after:text-gray-500">
					<span class="mr-2">(1)</span>
					<span style="width: max-content;">Git Credential</span>
				</span>
			</span>
		</li>
		<li :class="{ 'text-blue-600 dark:text-blue-500 active': azureToAzureStore.currentStep == 2 }"
			class="flex md:w-full items-center after:content-[''] after:w-full after:h-1 after:border-b after:border-gray-200 after:border-1 after:hidden sm:after:inline-block after:mx-6 xl:after:mx-10 dark:after:border-gray-700">
			<span
				class="title flex items-center after:content-['/'] sm:after:hidden after:mx-2 after:font-light after:text-gray-200 dark:after:text-gray-500">
				<span class="mr-2">(2)</span>
				Projects
			</span>
		</li>
		<li class="flex items-center"
			:class="{ 'text-blue-600 dark:text-blue-500 active': azureToAzureStore.currentStep == 3 }">
			<span class="title">
				<span class="mr-2">(3)</span>
				Submit
			</span>
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
			<font-awesome-icon icon="fa-solid fa-chevron-left" class="mr-2" />
			Previous
		</button>

		<button type="button" v-if="azureToAzureStore.currentStep == 3" @click="azureToAzureStore.submit()"
			:disabled="azureToAzureStore.stillInrogress"
			:class="{ 'bg-blue-400 dark:bg-blue-500 cursor-not-allowed': !azureToAzureStore.isCurrentStepValid, 'hover:bg-blue-800': azureToAzureStore.isCurrentStepValid }"
			class="mx-3 text-white bg-blue-700  focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
			Submit
			<font-awesome-icon icon="fa-solid fa-rocket" class="ml-2" />
		</button>

		<button type="button" v-if="azureToAzureStore.currentStep == 1 || azureToAzureStore.currentStep == 2"
			@click="azureToAzureStore.next()" :disabled="!azureToAzureStore.isCurrentStepValid"
			:class="{ 'bg-blue-400 dark:bg-blue-500 cursor-not-allowed': !azureToAzureStore.isCurrentStepValid, 'hover:bg-blue-800': azureToAzureStore.isCurrentStepValid }"
			class="mx-3 text-white bg-blue-700  focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
			Next <span v-if="azureToAzureStore.currentStep == 2">
				({{ azureToAzureStore?.selectedProjectsCount }} projects)</span>
			<font-awesome-icon icon="fa-solid fa-chevron-right" class="ml-2" />
		</button>
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

ol li.active .title {
	background-color: rgba(110, 95, 95, 0.15);
	padding: 15px;
	border-radius: 1px;
}
</style>