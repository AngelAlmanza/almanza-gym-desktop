<script setup lang="ts">
defineProps<{
  title: string
  message: string
  confirmLabel?: string
  cancelLabel?: string
  variant?: 'danger' | 'default'
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <el-dialog
    :title="title"
    model-value
    width="420px"
    :close-on-click-modal="true"
    :show-close="false"
    align-center
    @close="emit('cancel')"
  >
    <p style="color: var(--gym-text-secondary); font-size: 14px; line-height: 1.6;">
      {{ message }}
    </p>
    <template #footer>
      <el-button @click="emit('cancel')">
        {{ cancelLabel ?? 'Cancelar' }}
      </el-button>
      <el-button
        :type="variant === 'danger' ? 'danger' : 'primary'"
        @click="emit('confirm')"
      >
        {{ confirmLabel ?? 'Confirmar' }}
      </el-button>
    </template>
  </el-dialog>
</template>
