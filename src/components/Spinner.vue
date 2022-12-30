<script setup>
const props = defineProps({
  msg: String,
  show: Boolean
});

</script>

<template>
    <section class="spinner-container" v-if="show">
        <div class="spinner-content">
            <div class="spinner-grow text-warning" role="status">
            </div>
            <h6 class="text-muted">{{ msg || 'Submitting...'}}</h6>
            <small>
                <a @click="$emit('cancel')" href="javascript:void(0)">cancel</a>
            </small>
        </div>
    </section>
</template>

<style scoped>
.spinner-container {
    position: absolute;
    background-color: rgba(221, 221, 221, 0.781);
    width: 100%;
    height: 100%;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    text-align: center;
}

.spinner-content {
    display: block;
    margin: auto;
    margin-top: 40%;
}
</style>
