import { ref } from 'vue'
import type { MembershipType, ExpiringMembershipInfo } from '../types'
import { useAuthStore } from '@/stores/auth'
import { membershipsService } from '../services/memberships-service'

export function useMemberships() {
  const authStore = useAuthStore()
  const types = ref<MembershipType[]>([])
  const expiring = ref<ExpiringMembershipInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchTypes() {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      types.value = await membershipsService.listTypes(authStore.token)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function updatePrice(typeId: number, price: number) {
    if (!authStore.token) return
    error.value = null
    try {
      const updated = await membershipsService.updateTypePrice(authStore.token, typeId, price)
      const idx = types.value.findIndex(t => t.id === typeId)
      if (idx !== -1) types.value[idx] = updated
      return updated
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function fetchExpiring() {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      expiring.value = await membershipsService.getExpiring(authStore.token)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  return { types, expiring, loading, error, fetchTypes, updatePrice, fetchExpiring }
}
