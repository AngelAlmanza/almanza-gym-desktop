import { ref } from 'vue'
import type { User } from '@/modules/auth/types'
import { useAuthStore } from '@/stores/auth'
import { usersService } from '../services/users-service'
import type { CreateUserRequest, UpdateUserRequest } from '../types'

export function useUsers() {
  const authStore = useAuthStore()
  const users = ref<User[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchUsers() {
    if (!authStore.token) return
    loading.value = true
    error.value = null
    try {
      users.value = await usersService.list(authStore.token)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function createUser(request: CreateUserRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      const newUser = await usersService.create(authStore.token, request)
      users.value.unshift(newUser)
      return newUser
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function updateUser(id: number, request: UpdateUserRequest) {
    if (!authStore.token) return
    error.value = null
    try {
      const updated = await usersService.update(authStore.token, id, request)
      const idx = users.value.findIndex((u) => u.id === id)
      if (idx !== -1) users.value[idx] = updated
      return updated
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function deactivateUser(id: number) {
    if (!authStore.token) return
    error.value = null
    try {
      await usersService.deactivate(authStore.token, id)
      const idx = users.value.findIndex((u) => u.id === id)
      if (idx !== -1) users.value[idx] = { ...users.value[idx], is_active: false }
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  return { users, loading, error, fetchUsers, createUser, updateUser, deactivateUser }
}
