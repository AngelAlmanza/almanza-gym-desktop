import { invoke } from '@tauri-apps/api/core'
import type { Member, CreateMemberRequest, UpdateMemberRequest } from '../types'

export const membersService = {
  list: (token: string) =>
    invoke<Member[]>('list_members', { token }),

  get: (token: string, memberId: number) =>
    invoke<Member>('get_member', { token, memberId }),

  search: (token: string, query: string) =>
    invoke<Member[]>('search_members', { token, query }),

  create: (token: string, request: CreateMemberRequest) =>
    invoke<Member>('create_member', { token, request }),

  update: (token: string, memberId: number, request: UpdateMemberRequest) =>
    invoke<Member>('update_member', { token, memberId, request }),

  deactivate: (token: string, memberId: number) =>
    invoke<void>('deactivate_member', { token, memberId }),

  regenerateAccessCode: (token: string, memberId: number) =>
    invoke<string>('regenerate_access_code', { token, memberId }),
}
