import { Api, type RequestParams, type User, type UserLoginResponse } from '@/apis/api'
import { authorizationHeader } from '@/utils'

class AuthService {
  private readonly api: Api<unknown>

  get requestParams(): RequestParams {
    return {
      headers: authorizationHeader()
    }
  }

  constructor() {
    this.api = new Api({
      baseUrl: import.meta.env.VITE_BASE_URL
    })
  }

  public async login(token: string): Promise<UserLoginResponse> {
    const res = await this.api.auth.login({ token })
    return res.data
  }

  public async fetchUser(): Promise<User> {
    const res = await this.api.auth.fetchUser(this.requestParams)
    return res.data
  }
}

const authService = new AuthService()
export { authService }
