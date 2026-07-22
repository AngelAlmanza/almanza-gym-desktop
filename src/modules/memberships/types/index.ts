export interface MembershipType {
  id: number
  type: string
  name: string
  price: number
  duration_days: number
  is_active: boolean
  updated_at: string
}

export interface MembershipResponse {
  id: number
  member_id: number
  membership_type_name: string
  membership_type: string
  status: string
  price_paid: number
  original_price: number
  start_date: string
  end_date: string
  student_credential_verified: boolean
  created_at: string
}

export interface ExpiringMembershipInfo {
  membership_id: number
  member_id: number
  member_name: string
  membership_type_name: string
  end_date: string
  days_remaining: number
}

export interface AssignMembershipRequest {
  member_id: number
  membership_type_id: number
  student_credential_verified: boolean
}

export interface RenewMembershipRequest {
  member_id: number
  membership_type_id: number
  student_credential_verified: boolean
}
