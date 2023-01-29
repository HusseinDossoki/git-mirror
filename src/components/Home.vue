<template>
	<section class="logo">
		<h1 class="mt-3 pb-0 mb-0">Git Mirror!</h1>
		<small>By <strong>Hussein Dossoki</strong></small>
	</section>

	<div class="card my-3" v-for="opt in options" @click="$emit('onOptionChanges', opt)">
		<div class="row no-gutters">
			<div class="col-4 m-auto">
				<img :src="opt.image" class="card-img" alt="image">
			</div>
			<div class="col-8">
				<div class="card-body">
					<h5 class="card-title">{{ opt.title }}</h5>
					<p class="card-text">{{ opt.description }}</p>
				</div>
			</div>
		</div>
	</div>

</template>

<script setup>
import { ref } from "vue";

const options = ref([
	{
		id: 1,
		image: new URL('../assets/git.png', import.meta.url).href,
		title: "Mirror Git Repository",
		description: "Mirroring any single repository in another location Regardless of the type of the version controle (github, gitlab, bitbucket, azure devops, ...etc)"
	},
	{
		id: 2,
		image: new URL('../assets/azure.png', import.meta.url).href,
		title: "Sync Azure Devops Organizations",
		description: "Clone/Copy specific AzureDevOps project repositories to another AzureDevOps project"
	},
	{
		id: 3,
		image: new URL('../assets/github-to-azure.jpg', import.meta.url).href,
		title: "Github to Azure Devops",
		description: "Clone/Copy specific GitHub project repositories to AzureDevOps project"
	}
]);
</script>

<style scoped>
.logo {
	text-align: center;
	margin-bottom: 40px;
}

.logo h1 {
	letter-spacing: 5px;
	font-family: Arial;
	font-weight: bold;
}

.logo small strong {
	color: burlywood;
}

.card:hover {
	cursor: pointer;
	box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px;
}

.card img {
	height: 100px;
	object-fit: contain;
	padding-left: 15px;
}
</style>