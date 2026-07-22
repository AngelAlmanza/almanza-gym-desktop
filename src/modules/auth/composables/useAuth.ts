import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'

export function useAuth() {
  const store = useAuthStore()

  const user = computed(() => store.user)
  const isAuthenticated = computed(() => store.isAuthenticated)
  const isAdmin = computed(() => store.user?.role === 'admin')
  const isManager = computed(() => store.user?.role === 'manager')
  const isCashier = computed(() => store.user?.role === 'cashier')

  function hasPermission(permission: string): boolean {
    return store.hasPermission(permission)
  }

  async function logout() {
    await store.logout()
  }

  return {
    user,
    isAuthenticated,
    isAdmin,
    isManager,
    isCashier,
    hasPermission,
    logout,
  }
}
