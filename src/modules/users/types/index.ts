import type { UserRole } from '@/modules/auth/types'

export interface CreateUserRequest {
  username: string
  full_name: string
  password: string
  role: UserRole
}

export interface UpdateUserRequest {
  full_name?: string
  password?: string
  role?: UserRole
}
