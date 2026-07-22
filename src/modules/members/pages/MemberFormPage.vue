<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useMembers } from '../composables/useMembers'
import { useMember } from '../composables/useMember'

const route = useRoute()
const router = useRouter()
const { createMember } = useMembers()
const { member, fetchMember, updateMember } = useMember()

const editId = computed(() => {
  const id = route.params.id
  return id ? Number(id) : null
})
const isEdit = computed(() => editId.value !== null)

const form = reactive({
  fullName: '',
  phone: '',
  email: '',
  dateOfBirth: '',
  emergencyContact: '',
})
const error = ref('')
const loading = ref(false)

onMounted(async () => {
  if (isEdit.value && editId.value) {
    await fetchMember(editId.value)
    if (member.value) {
      form.fullName = member.value.full_name
      form.phone = member.value.phone
      form.email = member.value.email ?? ''
      form.dateOfBirth = member.value.date_of_birth
      form.emergencyContact = member.value.emergency_contact ?? ''
    }
  }
})

async function handleSubmit() {
  error.value = ''

  if (!form.fullName.trim()) {
    error.value = 'El nombre completo es requerido'
    return
  }
  if (!form.phone.trim()) {
    error.value = 'El telefono es requerido'
    return
  }
  if (!form.dateOfBirth) {
    error.value = 'La fecha de nacimiento es requerida'
    return
  }

  loading.value = true
  try {
    if (isEdit.value && editId.value) {
      await updateMember(editId.value, {
        full_name: form.fullName.trim(),
        phone: form.phone.trim(),
        email: form.email.trim() || null,
        date_of_birth: form.dateOfBirth,
        emergency_contact: form.emergencyContact.trim() || null,
      })
      router.push(`/members/${editId.value}`)
    } else {
      const newMember = await createMember({
        full_name: form.fullName.trim(),
        phone: form.phone.trim(),
        email: form.email.trim() || null,
        date_of_birth: form.dateOfBirth,
        emergency_contact: form.emergencyContact.trim() || null,
      })
      if (newMember) {
        router.push(`/members/${newMember.id}`)
      }
    }
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="page page-narrow page-form-grid">
    <h1 class="page-title">{{ isEdit ? 'Editar miembro' : 'Nuevo miembro' }}</h1>

    <el-card>
      <el-form label-position="top" @submit.prevent="handleSubmit">
        <el-form-item label="Nombre completo *">
          <el-input v-model="form.fullName" />
        </el-form-item>

        <el-form-item label="Telefono *">
          <el-input v-model="form.phone" />
        </el-form-item>

        <el-form-item label="Fecha de nacimiento *">
          <el-input v-model="form.dateOfBirth" type="date" />
        </el-form-item>

        <el-form-item label="Correo electronico">
          <el-input v-model="form.email" type="email" />
        </el-form-item>

        <el-form-item label="Contacto de emergencia">
          <el-input v-model="form.emergencyContact" />
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
          <el-button @click="router.back()">Cancelar</el-button>
          <el-button type="primary" native-type="submit" :loading="loading">
            {{ loading ? 'Guardando...' : (isEdit ? 'Guardar cambios' : 'Registrar miembro') }}
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
