<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuth } from '@/modules/auth'

const route = useRoute()
const router = useRouter()
const { user, hasPermission, logout } = useAuth()

interface NavItem {
  label: string
  path: string
  icon: string
  permission?: string
}

const navItems: NavItem[] = [
  { label: 'Miembros', path: '/members', icon: 'members' },
  { label: 'Por vencer', path: '/memberships/expiring', icon: 'expiring', permission: 'memberships:manage' },
  { label: 'Config. membresias', path: '/memberships/settings', icon: 'settings', permission: 'memberships:settings' },
  { label: 'Usuarios', path: '/users', icon: 'users', permission: 'users:manage' },
]

const visibleItems = computed(() =>
  navItems.filter((item) => !item.permission || hasPermission(item.permission)),
)

function isActive(path: string): boolean {
  return route.path === path || route.path.startsWith(path + '/')
}

const roleLabel = computed(() => {
  const labels: Record<string, string> = {
    admin: 'Administrador',
    manager: 'Gerente',
    cashier: 'Cajero',
  }
  return user.value ? labels[user.value.role] ?? user.value.role : ''
})

function navigate(path: string) {
  router.push(path)
}

const userInitial = computed(() =>
  user.value?.full_name?.charAt(0).toUpperCase() ?? '?'
)
</script>

<template>
  <aside class="sidebar">
    <!-- Brand -->
    <div class="sidebar-brand">
      <div class="brand-mark">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
          <rect x="2" y="7" width="16" height="6" rx="1" fill="currentColor" opacity="0.3"/>
          <rect x="5" y="4" width="4" height="12" rx="1" fill="currentColor"/>
          <rect x="11" y="4" width="4" height="12" rx="1" fill="currentColor"/>
          <rect x="0" y="6" width="3" height="8" rx="1" fill="currentColor" opacity="0.6"/>
          <rect x="17" y="6" width="3" height="8" rx="1" fill="currentColor" opacity="0.6"/>
        </svg>
      </div>
      <div class="brand-label">
        <span class="brand-name">Almanza</span>
        <span class="brand-suffix">GYM</span>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="sidebar-nav">
      <button
        v-for="item in visibleItems"
        :key="item.path"
        :class="['nav-item', { active: isActive(item.path) }]"
        @click="navigate(item.path)"
      >
        <svg v-if="item.icon === 'members'" class="nav-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
          <circle cx="9" cy="7" r="4"/>
          <path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
          <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
        </svg>
        <svg v-else-if="item.icon === 'expiring'" class="nav-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/>
          <polyline points="12 6 12 12 16 14"/>
        </svg>
        <svg v-else-if="item.icon === 'settings'" class="nav-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
        </svg>
        <svg v-else-if="item.icon === 'users'" class="nav-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
          <circle cx="12" cy="7" r="4"/>
        </svg>
        <span class="nav-label">{{ item.label }}</span>
      </button>
    </nav>

    <!-- User -->
    <div class="sidebar-user">
      <div class="user-avatar">{{ userInitial }}</div>
      <div class="user-details">
        <span class="user-name">{{ user?.full_name }}</span>
        <span class="user-role">{{ roleLabel }}</span>
      </div>
      <button class="user-logout" title="Cerrar sesion" @click="logout">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
          <polyline points="16 17 21 12 16 7"/>
          <line x1="21" y1="12" x2="9" y2="12"/>
        </svg>
      </button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 224px;
  min-height: 100vh;
  background: var(--gym-bg);
  border-right: 1px solid var(--gym-border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  user-select: none;
}

/* ---- Brand ---- */
.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 16px 20px 20px;
}

.brand-mark {
  width: 36px;
  height: 36px;
  background: var(--gym-accent);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.brand-label {
  display: flex;
  flex-direction: column;
  line-height: 1.1;
}

.brand-name {
  font-size: 15px;
  font-weight: 700;
  color: var(--gym-text);
  letter-spacing: -0.4px;
}

.brand-suffix {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 3px;
  color: var(--gym-accent);
  text-transform: uppercase;
}

/* ---- Navigation ---- */
.sidebar-nav {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 12px 10px;
  border-top: 1px solid var(--gym-border);
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--gym-text-secondary);
  font-family: inherit;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
  text-align: left;
  width: 100%;
}

.nav-item:hover {
  background: var(--gym-surface);
  color: var(--gym-text);
}

.nav-item.active {
  background: var(--gym-surface-elevated);
  color: #fff;
}

/* Crimson power stripe — the signature */
.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 6px;
  bottom: 6px;
  width: 3px;
  background: var(--gym-accent);
  border-radius: 0 2px 2px 0;
}

.nav-icon {
  flex-shrink: 0;
  opacity: 0.65;
  transition: opacity 0.15s ease;
}

.nav-item:hover .nav-icon {
  opacity: 0.85;
}

.nav-item.active .nav-icon {
  opacity: 1;
  color: var(--gym-accent);
}

.nav-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ---- User ---- */
.sidebar-user {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px 14px 20px;
  border-top: 1px solid var(--gym-border);
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: rgba(229, 57, 53, 0.15);
  color: var(--gym-accent);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 700;
  flex-shrink: 0;
}

.user-details {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.user-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--gym-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.user-role {
  font-size: 11px;
  color: var(--gym-text-muted);
  letter-spacing: 0.2px;
}

.user-logout {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--gym-text-muted);
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
  flex-shrink: 0;
}

.user-logout:hover {
  background: rgba(239, 83, 80, 0.1);
  color: var(--gym-danger);
}

@media (max-width: 760px) {
  .sidebar {
    width: 100%;
    min-height: auto;
    border-right: none;
    border-bottom: 1px solid var(--gym-border);
  }

  .sidebar-brand {
    padding: 14px 16px;
  }

  .sidebar-nav {
    flex-direction: row;
    overflow-x: auto;
    padding: 8px 12px 12px;
  }

  .nav-item {
    flex: 0 0 auto;
    width: auto;
    min-width: 44px;
  }

  .nav-item.active::before {
    left: 10px;
    right: 10px;
    top: auto;
    bottom: 0;
    width: auto;
    height: 3px;
    border-radius: 2px 2px 0 0;
  }

  .sidebar-user {
    display: none;
  }
}
</style>
