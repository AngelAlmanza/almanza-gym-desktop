import { ref } from 'vue'
import type {
  MembershipResponse,
  MembershipType,
  AssignMembershipRequest,
  RenewMembershipRequest,
} from '../types'
import { useAuthStore } from '@/stores/auth'
import { membershipsService } from '../services/memberships-service'

export function useMemberMembership() {
  const authStore = useAuthStore()
  const memberships = ref<MembershipResponse[]>([])
  const types = ref<MembershipType[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchMemberships(memberId: number) {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      memberships.value = await membershipsService.getMemberMemberships(authStore.token, memberId)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function fetchTypes() {
    if (!authStore.token) return
    try {
      types.value = await membershipsService.listTypes(authStore.token)
    } catch (e) {
      error.value = String(e)
    }
  }

  async function assignMembership(request: AssignMembershipRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      const result = await membershipsService.assign(authStore.token, request)
      return result
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function renewMembership(request: RenewMembershipRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      const result = await membershipsService.renew(authStore.token, request)
      return result
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  return { memberships, types, loading, error, fetchMemberships, fetchTypes, assignMembership, renewMembership }
}
