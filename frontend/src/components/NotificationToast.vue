<template>
  <Teleport to="body">
    <div
      v-if="notifications.length > 0"
      class="notification-stack"
    >
      <TransitionGroup name="toast-fade">
        <v-card
          v-for="notification in notifications"
          :key="notification.id"
          class="notification-toast bg-[#363636dd] backdrop-blur-sm border border-[#ffffff22] text-white px-4 py-3 rounded-md shadow-lg"
          theme="dark"
        >
          <div class="flex items-center gap-3">
            <v-icon icon="mdi-alert-circle-outline" />
            <span class="text-sm break-words flex-1">{{ notification.message }}</span>
            <v-btn
              class="py-1 px-3 rounded-md bg-[#0B5087] hover:bg-[#0A3E6B] shrink-0"
              size="small"
              variant="elevated"
              theme="dark"
              @click="emit('action', notification.action.type)"
            >
              {{ notification.action.label }}
            </v-btn>
          </div>
        </v-card>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
export type NotificationActionType = 'goToSetup' | 'uploadScript' | 'updateScript'

export interface AppNotification {
  id: string
  message: string
  action: {
    type: NotificationActionType
    label: string
  }
}

defineProps<{
  notifications: AppNotification[]
}>()

const emit = defineEmits<{
  (e: 'action', actionType: NotificationActionType): void
}>()
</script>

<style scoped>
.notification-stack {
  position: fixed;
  bottom: 1rem;
  left: 50%;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: min(90vw, 640px);
  transform: translateX(-50%);
  pointer-events: none;
}

.notification-toast {
  pointer-events: auto;
}

.toast-fade-enter-active,
.toast-fade-leave-active {
  transition: opacity 0.2s ease;
}

.toast-fade-enter-from,
.toast-fade-leave-to {
  opacity: 0;
}
</style>
