import { invoke } from '@tauri-apps/api/core'
import type { LoginRequest, SetupRequest, User } from '../types'

export const authService = {
  hasUsers: () => invoke<boolean>('has_users'),

  setup: (request: SetupRequest) =>
    invoke<[User, string]>('setup_admin', { request }),

  login: (request: LoginRequest) =>
    invoke<[User, string]>('login', { request }),

  logout: (token: string) => invoke<void>('logout', { token }),

  validateSession: (token: string) =>
    invoke<User>('validate_session', { token }),
}
