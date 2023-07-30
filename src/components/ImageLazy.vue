<template>
    <img v-lazy="url" @load="imgLoad">
</template>

<script setup>
    import { inject } from 'vue'
    
    const imgLoaded = inject("imgLoaded");

    const props = defineProps({
        url: {
            type: String,
            required: true
        }
    });

    const imgLoad = () => {
        imgLoaded();
    }
</script>