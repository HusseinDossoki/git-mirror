<template>

    <div class="input-group input-group-sm flex-nowrap mt-2">
        <span class="input-group-text">Filter by Name</span>
        <input type="text" class="form-control" v-model.trim="azureDevopsStore.filter">
    </div>

    <div class="form-check my-2">
        <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" @change="onCheckChnages">
        <label class="form-check-label" for="flexCheckDefault">
            Select All Projects
        </label>
    </div>

    <div class="row card-container">
        <div class="col-12 mt-2" v-for="project in azureDevopsStore.filtered">
            <div class="card" @click="onProjectClicked(project)" :class="{ selected: project.selected }">
                <!-- <img class="card-img-top" src="..." alt="Card image cap"> -->
                <div class="card-body">
                    <h5 class="card-title">{{ project.name }}</h5>
                    <p class="card-text">{{ project.description }}</p>
                </div>
            </div>
        </div>
    </div>

</template>

<script setup>
import { useAzureDevopsStore } from "../../stores/AzureDevopsStore";

const azureDevopsStore = useAzureDevopsStore();

function onCheckChnages(e) {
    e.target.checked ? azureDevopsStore.checkAll() : azureDevopsStore.uncheckAll();
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

.card:hover {
    cursor: pointer;
    box-shadow: rgba(149, 157, 165, 0.2) 0px 8px 24px;
}

.card.selected {
    background-color: cornflowerblue;
    color: white;
}
</style>