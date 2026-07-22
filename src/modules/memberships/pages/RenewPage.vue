<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useMemberMembership } from '../composables/useMemberMembership'
import { membersService } from '@/modules/members/services/members-service'
import { useAuthStore } from '@/stores/auth'
import type { Member } from '@/modules/members/types'
import StatusBadge from '@/shared/components/StatusBadge.vue'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const { memberships, types, error, fetchMemberships, fetchTypes, renewMembership } = useMemberMembership()

const memberId = computed(() => Number(route.params.memberId))
const member = ref<Member | null>(null)
const selectedTypeId = ref<number | null>(null)
const studentVerified = ref(false)
const submitting = ref(false)
const loadingData = ref(true)

const currentMembership = computed(() =>
  memberships.value.length > 0 ? memberships.value[0] : null
)

const isCurrentActive = computed(() => currentMembership.value?.status === 'active')

const selectedType = computed(() =>
  types.value.find(t => t.id === selectedTypeId.value) ?? null
)

const isStudentType = computed(() => selectedType.value?.type === 'student')

const canSubmit = computed(() => {
  if (!selectedTypeId.value || !selectedType.value) return false
  if (selectedType.value.price <= 0) return false
  if (isStudentType.value && !studentVerified.value) return false
  return true
})

const renewalDates = computed(() => {
  if (!selectedType.value) return null
  if (isCurrentActive.value && currentMembership.value) {
    const currentEnd = currentMembership.value.end_date
    const endDate = new Date(currentEnd + 'T00:00:00')
    endDate.setDate(endDate.getDate() + selectedType.value.duration_days)
    return {
      type: 'early' as const,
      start: currentEnd,
      end: endDate.toISOString().split('T')[0],
    }
  } else {
    const today = new Date()
    const todayStr = today.toISOString().split('T')[0]
    const end = new Date(today)
    end.setDate(end.getDate() + selectedType.value.duration_days)
    return {
      type: 'late' as const,
      start: todayStr,
      end: end.toISOString().split('T')[0],
    }
  }
})

onMounted(async () => {
  await Promise.all([fetchTypes(), fetchMemberships(memberId.value)])
  if (authStore.token) {
    try {
      member.value = await membersService.get(authStore.token, memberId.value)
    } catch (e) {
      error.value = String(e)
    }
  }
  loadingData.value = false
})

async function handleSubmit() {
  if (!canSubmit.value || !selectedTypeId.value) return
  submitting.value = true
  try {
    await renewMembership({
      member_id: memberId.value,
      membership_type_id: selectedTypeId.value,
      student_credential_verified: studentVerified.value,
    })
    router.push(`/members/${memberId.value}`)
  } catch {
    // error set by composable
  } finally {
    submitting.value = false
  }
}

function formatPrice(price: number): string {
  return `$${price.toFixed(2)}`
}
</script>

<template>
  <div class="page page-narrow">
    <h1 class="page-title">Renovar membresia</h1>

    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 16px;"
    />

    <div v-if="loadingData" v-loading="true" style="height: 200px;" />

    <template v-else-if="member">
      <div class="member-info">
        <span class="member-label">Miembro:</span>
        <span class="member-name">{{ member.full_name }}</span>
      </div>

      <!-- Current membership -->
      <el-card v-if="currentMembership" style="margin-bottom: 16px;">
        <template #header>
          <span class="card-title">Membresia actual</span>
        </template>
        <div class="detail-row">
          <span class="detail-label">Tipo</span>
          <span class="detail-value">{{ currentMembership.membership_type_name }}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Estado</span>
          <StatusBadge :status="currentMembership.status as 'active' | 'inactive' | 'expired' | 'cancelled'" />
        </div>
        <div class="detail-row">
          <span class="detail-label">Vencimiento</span>
          <span class="detail-value">{{ currentMembership.end_date }}</span>
        </div>
      </el-card>

      <!-- Renewal form -->
      <el-card>
        <template #header>
          <span class="card-title">Seleccionar tipo para renovacion</span>
        </template>

        <div class="type-options">
          <label
            v-for="t in types.filter(t => t.is_active)"
            :key="t.id"
            :class="['type-option', { selected: selectedTypeId === t.id }]"
          >
            <input
              v-model="selectedTypeId"
              type="radio"
              :value="t.id"
              class="type-radio"
            />
            <div class="type-content">
              <div class="type-header">
                <span class="type-name">{{ t.name }}</span>
                <el-tag :type="t.type === 'student' ? 'success' : ''" size="small" effect="dark">
                  {{ t.type }}
                </el-tag>
              </div>
              <div class="type-meta">
                <span>{{ t.duration_days }} dias</span>
                <span v-if="t.price > 0" class="type-price">{{ formatPrice(t.price) }}</span>
                <span v-else class="type-no-price">Precio no configurado</span>
              </div>
            </div>
          </label>
        </div>

        <el-alert
          v-if="isStudentType"
          type="warning"
          :closable="false"
          style="margin-bottom: 16px;"
        >
          <el-checkbox v-model="studentVerified">
            Se verifico la credencial de estudiante
          </el-checkbox>
        </el-alert>

        <template v-if="renewalDates">
          <div class="summary">
            <h3 class="card-title">Calculo de renovacion</h3>

            <el-alert
              v-if="renewalDates.type === 'early'"
              type="info"
              :closable="false"
              title="Renovacion anticipada: se extiende desde el vencimiento actual"
              style="margin-bottom: 12px;"
            />
            <el-alert
              v-else
              type="warning"
              :closable="false"
              title="Membresia vencida: inicia desde hoy"
              style="margin-bottom: 12px;"
            />

            <div class="summary-row">
              <span class="summary-label">Nueva fecha de inicio</span>
              <span class="summary-value">{{ renewalDates.start }}</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">Nuevo vencimiento</span>
              <span class="summary-value">{{ renewalDates.end }}</span>
            </div>
            <div v-if="selectedType" class="summary-row">
              <span class="summary-label">Precio</span>
              <span class="summary-price">{{ formatPrice(selectedType.price) }}</span>
            </div>
          </div>
        </template>

        <div class="form-actions">
          <el-button @click="router.back()">Cancelar</el-button>
          <el-button
            type="primary"
            :disabled="!canSubmit"
            :loading="submitting"
            @click="handleSubmit"
          >
            {{ submitting ? 'Renovando...' : 'Renovar membresia' }}
          </el-button>
        </div>
      </el-card>
    </template>
  </div>
</template>

<style scoped>
.page {
  width: min(100%, var(--gym-page-narrow));
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--gym-text);
  margin-bottom: 20px;
}

.member-info {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  padding: 12px 16px;
  background: var(--gym-surface-elevated);
  border: 1px solid var(--gym-border);
  border-radius: 8px;
}

.member-label {
  font-size: 13px;
  color: var(--gym-text-secondary);
}

.member-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--gym-text);
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--gym-text);
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 0;
  border-bottom: 1px solid var(--gym-border);
}

.detail-row:last-child {
  border-bottom: none;
}

.detail-label {
  font-size: 13px;
  color: var(--gym-text-secondary);
}

.detail-value {
  font-size: 13px;
  color: var(--gym-text);
  font-weight: 500;
}

.type-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 260px), 1fr));
  gap: 8px;
  margin-bottom: 16px;
}

.type-option {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px;
  border: 1px solid var(--gym-border);
  border-radius: 6px;
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
}

.type-option:hover {
  background: rgba(255, 255, 255, 0.02);
}

.type-option.selected {
  border-color: var(--gym-accent);
  background: var(--gym-accent-glow);
}

.type-radio {
  margin-top: 2px;
  accent-color: var(--gym-accent);
}

.type-content {
  flex: 1;
}

.type-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.type-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--gym-text);
}

.type-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  font-size: 13px;
  color: var(--gym-text-secondary);
}

.type-price {
  color: var(--gym-text);
  font-weight: 500;
}

.type-no-price {
  color: var(--gym-danger);
  font-style: italic;
  font-size: 12px;
}

.summary {
  padding-top: 16px;
  border-top: 1px solid var(--gym-border);
  margin-bottom: 16px;
}

.summary .card-title {
  margin-bottom: 12px;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 6px 0;
}

.summary-label {
  font-size: 13px;
  color: var(--gym-text-secondary);
}

.summary-value {
  font-size: 13px;
  color: var(--gym-text);
  font-weight: 500;
}

.summary-price {
  font-size: 18px;
  font-weight: 700;
  color: var(--gym-accent);
}

.form-actions {
  display: flex;
  gap: 8px;
  padding-top: 16px;
  border-top: 1px solid var(--gym-border);
  justify-content: flex-end;
}
</style>
