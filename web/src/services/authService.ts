import { Api, UserLoginResponse } from "@/apis/api";

class AuthService {

  private api: Api<unknown>;

  constructor() {
    this.api = new Api({
      baseUrl: import.meta.env.VITE_BASE_URL,
    });
  }

  public async login(token: string): Promise<UserLoginResponse> {
    let res = await this.api.auth.login({ token });
    return res.data;
  }
}

const authService = new AuthService()
export { authService }
