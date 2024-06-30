import { type User } from '@/apis/api'
import { authService } from '@/services/authService'
import { defineStore } from 'pinia'

interface State {
  user?: User
  loaded: boolean
}

export const useAuthStore = defineStore('auth', {
  state: (): State => {
    return {
      user: undefined,
      loaded: false
    }
  },
  actions: {
    setUser(user?: User) {
      this.user = user
    },
    async fetchUser() {
      try {
        const res = await authService.fetchUser()
        this.setUser(res)
        this.loaded = true
      } catch (error) {
        this.loaded = true
      }
    }
  }
})
