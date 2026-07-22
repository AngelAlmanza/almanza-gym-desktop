import { ref, watch } from 'vue'
import type { Member, CreateMemberRequest } from '../types'
import { useAuthStore } from '@/stores/auth'
import { membersService } from '../services/members-service'

export function useMembers() {
  const authStore = useAuthStore()
  const members = ref<Member[]>([])
  const searchQuery = ref('')
  const loading = ref(false)
  const error = ref<string | null>(null)

  let debounceTimer: ReturnType<typeof setTimeout> | null = null

  async function fetchMembers() {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      members.value = await membersService.list(authStore.token)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function search(query: string) {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      members.value = await membersService.search(authStore.token, query)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  watch(searchQuery, (q) => {
    if (debounceTimer) clearTimeout(debounceTimer)
    debounceTimer = setTimeout(() => {
      if (q.trim()) {
        search(q)
      } else {
        fetchMembers()
      }
    }, 300)
  })

  async function createMember(request: CreateMemberRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      const newMember = await membersService.create(authStore.token, request)
      return newMember
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  return { members, searchQuery, loading, error, fetchMembers, createMember }
}
