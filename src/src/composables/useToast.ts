import { ref } from 'vue'

// Global toast instance reference
const toastInstance = ref<any>(null)

export function setToastInstance(instance: any) {
  toastInstance.value = instance
}

export function useToast() {
  return {
    success: (message: string, duration?: number) => {
      toastInstance.value?.success(message, duration)
    },
    error: (message: string, duration?: number) => {
      toastInstance.value?.error(message, duration)
    },
    info: (message: string, duration?: number) => {
      toastInstance.value?.info(message, duration)
    },
    warning: (message: string, duration?: number) => {
      toastInstance.value?.warning(message, duration)
    },
  }
}
