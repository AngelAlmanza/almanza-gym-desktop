<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useMemberships } from '../composables/useMemberships'

const router = useRouter()
const { expiring, loading, error, fetchExpiring } = useMemberships()

onMounted(fetchExpiring)

function urgencyType(days: number): 'danger' | 'warning' | '' {
  if (days <= 0) return 'danger'
  if (days <= 1) return 'warning'
  return 'warning'
}

function urgencyLabel(days: number): string {
  if (days <= 0) return 'Vence hoy'
  if (days === 1) return '1 dia'
  return `${days} dias`
}
</script>

<template>
  <div class="page page-wide">
    <div class="page-header">
      <h1 class="page-title">Membresias por vencer</h1>
      <el-tag type="info" effect="dark">Proximos 3 dias</el-tag>
    </div>

    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 16px;"
    />

    <el-table v-loading="loading" :data="expiring" style="width: 100%">
      <el-table-column label="Miembro">
        <template #default="{ row }">
          <span
            class="member-link"
            @click="router.push(`/members/${row.member_id}`)"
          >
            {{ row.member_name }}
          </span>
        </template>
      </el-table-column>
      <el-table-column prop="membership_type_name" label="Tipo" width="160" />
      <el-table-column prop="end_date" label="Vencimiento" width="140" />
      <el-table-column label="Tiempo restante" width="160" align="center">
        <template #default="{ row }">
          <el-tag
            :type="urgencyType(row.days_remaining)"
            effect="dark"
            size="small"
          >
            {{ urgencyLabel(row.days_remaining) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="Acciones" width="120" align="center">
        <template #default="{ row }">
          <el-button
            type="primary"
            size="small"
            @click="router.push(`/memberships/renew/${row.member_id}`)"
          >
            Renovar
          </el-button>
        </template>
      </el-table-column>
      <template #empty>
        <p style="color: var(--gym-text-secondary); padding: 32px 0;">
          No hay membresias por vencer en los proximos 3 dias
        </p>
      </template>
    </el-table>
  </div>
</template>

<style scoped>
.page {
  width: min(100%, var(--gym-page-wide));
}

.page-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--gym-text);
}

.member-link {
  cursor: pointer;
  color: var(--gym-accent);
  font-weight: 500;
}

.member-link:hover {
  text-decoration: underline;
}
</style>
