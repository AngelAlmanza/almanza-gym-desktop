import { ref } from 'vue'
import type { Member, UpdateMemberRequest } from '../types'
import { useAuthStore } from '@/stores/auth'
import { membersService } from '../services/members-service'

export function useMember() {
  const authStore = useAuthStore()
  const member = ref<Member | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchMember(id: number) {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      member.value = await membersService.get(authStore.token, id)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function updateMember(id: number, request: UpdateMemberRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      member.value = await membersService.update(authStore.token, id, request)
      return member.value
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function deactivateMember(id: number) {
    if (!authStore.token) return
    error.value = null
    try {
      await membersService.deactivate(authStore.token, id)
      if (member.value) {
        member.value = { ...member.value, is_active: false }
      }
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function regenerateCode(id: number) {
    if (!authStore.token) return
    error.value = null
    try {
      const newCode = await membersService.regenerateAccessCode(authStore.token, id)
      if (member.value) {
        member.value = { ...member.value, access_code: newCode }
      }
      return newCode
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  return { member, loading, error, fetchMember, updateMember, deactivateMember, regenerateCode }
}
