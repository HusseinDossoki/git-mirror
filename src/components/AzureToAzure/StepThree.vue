<template>
	<div>
		<h3 class="mb-2 text-lg font-semibold text-gray-900 dark:text-white">We are going to sync <strong>{{
			azureToAzureStore.selectedProjectsCount
		}}</strong> projects</h3>

		<div class="flex items-center mb-4">
			<input id="projects-checkbox" :checked="azureToAzureStore.createProjectIfNotExist" @change="onCheckChnages"
				type="checkbox" value="" :disabled="azureToAzureStore.stillInrogress"
				class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
			<label for="projects-checkbox" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">Create the
				Project if Not Exist</label>
		</div>

		<a v-for="project in azureToAzureStore.selectedProjects" href="javascript:void(9)"
			class="mt-2 block p-4 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
			<div class="flex">
				<h5 class="mb-2 text-1xl font-bold tracking-tight text-gray-900 dark:text-white">{{ project.name }}</h5>

				<div class="ml-auto flex">
					<div role="status" v-if="project.status == 'inprogress'">
						<svg aria-hidden="true"
							class="inline w-6 h-6 mr-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
							viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path
								d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
								fill="currentColor" />
							<path
								d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
								fill="currentFill" />
						</svg>
					</div>
					<div v-if="project.status == 'completed'">✅</div>
					<div v-if="project.status == 'failed'">🚫</div>
					<small class="dark:text-white" style="line-height: 30px;">{{ formatTime(project.timerCount) }}</small>
				</div>
			</div>
			<p class="font-normal text-gray-700 dark:text-gray-400">{{ project.description }}</p>
			<small style="color: red;" v-if="project.error">{{project.error}}</small>
		</a>

	</div>
</template>

<script setup>
import { useAzureToAzureStore } from "../../stores/AzureToAzureStore";
import { computed } from "vue";

const azureToAzureStore = useAzureToAzureStore();
const formatTime = computed(() => {
	return time => {
		if (!time) return;
		let minutes = Math.floor(time / 60);
		let seconds = time - minutes * 60;
		return minutes > 0 ? `${minutes}m ${seconds}s` : `${seconds}s`;
	};
});
function onCheckChnages(e) {
	azureToAzureStore.createProjectIfNotExist = e.target.checked;
}
</script>

<style scoped>

</style>