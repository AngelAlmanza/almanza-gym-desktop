<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useMembers } from '../composables/useMembers'
import { Search } from '@element-plus/icons-vue'

const router = useRouter()
const { members, searchQuery, loading, error, fetchMembers } = useMembers()

onMounted(fetchMembers)

function goToProfile(id: number) {
  router.push(`/members/${id}`)
}
</script>

<template>
  <div class="page page-wide">
    <div class="page-header">
      <h1 class="page-title">Miembros</h1>
      <el-button type="primary" @click="router.push('/members/new')">
        Nuevo miembro
      </el-button>
    </div>

    <el-input
      v-model="searchQuery"
      placeholder="Buscar por nombre, codigo o telefono..."
      :prefix-icon="Search"
      clearable
      style="margin-bottom: 16px;"
    />

    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 16px;"
    />

    <el-table
      v-loading="loading"
      :data="members"
      style="width: 100%"
      row-class-name="clickable-row"
      @row-click="(row: any) => goToProfile(row.id)"
    >
      <el-table-column prop="full_name" label="Nombre" />
      <el-table-column prop="phone" label="Telefono" width="160" />
      <el-table-column label="Codigo de acceso" width="180">
        <template #default="{ row }">
          <span class="access-code">{{ row.access_code }}</span>
        </template>
      </el-table-column>
      <el-table-column label="Acciones" width="120" align="center">
        <template #default="{ row }">
          <el-button size="small" @click.stop="goToProfile(row.id)">
            Ver perfil
          </el-button>
        </template>
      </el-table-column>
      <template #empty>
        <p style="color: var(--gym-text-secondary); padding: 32px 0;">
          {{ searchQuery ? 'No se encontraron miembros' : 'No hay miembros registrados' }}
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
  justify-content: space-between;
  margin-bottom: 20px;
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--gym-text);
}

.access-code {
  font-family: 'Courier New', monospace;
  font-weight: 600;
  letter-spacing: 2px;
  color: var(--gym-accent);
}

:deep(.clickable-row) {
  cursor: pointer;
}
</style>
