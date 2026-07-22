<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const store = useAuthStore()

const form = reactive({
  username: '',
  fullName: '',
  password: '',
  confirmPassword: '',
})
const error = ref('')
const loading = ref(false)

async function handleSubmit() {
  error.value = ''

  if (!form.username.trim()) {
    error.value = 'El nombre de usuario es requerido'
    return
  }
  if (!form.fullName.trim()) {
    error.value = 'El nombre completo es requerido'
    return
  }
  if (form.password.length < 8) {
    error.value = 'La contrasena debe tener al menos 8 caracteres'
    return
  }
  if (form.password !== form.confirmPassword) {
    error.value = 'Las contrasenas no coinciden'
    return
  }

  loading.value = true
  try {
    await store.setupAdmin({
      username: form.username.trim(),
      full_name: form.fullName.trim(),
      password: form.password,
    })
    router.push('/')
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="setup-page">
    <div class="setup-card">
      <div class="setup-header">
        <div class="brand-icon">A</div>
        <h1 class="setup-title">Almanza Gym</h1>
        <p class="setup-subtitle">Configuracion inicial del sistema</p>
      </div>

      <el-form label-position="top" @submit.prevent="handleSubmit">
        <el-form-item label="Nombre de usuario">
          <el-input
            v-model="form.username"
            placeholder="admin"
            autocomplete="username"
          />
        </el-form-item>

        <el-form-item label="Nombre completo">
          <el-input
            v-model="form.fullName"
            placeholder="Carlos Almanza"
          />
        </el-form-item>

        <el-form-item label="Contrasena">
          <el-input
            v-model="form.password"
            type="password"
            show-password
            placeholder="Minimo 8 caracteres"
            autocomplete="new-password"
          />
        </el-form-item>

        <el-form-item label="Confirmar contrasena">
          <el-input
            v-model="form.confirmPassword"
            type="password"
            show-password
            placeholder="Repetir contrasena"
            autocomplete="new-password"
          />
        </el-form-item>

        <el-alert
          v-if="error"
          :title="error"
          type="error"
          show-icon
          :closable="false"
          style="margin-bottom: 20px;"
        />

        <el-button
          type="primary"
          native-type="submit"
          :loading="loading"
          size="large"
          style="width: 100%;"
        >
          {{ loading ? 'Creando cuenta...' : 'Crear cuenta de administrador' }}
        </el-button>
      </el-form>
    </div>
  </div>
</template>

<style scoped>
.setup-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 24px;
  background-color: var(--gym-bg);
}

.setup-card {
  width: 100%;
  max-width: 420px;
  background: var(--gym-surface-elevated);
  border: 1px solid var(--gym-border);
  border-radius: 10px;
  padding: 40px 32px;
}

.setup-header {
  text-align: center;
  margin-bottom: 32px;
}

.brand-icon {
  width: 48px;
  height: 48px;
  background: var(--gym-accent);
  border-radius: 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-weight: 800;
  font-size: 24px;
  color: #fff;
  margin-bottom: 16px;
}

.setup-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--gym-text);
  letter-spacing: -0.5px;
}

.setup-subtitle {
  font-size: 14px;
  color: var(--gym-text-secondary);
  margin-top: 4px;
}
</style>
