import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

export const useAzureDevopsStore = defineStore("azureDevopsStore", {
	state: () => ({
		srcPat: null,
		srcOrgName: null,
		destPat: null,
		destOrgName: null,
		loading: false,
		filter: null,
		currentStep: 1,
		srcProjects: [],
		destProjects: [],
		errors: []
	}),
	getters: {
		isCurrentStepValid(state) {
			switch (state.currentStep) {
				case 1:
					return state.srcPat && state.srcOrgName && state.destPat && state.destOrgName;
				case 2:
					return state.srcProjects.filter(x => x.selected).length > 0;
				case 3:
					return true;
			}
		},
		selectedProjects(state) {
			return state.srcProjects.filter(x => x.selected);
		},
		selectedProjectsCount(state) {
			return state.srcProjects.filter(x => x.selected).length;
		},
		filtered(state) {
			if (!state.filter) return state.srcProjects;
			return state.srcProjects.filter(x => x.name.trim().toLowerCase().includes(state.filter.trim().toLowerCase()));
		},
		stillInrogress(state) {
			return state.srcProjects?.filter(x => x.selected && x.status == "inprogress").length > 0;
		},
	},
	actions: {
		async next() {
			this.errors = [];
			if (this.currentStep == 3) return;

			if (this.currentStep == 1) {
				this.loading = true;

				let getSrcProjects = invoke("get_azure_devops_projects",
					{
						params: {
							pat: this.srcPat,
							organization_name: this.srcOrgName,
						}
					});

				let getDestProjects = invoke("get_azure_devops_projects",
					{
						params: {
							pat: this.destPat,
							organization_name: this.destOrgName,
						}
					});

				Promise.all([getSrcProjects, getDestProjects])
					.then((res) => {
						this.srcProjects = res[0];
						this.currentStep++;
						this.loading = false;
					})
					.catch((err) => {
						function onlyUnique(value, index, self) {
							return self.indexOf(value) === index;
						}
						this.errors.push(err);
						this.errors = this.errors.filter(x => onlyUnique);
						this.loading = false;
					});
			} else {
				this.currentStep++;
			}
		},
		prev() {
			if (this.currentStep == 1) return;
			this.currentStep--;
		},
		async submit() {

			this.selectedProjects.map(x => x.name).forEach(productName => {
				let product = this.srcProjects.find(x => x.name == productName);
				product.timerCount = 0;
				product.timer = setInterval(() => {
					product.timerCount++;
				}, 1000);
				product.status = "inprogress";

				invoke("sync_azure_devops_project",
					{
						params: {
							src_project_ref: {
								pat: this.srcPat,
								organization_name: this.srcOrgName,
								project_name: productName
							},
							dest_project_ref: {
								pat: this.destPat,
								organization_name: this.destOrgName,
								project_name: productName
							},
							create_project_if_not_exist: true
						}
					})
					.then((res) => {
						clearInterval(product.timer);
						product.status = "completed";
					})
					.catch((err) => {
						clearInterval(product.timer);
						product.status = "failed";
						product.error = err;
					});

			});


		},
		checkAll() {
			this.srcProjects.forEach(x => {
				x.selected = true;
			});
		},
		uncheckAll() {
			this.srcProjects.forEach(x => {
				x.selected = false;
			});
		}
	}
});