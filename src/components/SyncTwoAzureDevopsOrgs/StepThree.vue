<template>
	<div>
		We are going to sync <strong>{{ azureDevopsStore.selectedProjectsCount }}</strong> projects:-

		<div class="form-check my-2">
			<input class="form-check-input" type="checkbox" :checked="azureDevopsStore.createProjectIfNotExist" value=""
				id="flexCheckDefault" @change="onCheckChnages" :disabled="azureDevopsStore.stillInrogress">
			<label class="form-check-label" for="flexCheckDefault">
				Create the Project If Not Exist
			</label>
		</div>

		<div class="list-group mt-2">
			<a href="javascript:void(0)" v-for="project in azureDevopsStore.selectedProjects"
				class="list-group-item list-group-item-action flex-column align-items-start">
				<div class="d-flex w-100 justify-content-between">
					<h5 class="mb-1">{{ project.name }}</h5>
					<div>
						<small class="text-muted">{{ formatTime(project.timerCount) }}</small>
						<div v-if="project.status == 'inprogress'" class="spinner-border spinner-border-sm text-primary"
							role="status"></div>
						<div v-if="project.status == 'completed'" class="status">✅</div>
						<div v-if="project.status == 'failed'" class="status">🚫</div>
					</div>
				</div>
				<p class="mb-1">{{ project.description }}</p>
				<small v-if="project.status == 'failed'" class="text-danger">{{ project.error }}</small>
			</a>
		</div>
	</div>
</template>

<script setup>
import { useAzureDevopsStore } from "../../stores/AzureDevopsStore";
import { computed } from "vue";

const azureDevopsStore = useAzureDevopsStore();
const formatTime = computed(() => {
	return time => {
		if (!time) return;
		let minutes = Math.floor(time / 60);
		let seconds = time - minutes * 60;
		return minutes > 0 ? `${minutes}m ${seconds}s` : `${seconds}s`;
	};
});
function onCheckChnages(e) {
	azureDevopsStore.createProjectIfNotExist = e.target.checked;
}
</script>

<style scoped>
.list-group {
	max-height: 70vh;
	overflow-y: scroll;
}

.list-group-item .spinner-border {
	margin-left: 5px;
}

.list-group-item .status {
	display: inline-block;
	margin-left: 5px;
}
</style>