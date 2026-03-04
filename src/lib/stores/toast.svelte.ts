export type ToastType = "success" | "error" | "warning" | "info";

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number;
}

let _toasts = $state<Toast[]>([]);
let _nextId = 0;

export const toastStore = {
  get toasts() {
    return _toasts;
  },

  add(type: ToastType, message: string, duration = 4000) {
    const id = `toast-${_nextId++}`;
    const toast: Toast = { id, type, message, duration };
    _toasts = [..._toasts, toast];

    if (duration > 0) {
      setTimeout(() => {
        this.dismiss(id);
      }, duration);
    }

    return id;
  },

  dismiss(id: string) {
    _toasts = _toasts.filter((t) => t.id !== id);
  },

  success(message: string, duration?: number) {
    return this.add("success", message, duration);
  },

  error(message: string, duration = 6000) {
    return this.add("error", message, duration);
  },

  warning(message: string, duration?: number) {
    return this.add("warning", message, duration);
  },

  info(message: string, duration?: number) {
    return this.add("info", message, duration);
  },
};
