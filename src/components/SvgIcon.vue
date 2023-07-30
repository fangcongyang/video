<template>
    <svg aria-hidden="true" :style="getStyle">
      <title>{{ title }}</title>
      <use :xlink:href="symbolId" :fill="color" />
    </svg>
</template>

<script>
import { defineComponent, computed } from 'vue'

export default defineComponent({
    name: 'SvgIcon',
    props: {
        prefix: {
            type: String,
            default: 'icon',
        },
        name: {
            type: String,
            required: true,
        },
        size: {
            type: [Number, String],
            default: 24,
        },
        color: {
            type: String,
        },
        title: {
            type: String,
        },
        styleVar: {
            type: Object,
            default: () => ({}),
        },
    },
    setup(props) {
        const symbolId = computed(() => `#${props.prefix}-${props.name}`)

        const getStyle = computed(() => {
            const { size, styleVar } = props;
            if (styleVar.width) {
                return styleVar;
            }
            let s = `${size}`;
            s = `${s.replace('px', '')}px`;
            return {
                width: s,
                height: s,
            };
        });
        return { symbolId, getStyle }
    },
})
</script>