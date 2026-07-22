import { invoke } from '@tauri-apps/api/core'
import type {
  MembershipType,
  MembershipResponse,
  ExpiringMembershipInfo,
  AssignMembershipRequest,
  RenewMembershipRequest,
} from '../types'

export const membershipsService = {
  listTypes: (token: string) =>
    invoke<MembershipType[]>('list_membership_types', { token }),

  updateTypePrice: (token: string, typeId: number, price: number) =>
    invoke<MembershipType>('update_membership_type_price', { token, typeId, price }),

  assign: (token: string, request: AssignMembershipRequest) =>
    invoke<MembershipResponse>('assign_membership', { token, request }),

  renew: (token: string, request: RenewMembershipRequest) =>
    invoke<MembershipResponse>('renew_membership', { token, request }),

  getMemberMemberships: (token: string, memberId: number) =>
    invoke<MembershipResponse[]>('get_member_memberships', { token, memberId }),

  getExpiring: (token: string) =>
    invoke<ExpiringMembershipInfo[]>('get_expiring_memberships', { token }),
}
