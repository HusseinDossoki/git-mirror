<template>

	<div class="text-center my-8">
		<h1 class="mb-4 text-3xl font-extrabold text-gray-900 dark:text-white md:text-5xl lg:text-6xl"><span
				class="text-transparent bg-clip-text bg-gradient-to-r to-emerald-600 from-sky-400">Git Mirror!</span></h1>
		<p class="text-lg font-normal text-gray-500 lg:text-xl dark:text-gray-400">Git Mirror is a tool to keep multiple git
			repositories in sync. Whenever something is pushed to any repository, You can use this tool to forward the commits
			to all the others. Or you can use it to copy (Mirroring a repository in another location)</p>
	</div>


	<a href="javascript:void(0)"
		class="mt-2 flex items-center bg-white border border-gray-200 rounded-lg shadow md:flex-row hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700"
		v-for="opt in options" @click="$emit('onOptionChanges', opt)">
		<img class="object-cover rounded-t-lg md:rounded-none md:rounded-l-lg card-img p-2" :src="opt.image" alt="">
		<div class="flex flex-col justify-between p-4 leading-normal">
			<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{{ opt.title }}</h5>
			<p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{{ opt.description }}</p>
		</div>
	</a>

</template>

<script setup>
import { ref } from "vue";

const options = ref([
	{
		id: 1,
		image: new URL('../assets/git.png', import.meta.url).href,
		title: "Mirror Git Repository",
		description: "Mirroring any single repository in another location Regardless of the type of the version controle (github, gitlab, azure devops, ...etc)"
	},
	{
		id: 2,
		image: new URL('../assets/azure.png', import.meta.url).href,
		title: "Sync Azure Devops Organizations",
		description: "Clone/Copy specific AzureDevOps project repositories to another AzureDevOps project"
	},
	{
		id: 3,
		image: new URL('../assets/github-to-azure.png', import.meta.url).href,
		title: "Github to Azure Devops",
		description: "Clone/Copy specific GitHub project repositories to AzureDevOps project"
	}
]);
</script>

<style scoped>
img.card-img {
	width: 140px !important;
	height: 120px !important;
	object-fit: contain !important;
}
</style>