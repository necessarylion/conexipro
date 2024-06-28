import { User } from "@/apis/api"
import { defineStore } from "pinia"

interface State {
  user?: User
}

export const useAuthStore = defineStore('auth', {
  state: (): State => {
    return {
      user: undefined,
    }
  },
  actions: {
    setUser(user?: User) {
      this.user = user
    },
  },
})

