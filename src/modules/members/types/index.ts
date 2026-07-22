export interface Member {
  id: number
  full_name: string
  phone: string
  email: string | null
  date_of_birth: string
  emergency_contact: string | null
  photo_path: string | null
  access_code: string
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface CreateMemberRequest {
  full_name: string
  phone: string
  email?: string | null
  date_of_birth: string
  emergency_contact?: string | null
  photo_path?: string | null
}

export interface UpdateMemberRequest {
  full_name?: string
  phone?: string
  email?: string | null
  date_of_birth?: string
  emergency_contact?: string | null
  photo_path?: string | null
}
