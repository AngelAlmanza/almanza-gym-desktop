import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

// Mock vue-router
vi.mock('vue-router', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}))

// Mock auth service - use inline factory to avoid hoisting issues
vi.mock('@/modules/auth/services/auth-service', () => ({
  authService: {
    hasUsers: vi.fn(),
    setup: vi.fn(),
    login: vi.fn(),
    logout: vi.fn(),
    validateSession: vi.fn(),
  },
}))

import { useAuthStore } from '../auth'
import { authService } from '@/modules/auth/services/auth-service'

const mockedService = vi.mocked(authService)

describe('Auth Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
    localStorage.clear()
  })

  it('initializes with needsSetup=true when no users exist', async () => {
    mockedService.hasUsers.mockResolvedValue(false)
    const store = useAuthStore()

    await store.initialize()

    expect(store.needsSetup).toBe(true)
    expect(store.isAuthenticated).toBe(false)
    expect(store.isLoading).toBe(false)
  })

  it('initializes and validates existing session', async () => {
    const mockUser = { id: 1, username: 'admin', full_name: 'Admin', role: 'admin' as const, is_active: true }
    localStorage.setItem('almanza_session_token', 'test-token')
    mockedService.hasUsers.mockResolvedValue(true)
    mockedService.validateSession.mockResolvedValue(mockUser)

    const store = useAuthStore()
    await store.initialize()

    expect(store.user).toEqual(mockUser)
    expect(store.isAuthenticated).toBe(true)
    expect(store.needsSetup).toBe(false)
  })

  it('clears token when session validation fails', async () => {
    localStorage.setItem('almanza_session_token', 'expired-token')
    mockedService.hasUsers.mockResolvedValue(true)
    mockedService.validateSession.mockRejectedValue(new Error('Invalid'))

    const store = useAuthStore()
    await store.initialize()

    expect(store.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
    expect(localStorage.getItem('almanza_session_token')).toBeNull()
  })

  it('login stores token and sets user', async () => {
    const mockUser = { id: 1, username: 'admin', full_name: 'Admin', role: 'admin' as const, is_active: true }
    mockedService.login.mockResolvedValue([mockUser, 'new-token'])

    const store = useAuthStore()
    await store.login({ username: 'admin', password: 'password123' })

    expect(store.user).toEqual(mockUser)
    expect(store.token).toBe('new-token')
    expect(localStorage.getItem('almanza_session_token')).toBe('new-token')
  })

  it('logout clears state', async () => {
    const mockUser = { id: 1, username: 'admin', full_name: 'Admin', role: 'admin' as const, is_active: true }
    mockedService.login.mockResolvedValue([mockUser, 'token'])

    const store = useAuthStore()
    await store.login({ username: 'admin', password: 'password123' })

    mockedService.logout.mockResolvedValue(undefined)
    await store.logout()

    expect(store.user).toBeNull()
    expect(store.token).toBeNull()
    expect(localStorage.getItem('almanza_session_token')).toBeNull()
  })

  it('hasPermission respects role matrix - admin has full access', async () => {
    const adminUser = { id: 1, username: 'admin', full_name: 'Admin', role: 'admin' as const, is_active: true }
    mockedService.login.mockResolvedValue([adminUser, 'token'])

    const store = useAuthStore()
    await store.login({ username: 'admin', password: 'pass' })

    expect(store.hasPermission('users:manage')).toBe(true)
    expect(store.hasPermission('members:manage')).toBe(true)
    expect(store.hasPermission('memberships:settings')).toBe(true)
  })

  it('cashier cannot manage users or memberships settings', async () => {
    const cashierUser = { id: 2, username: 'cajero', full_name: 'Cajero', role: 'cashier' as const, is_active: true }
    mockedService.login.mockResolvedValue([cashierUser, 'token'])

    const store = useAuthStore()
    await store.login({ username: 'cajero', password: 'pass' })

    expect(store.hasPermission('users:manage')).toBe(false)
    expect(store.hasPermission('memberships:settings')).toBe(false)
    expect(store.hasPermission('members:manage')).toBe(true)
  })

  it('manager can create cashier but not manage all users', async () => {
    const managerUser = { id: 3, username: 'gerente', full_name: 'Gerente', role: 'manager' as const, is_active: true }
    mockedService.login.mockResolvedValue([managerUser, 'token'])

    const store = useAuthStore()
    await store.login({ username: 'gerente', password: 'pass' })

    expect(store.hasPermission('users:manage')).toBe(false)
    expect(store.hasPermission('users:create_cashier')).toBe(true)
    expect(store.hasPermission('memberships:manage')).toBe(true)
    expect(store.hasPermission('memberships:settings')).toBe(false)
  })
})
