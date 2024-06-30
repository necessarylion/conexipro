<template>
  <main>
    <RouterView v-if="loaded" />
    <div v-else>loading</div>
  </main>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import firebaseConfig from '@/utils/firebaseConfig'
import firebase from 'firebase/compat/app'
import { authService } from './services/authService'
import { useAuthStore } from './stores/authStore'

const authStore = useAuthStore()
const loaded = ref(false)

onMounted(async () => {
  firebase.initializeApp(firebaseConfig)
  try {
    const res = await authService.fetchUser()
    authStore.setUser(res)
    loaded.value = true
  } catch (error) {
    loaded.value = true
    // ignore error
  }
})
</script>
