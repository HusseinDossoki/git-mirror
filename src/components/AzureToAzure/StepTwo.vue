<template>

	<div class="relative mb-6">
		<div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
			<svg aria-hidden="true" class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor"
				viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
					d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
			</svg>
		</div>
		<input type="text" id="input-group-1"
			class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
			placeholder="Filter by Name" v-model.trim="azureToAzureStore.filter">
	</div>

	<div class="flex items-center mb-4">
		<input id="projects-checkbox" @change="onCheckChnages" type="checkbox" value=""
			class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
		<label for="projects-checkbox" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">Select
			All Projects</label>
	</div>

	<ul class="grid w-full gap-2 md:grid-cols-3">
		<li v-for="project in azureToAzureStore.filtered">
			<input type="checkbox" :id="project.id" @change="onProjectClicked(project)" :checked="project.selected" value=""
				class="hidden peer">
			<label :for="project.id"
				class="inline-flex items-center justify-between w-full p-4 text-gray-500 bg-white border-2 border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 peer-checked:border-blue-600 hover:text-gray-600 dark:peer-checked:text-gray-300 peer-checked:text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
				<div class="block">
					<!-- <img src="" class="mb-2 w-7 h-7 text-sky-500" alt=""> -->
					<div class="w-full text-lg font-semibold">{{ project.name }}</div>
					<div class="w-full text-sm">{{ project.description }}</div>
				</div>
			</label>
		</li>
	</ul>

</template>

<script setup>
import { useAzureToAzureStore } from "../../stores/AzureToAzureStore";

const azureToAzureStore = useAzureToAzureStore();

function onCheckChnages(e) {
	e.target.checked ? azureToAzureStore.checkAll() : azureToAzureStore.uncheckAll();
}
function onProjectClicked(project) {
	project.selected = !project.selected;
}
</script>

<style scoped>
.card-container {
	max-height: 72vh;
	overflow-y: scroll;
}

.selected {
	background-color: cornflowerblue !important;
	/* color: white; */
}
</style>