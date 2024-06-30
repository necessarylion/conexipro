<template>
  <main>
    <RouterView v-if="authStore.loaded" />
  </main>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import firebaseConfig from '@/utils/firebaseConfig'
import firebase from 'firebase/compat/app'
import { useAuthStore } from './stores/authStore'

const authStore = useAuthStore()

onMounted(async () => {
  firebase.initializeApp(firebaseConfig)
  await authStore.fetchUser()
})
</script>
