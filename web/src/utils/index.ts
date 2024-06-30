const AUTH_TOKEN = 'SYSTEM_AUTH_TOKEN'

export function authorizationHeader(): Record<string, string> {
  return {
    Authorization: 'Bearer ' + localStorage.getItem(AUTH_TOKEN)
  }
}

export function setAuthToken(token: string) {
  window.localStorage.setItem(AUTH_TOKEN, token)
}
