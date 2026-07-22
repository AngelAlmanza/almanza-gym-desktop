export interface User {
  id: number
  username: string
  full_name: string
  role: UserRole
  is_active: boolean
}

export type UserRole = 'admin' | 'manager' | 'cashier'

export interface LoginRequest {
  username: string
  password: string
}

export interface SetupRequest {
  username: string
  full_name: string
  password: string
}
