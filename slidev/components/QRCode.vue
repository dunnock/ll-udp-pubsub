<script>
import QrcodeVue from 'qrcode.vue'
import { ref } from 'vue'

export default {
    setup() {
        const href = ref("")
        return { href }
    },
    components: {
        QrcodeVue
    }
}
</script>


<template>
    <p><center><qrcode-vue :value="href"></qrcode-vue></center></p>
</template>
