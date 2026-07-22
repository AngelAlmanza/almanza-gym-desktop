<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useUsers } from '../composables/useUsers'
import type { UserRole } from '@/modules/auth/types'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const { createUser, updateUser, fetchUsers, users } = useUsers()

const editId = computed(() => {
  const id = route.params.id
  return id ? Number(id) : null
})
const isEdit = computed(() => editId.value !== null)

const form = reactive({
  username: '',
  fullName: '',
  password: '',
  role: 'cashier' as UserRole,
})
const error = ref('')
const loading = ref(false)

const availableRoles = computed(() => {
  if (authStore.user?.role === 'manager') {
    return [{ value: 'cashier', label: 'Cajero' }]
  }
  return [
    { value: 'admin', label: 'Administrador' },
    { value: 'manager', label: 'Gerente' },
    { value: 'cashier', label: 'Cajero' },
  ]
})

onMounted(async () => {
  if (isEdit.value) {
    await fetchUsers()
    const user = users.value.find((u) => u.id === editId.value)
    if (user) {
      form.username = user.username
      form.fullName = user.full_name
      form.role = user.role
    }
  }
})

async function handleSubmit() {
  error.value = ''

  if (!isEdit.value) {
    if (!form.username.trim()) {
      error.value = 'El nombre de usuario es requerido'
      return
    }
    if (form.password.length < 8) {
      error.value = 'La contrasena debe tener al menos 8 caracteres'
      return
    }
  }

  if (!form.fullName.trim()) {
    error.value = 'El nombre completo es requerido'
    return
  }

  if (isEdit.value && form.password && form.password.length < 8) {
    error.value = 'La contrasena debe tener al menos 8 caracteres'
    return
  }

  loading.value = true
  try {
    if (isEdit.value && editId.value) {
      await updateUser(editId.value, {
        full_name: form.fullName.trim(),
        password: form.password || undefined,
        role: form.role,
      })
    } else {
      await createUser({
        username: form.username.trim(),
        full_name: form.fullName.trim(),
        password: form.password,
        role: form.role,
      })
    }
    router.push('/users')
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="page page-narrow page-form-grid">
    <h1 class="page-title">{{ isEdit ? 'Editar usuario' : 'Nuevo usuario' }}</h1>

    <el-card>
      <el-form label-position="top" @submit.prevent="handleSubmit">
        <el-form-item label="Nombre de usuario">
          <el-input
            v-model="form.username"
            :disabled="isEdit"
            autocomplete="off"
          />
        </el-form-item>

        <el-form-item label="Nombre completo">
          <el-input v-model="form.fullName" />
        </el-form-item>

        <el-form-item :label="isEdit ? 'Nueva contrasena (dejar vacio para no cambiar)' : 'Contrasena'">
          <el-input
            v-model="form.password"
            type="password"
            show-password
            autocomplete="new-password"
            :placeholder="isEdit ? 'Sin cambios' : 'Minimo 8 caracteres'"
          />
        </el-form-item>

        <el-form-item label="Rol">
          <el-select v-model="form.role" style="width: 100%;">
            <el-option
              v-for="r in availableRoles"
              :key="r.value"
              :label="r.label"
              :value="r.value"
            />
          </el-select>
        </el-form-item>

        <el-alert
          v-if="error"
          :title="error"
          type="error"
          show-icon
          :closable="false"
          style="margin-bottom: 20px;"
        />

        <div class="form-actions">
          <el-button @click="router.push('/users')">Cancelar</el-button>
          <el-button type="primary" native-type="submit" :loading="loading">
            {{ loading ? 'Guardando...' : (isEdit ? 'Guardar cambios' : 'Crear usuario') }}
          </el-button>
        </div>
      </el-form>
    </el-card>
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
  margin-bottom: 24px;
}

:deep(.el-form) {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 280px), 1fr));
  column-gap: 16px;
}

:deep(.el-alert),
.form-actions {
  grid-column: 1 / -1;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 8px;
}
</style>
