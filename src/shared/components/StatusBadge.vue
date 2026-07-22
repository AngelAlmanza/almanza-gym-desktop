<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  status: 'active' | 'inactive' | 'expired' | 'cancelled'
}>()

const tagType = computed(() => {
  const map: Record<string, 'success' | 'info' | 'warning' | 'danger'> = {
    active: 'success',
    inactive: 'info',
    expired: 'warning',
    cancelled: 'danger',
  }
  return map[props.status] ?? 'info'
})

const label = computed(() => {
  const labels: Record<string, string> = {
    active: 'Activo',
    inactive: 'Inactivo',
    expired: 'Vencida',
    cancelled: 'Cancelada',
  }
  return labels[props.status] ?? props.status
})
</script>

<template>
  <el-tag :type="tagType" size="small" effect="dark" round>
    {{ label }}
  </el-tag>
</template>
