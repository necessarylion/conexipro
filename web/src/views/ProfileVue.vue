<template>
  <div>
    This is about view
    {{ authStore.user }}
  </div>
</template>

<script lang="ts" setup>
import { useAuthStore } from '@/stores/authStore'

const authStore = useAuthStore()
</script>
