<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUsers } from '../composables/useUsers'
import StatusBadge from '@/shared/components/StatusBadge.vue'
import ConfirmDialog from '@/shared/components/ConfirmDialog.vue'

const router = useRouter()
const { users, loading, error, fetchUsers, deactivateUser } = useUsers()

const showDeactivateDialog = ref(false)
const userToDeactivate = ref<number | null>(null)

onMounted(fetchUsers)

function confirmDeactivate(userId: number) {
  userToDeactivate.value = userId
  showDeactivateDialog.value = true
}

async function handleDeactivate() {
  if (userToDeactivate.value !== null) {
    await deactivateUser(userToDeactivate.value)
  }
  showDeactivateDialog.value = false
  userToDeactivate.value = null
}

const roleLabels: Record<string, string> = {
  admin: 'Administrador',
  manager: 'Gerente',
  cashier: 'Cajero',
}
</script>

<template>
  <div class="page page-wide">
    <div class="page-header">
      <h1 class="page-title">Usuarios</h1>
      <el-button type="primary" @click="router.push('/users/new')">
        Nuevo usuario
      </el-button>
    </div>

    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 16px;"
    />

    <el-table v-loading="loading" :data="users" style="width: 100%">
      <el-table-column prop="username" label="Usuario" width="160" />
      <el-table-column prop="full_name" label="Nombre completo" />
      <el-table-column label="Rol" width="140">
        <template #default="{ row }">
          {{ roleLabels[row.role] ?? row.role }}
        </template>
      </el-table-column>
      <el-table-column label="Estado" width="110" align="center">
        <template #default="{ row }">
          <StatusBadge :status="row.is_active ? 'active' : 'inactive'" />
        </template>
      </el-table-column>
      <el-table-column label="Acciones" width="200" align="center">
        <template #default="{ row }">
          <el-button size="small" @click="router.push(`/users/${row.id}/edit`)">
            Editar
          </el-button>
          <el-button
            v-if="row.is_active"
            size="small"
            type="danger"
            @click="confirmDeactivate(row.id)"
          >
            Desactivar
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <ConfirmDialog
      v-if="showDeactivateDialog"
      title="Desactivar usuario"
      message="El usuario no podra iniciar sesion. Esta accion se puede revertir."
      confirm-label="Desactivar"
      variant="danger"
      @confirm="handleDeactivate"
      @cancel="showDeactivateDialog = false"
    />
  </div>
</template>

<style scoped>
.page {
  width: min(100%, var(--gym-page-wide));
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--gym-text);
}
</style>
