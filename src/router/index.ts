import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

import SetupPage from '@/modules/auth/pages/SetupPage.vue'
import LoginPage from '@/modules/auth/pages/LoginPage.vue'
import AppLayout from '@/shared/components/AppLayout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/setup',
      component: SetupPage,
      meta: { requiresAuth: false, requiresNoUsers: true },
    },
    {
      path: '/login',
      component: LoginPage,
      meta: { requiresAuth: false },
    },
    {
      path: '/',
      component: AppLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          redirect: '/members',
        },
        // Members routes
        {
          path: 'members',
          component: () => import('@/modules/members/pages/MembersListPage.vue'),
        },
        {
          path: 'members/new',
          component: () => import('@/modules/members/pages/MemberFormPage.vue'),
        },
        {
          path: 'members/:id',
          component: () => import('@/modules/members/pages/MemberProfilePage.vue'),
        },
        {
          path: 'members/:id/edit',
          component: () => import('@/modules/members/pages/MemberFormPage.vue'),
          meta: { requiredPermission: 'members:edit' },
        },
        // Users routes
        {
          path: 'users',
          component: () => import('@/modules/users/pages/UsersListPage.vue'),
          meta: { requiredPermission: 'users:manage' },
        },
        {
          path: 'users/new',
          component: () => import('@/modules/users/pages/UserFormPage.vue'),
          meta: { requiredPermission: 'users:create_cashier' },
        },
        {
          path: 'users/:id/edit',
          component: () => import('@/modules/users/pages/UserFormPage.vue'),
          meta: { requiredPermission: 'users:manage' },
        },
        // Memberships routes
        {
          path: 'memberships/settings',
          component: () => import('@/modules/memberships/pages/MembershipSettingsPage.vue'),
          meta: { requiredPermission: 'memberships:settings' },
        },
        {
          path: 'memberships/assign/:memberId',
          component: () => import('@/modules/memberships/pages/AssignPage.vue'),
          meta: { requiredPermission: 'memberships:manage' },
        },
        {
          path: 'memberships/renew/:memberId',
          component: () => import('@/modules/memberships/pages/RenewPage.vue'),
          meta: { requiredPermission: 'memberships:manage' },
        },
        {
          path: 'memberships/expiring',
          component: () => import('@/modules/memberships/pages/ExpiringPage.vue'),
          meta: { requiredPermission: 'memberships:manage' },
        },
      ],
    },
  ],
})

let initialized = false

router.beforeEach(async (to) => {
  const authStore = useAuthStore()

  if (!initialized) {
    await authStore.initialize()
    initialized = true
  }

  // Redirect to setup if no users exist
  if (authStore.needsSetup && to.path !== '/setup') {
    return '/setup'
  }

  // If setup is done, block access to /setup
  if (!authStore.needsSetup && to.path === '/setup') {
    return '/login'
  }

  // If route requires auth and user is not authenticated
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    return '/login'
  }

  // If user is authenticated and going to login, redirect to home
  if (to.path === '/login' && authStore.isAuthenticated) {
    return '/'
  }

  // Check permissions
  const requiredPermission = to.meta.requiredPermission as string | undefined
  if (requiredPermission && !authStore.hasPermission(requiredPermission)) {
    return '/'
  }
})

export default router
