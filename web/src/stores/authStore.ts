import { UserDetail } from "@/apis/api"
import { defineStore } from "pinia"

interface State {
  user?: UserDetail
}

export const useUserStore = defineStore('auth', {
  state: (): State => {
    return {
      user: undefined,
    }
  },
})

