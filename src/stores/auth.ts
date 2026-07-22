import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { useRouter } from 'vue-router'
import type { User, UserRole } from '@/modules/auth/types'
import { authService } from '@/modules/auth/services/auth-service'

const TOKEN_KEY = 'almanza_session_token'

const PERMISSIONS: Record<string, UserRole[]> = {
  'users:manage': ['admin'],
  'users:create_cashier': ['admin', 'manager'],
  'members:manage': ['admin', 'manager', 'cashier'],
  'members:edit': ['admin', 'manager'],
  'memberships:manage': ['admin', 'manager'],
  'memberships:settings': ['admin'],
}

export const useAuthStore = defineStore('auth', () => {
  const router = useRouter()
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem(TOKEN_KEY))
  const isLoading = ref(true)
  const needsSetup = ref(false)

  const isAuthenticated = computed(() => user.value !== null)

  function hasPermission(permission: string): boolean {
    if (!user.value) return false
    const allowedRoles = PERMISSIONS[permission]
    if (!allowedRoles) return false
    return allowedRoles.includes(user.value.role)
  }

  async function initialize() {
    isLoading.value = true
    try {
      const hasExistingUsers = await authService.hasUsers()
      needsSetup.value = !hasExistingUsers

      if (needsSetup.value) {
        user.value = null
        token.value = null
        localStorage.removeItem(TOKEN_KEY)
        return
      }

      if (token.value) {
        try {
          const validatedUser = await authService.validateSession(token.value)
          user.value = validatedUser
        } catch {
          user.value = null
          token.value = null
          localStorage.removeItem(TOKEN_KEY)
        }
      }
    } finally {
      isLoading.value = false
    }
  }

  async function setupAdmin(request: { username: string; full_name: string; password: string }) {
    const [newUser, newToken] = await authService.setup(request)
    user.value = newUser
    token.value = newToken
    localStorage.setItem(TOKEN_KEY, newToken)
    needsSetup.value = false
  }

  async function login(request: { username: string; password: string }) {
    const [loggedUser, newToken] = await authService.login(request)
    user.value = loggedUser
    token.value = newToken
    localStorage.setItem(TOKEN_KEY, newToken)
  }

  async function logout() {
    if (token.value) {
      try {
        await authService.logout(token.value)
      } catch {
        // Ignore logout errors
      }
    }
    user.value = null
    token.value = null
    localStorage.removeItem(TOKEN_KEY)
    router.push('/login')
  }

  return {
    user,
    token,
    isLoading,
    needsSetup,
    isAuthenticated,
    hasPermission,
    initialize,
    setupAdmin,
    login,
    logout,
  }
})
