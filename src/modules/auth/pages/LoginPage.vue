<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const store = useAuthStore()

const form = reactive({
  username: '',
  password: '',
})
const error = ref('')
const loading = ref(false)

async function handleSubmit() {
  error.value = ''

  if (!form.username.trim() || !form.password) {
    error.value = 'Completa todos los campos'
    return
  }

  loading.value = true
  try {
    await store.login({
      username: form.username.trim(),
      password: form.password,
    })
    router.push('/')
  } catch {
    error.value = 'Usuario o contrasena incorrectos'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page">
    <div class="login-card">
      <div class="login-header">
        <div class="brand-icon">A</div>
        <h1 class="login-title">Almanza Gym</h1>
        <p class="login-subtitle">Iniciar sesion</p>
      </div>

      <el-form @submit.prevent="handleSubmit">
        <el-form-item label="Nombre de usuario">
          <el-input
            v-model="form.username"
            autocomplete="username"
          />
        </el-form-item>

        <el-form-item label="Contrasena">
          <el-input
            v-model="form.password"
            type="password"
            show-password
            autocomplete="current-password"
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
          {{ loading ? 'Ingresando...' : 'Ingresar' }}
        </el-button>
      </el-form>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 24px;
  background-color: var(--gym-bg);
}

.login-card {
  width: 100%;
  max-width: 380px;
  background: var(--gym-surface-elevated);
  border: 1px solid var(--gym-border);
  border-radius: 10px;
  padding: 40px 32px;
}

.login-header {
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

.login-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--gym-text);
  letter-spacing: -0.5px;
}

.login-subtitle {
  font-size: 14px;
  color: var(--gym-text-secondary);
  margin-top: 4px;
}
</style>
