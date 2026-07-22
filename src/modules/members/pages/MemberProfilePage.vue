<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useMember } from '../composables/useMember'
import { useAuth } from '@/modules/auth'
import { useMemberMembership } from '@/modules/memberships/composables/useMemberMembership'
import StatusBadge from '@/shared/components/StatusBadge.vue'
import ConfirmDialog from '@/shared/components/ConfirmDialog.vue'

const route = useRoute()
const router = useRouter()
const { hasPermission } = useAuth()
const { member, loading, error, fetchMember, deactivateMember, regenerateCode } = useMember()
const { memberships, loading: membershipLoading, fetchMemberships } = useMemberMembership()

const memberId = computed(() => Number(route.params.id))
const showDeactivateDialog = ref(false)
const showRegenerateDialog = ref(false)

const canEdit = computed(() => hasPermission('members:edit'))
const canManageMemberships = computed(() => hasPermission('memberships:manage'))

const activeMembership = computed(() =>
  memberships.value.find(m => m.status === 'active') ?? null
)

const membershipHistory = computed(() =>
  memberships.value.filter(m => m.status !== 'active')
)

onMounted(async () => {
  await fetchMember(memberId.value)
  await fetchMemberships(memberId.value)
})

async function handleDeactivate() {
  await deactivateMember(memberId.value)
  showDeactivateDialog.value = false
}

async function handleRegenerate() {
  await regenerateCode(memberId.value)
  showRegenerateDialog.value = false
}
</script>

<template>
  <div class="page page-wide">
    <div v-if="loading" v-loading="true" style="height: 200px;" />
    <el-alert
      v-else-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
    />

    <template v-else-if="member">
      <div class="profile-header">
        <div>
          <h1 class="member-name">{{ member.full_name }}</h1>
          <StatusBadge :status="member.is_active ? 'active' : 'inactive'" />
        </div>
        <div class="header-actions">
          <el-button
            v-if="canEdit"
            @click="router.push(`/members/${member.id}/edit`)"
          >
            Editar
          </el-button>
          <el-button
            v-if="canEdit && member.is_active"
            @click="showRegenerateDialog = true"
          >
            Regenerar codigo
          </el-button>
          <el-button
            v-if="canEdit && member.is_active"
            type="danger"
            @click="showDeactivateDialog = true"
          >
            Desactivar
          </el-button>
        </div>
      </div>

      <!-- Access code highlight -->
      <div class="access-code-card">
        <span class="code-label">Codigo de acceso</span>
        <span class="code-value">{{ member.access_code }}</span>
      </div>

      <div class="info-grid">
        <!-- Personal data -->
        <el-card>
          <template #header>
            <span class="card-title">Datos personales</span>
          </template>
          <div class="info-row">
            <span class="info-label">Telefono</span>
            <span class="info-value">{{ member.phone }}</span>
          </div>
          <div v-if="member.email" class="info-row">
            <span class="info-label">Correo</span>
            <span class="info-value">{{ member.email }}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Fecha de nacimiento</span>
            <span class="info-value">{{ member.date_of_birth }}</span>
          </div>
          <div v-if="member.emergency_contact" class="info-row">
            <span class="info-label">Contacto de emergencia</span>
            <span class="info-value">{{ member.emergency_contact }}</span>
          </div>
        </el-card>

        <!-- Membership -->
        <el-card>
          <template #header>
            <span class="card-title">Membresia</span>
          </template>

          <div v-if="membershipLoading" v-loading="true" style="height: 80px;" />

          <template v-else-if="activeMembership">
            <div class="info-row">
              <span class="info-label">Tipo</span>
              <span class="info-value">{{ activeMembership.membership_type_name }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Estado</span>
              <StatusBadge :status="activeMembership.status as 'active' | 'inactive' | 'expired' | 'cancelled'" />
            </div>
            <div class="info-row">
              <span class="info-label">Inicio</span>
              <span class="info-value">{{ activeMembership.start_date }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Vencimiento</span>
              <span class="info-value">{{ activeMembership.end_date }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">Precio pagado</span>
              <span class="info-value">${{ activeMembership.price_paid.toFixed(2) }}</span>
            </div>
          </template>
          <p v-else class="info-placeholder">Sin membresia activa</p>

          <div v-if="canManageMemberships && member.is_active" class="membership-actions">
            <el-button
              v-if="!activeMembership"
              type="primary"
              size="small"
              @click="router.push(`/memberships/assign/${member.id}`)"
            >
              Asignar membresia
            </el-button>
            <el-button
              size="small"
              @click="router.push(`/memberships/renew/${member.id}`)"
            >
              Renovar
            </el-button>
          </div>

          <!-- History -->
          <template v-if="membershipHistory.length > 0">
            <div class="history-divider">
              <span class="card-title">Historial</span>
            </div>
            <div v-for="h in membershipHistory" :key="h.id" class="history-row">
              <div class="history-info">
                <span class="history-type">{{ h.membership_type_name }}</span>
                <span class="history-dates">{{ h.start_date }} — {{ h.end_date }}</span>
              </div>
              <StatusBadge :status="h.status as 'active' | 'inactive' | 'expired' | 'cancelled'" />
            </div>
          </template>
        </el-card>
      </div>
    </template>

    <ConfirmDialog
      v-if="showDeactivateDialog"
      title="Desactivar miembro"
      message="El miembro no podra acceder al gimnasio. Sus datos se conservaran."
      confirm-label="Desactivar"
      variant="danger"
      @confirm="handleDeactivate"
      @cancel="showDeactivateDialog = false"
    />

    <ConfirmDialog
      v-if="showRegenerateDialog"
      title="Regenerar codigo de acceso"
      message="Se generara un nuevo codigo. El codigo anterior dejara de funcionar."
      confirm-label="Regenerar"
      @confirm="handleRegenerate"
      @cancel="showRegenerateDialog = false"
    />
  </div>
</template>

<style scoped>
.page {
  width: min(100%, var(--gym-page-wide));
}

.profile-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 24px;
  gap: 16px;
}

.member-name {
  font-size: 26px;
  font-weight: 600;
  color: var(--gym-text);
  margin-bottom: 6px;
}

.header-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.access-code-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  background: var(--gym-surface-elevated);
  border: 1px solid var(--gym-border);
  border-radius: 8px;
  padding: 16px 24px;
  margin-bottom: 24px;
}

.code-label {
  font-size: 13px;
  color: var(--gym-text-secondary);
  font-weight: 500;
}

.code-value {
  font-family: 'Courier New', monospace;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: 6px;
  color: var(--gym-accent);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr));
  gap: 16px;
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--gym-text);
}

.info-row {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 8px 0;
  border-bottom: 1px solid var(--gym-border);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 13px;
  color: var(--gym-text-secondary);
}

.info-value {
  font-size: 13px;
  color: var(--gym-text);
  font-weight: 500;
  text-align: right;
  overflow-wrap: anywhere;
}

.info-placeholder {
  font-size: 13px;
  color: var(--gym-text-muted);
  font-style: italic;
  margin-bottom: 16px;
}

.membership-actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}

.history-divider {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--gym-border);
  margin-bottom: 12px;
}

.history-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid var(--gym-border);
}

.history-row:last-child {
  border-bottom: none;
}

.history-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.history-type {
  font-size: 13px;
  color: var(--gym-text);
  font-weight: 500;
}

.history-dates {
  font-size: 12px;
  color: var(--gym-text-secondary);
}

@media (max-width: 760px) {
  .profile-header,
  .access-code-card {
    align-items: flex-start;
    flex-direction: column;
  }

  .header-actions,
  .membership-actions {
    flex-wrap: wrap;
  }

  .code-value {
    font-size: 24px;
    letter-spacing: 4px;
  }
}
</style>
