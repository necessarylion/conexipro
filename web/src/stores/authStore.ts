import { UserDetail } from "@/apis/api"
import { defineStore } from "pinia"

interface State {
  user?: UserDetail
}

export const useAuthStore = defineStore('auth', {
  state: (): State => {
    return {
      user: undefined,
    }
  },
  actions: {
    setUser(user?: UserDetail) {
      this.user = user
    },
  },
})

