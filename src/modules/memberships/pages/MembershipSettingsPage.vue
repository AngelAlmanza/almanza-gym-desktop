<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMemberships } from '../composables/useMemberships'

const { types, loading, error, fetchTypes, updatePrice } = useMemberships()

const editingId = ref<number | null>(null)
const editPrice = ref('')
const saving = ref(false)

onMounted(fetchTypes)

function startEdit(typeId: number, currentPrice: number) {
  editingId.value = typeId
  editPrice.value = String(currentPrice)
}

function cancelEdit() {
  editingId.value = null
  editPrice.value = ''
}

async function savePrice(typeId: number) {
  const price = parseFloat(editPrice.value)
  if (isNaN(price) || price < 0) return
  saving.value = true
  try {
    await updatePrice(typeId, price)
    editingId.value = null
  } finally {
    saving.value = false
  }
}

function formatPrice(price: number): string {
  return `$${price.toFixed(2)}`
}
</script>

<template>
  <div class="page page-wide">
    <h1 class="page-title">Configuracion de membresias</h1>

    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      :closable="false"
      style="margin-bottom: 16px;"
    />

    <el-table v-loading="loading" :data="types" style="width: 100%">
      <el-table-column label="Tipo" width="120">
        <template #default="{ row }">
          <el-tag :type="row.type === 'student' ? 'success' : ''" size="small" effect="dark">
            {{ row.type }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="name" label="Nombre" />
      <el-table-column label="Duracion" width="120">
        <template #default="{ row }">
          {{ row.duration_days }} dias
        </template>
      </el-table-column>
      <el-table-column label="Precio" width="180">
        <template #default="{ row }">
          <template v-if="editingId === row.id">
            <div class="edit-price">
              <span class="price-prefix">$</span>
              <el-input
                v-model="editPrice"
                size="small"
                style="width: 100px;"
                @keyup.enter="savePrice(row.id)"
                @keyup.escape="cancelEdit"
              />
            </div>
          </template>
          <template v-else>
            <span v-if="row.price > 0">{{ formatPrice(row.price) }}</span>
            <span v-else class="price-zero">Sin configurar</span>
          </template>
        </template>
      </el-table-column>
      <el-table-column label="Estado" width="110" align="center">
        <template #default="{ row }">
          <el-tag
            :type="row.is_active ? 'success' : 'info'"
            size="small"
            effect="dark"
          >
            {{ row.is_active ? 'Activo' : 'Inactivo' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="Acciones" width="200" align="center">
        <template #default="{ row }">
          <template v-if="editingId === row.id">
            <el-button
              type="primary"
              size="small"
              :loading="saving"
              @click="savePrice(row.id)"
            >
              Guardar
            </el-button>
            <el-button size="small" @click="cancelEdit">
              Cancelar
            </el-button>
          </template>
          <template v-else>
            <el-button size="small" @click="startEdit(row.id, row.price)">
              Editar precio
            </el-button>
          </template>
        </template>
      </el-table-column>
      <template #empty>
        <p style="color: var(--gym-text-secondary); padding: 32px 0;">
          No hay tipos de membresia configurados
        </p>
      </template>
    </el-table>
  </div>
</template>

<style scoped>
.page {
  width: min(100%, var(--gym-page-wide));
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: var(--gym-text);
  margin-bottom: 20px;
}

.edit-price {
  display: flex;
  align-items: center;
  gap: 4px;
}

.price-prefix {
  font-size: 14px;
  color: var(--gym-text-secondary);
}

.price-zero {
  color: var(--gym-text-muted);
  font-style: italic;
  font-size: 13px;
}
</style>
