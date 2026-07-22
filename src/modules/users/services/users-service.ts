import { invoke } from '@tauri-apps/api/core'
import type { User } from '@/modules/auth/types'
import type { CreateUserRequest, UpdateUserRequest } from '../types'

export const usersService = {
  list: (token: string) =>
    invoke<User[]>('list_users', { token }),

  create: (token: string, request: CreateUserRequest) =>
    invoke<User>('create_user', { token, request }),

  update: (token: string, userId: number, request: UpdateUserRequest) =>
    invoke<User>('update_user', { token, userId, request }),

  deactivate: (token: string, userId: number) =>
    invoke<void>('deactivate_user', { token, userId }),
}
