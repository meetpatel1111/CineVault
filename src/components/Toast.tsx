import React, { useEffect, useState } from 'react';
import './Toast.css';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
}

interface ToastProps {
  toast: Toast;
  onClose: (id: string) => void;
}

const ToastItem: React.FC<ToastProps> = ({ toast, onClose }) => {
  const [isExiting, setIsExiting] = useState(false);

  useEffect(() => {
    const duration = toast.duration || 5000;
    const timer = setTimeout(() => {
      handleClose();
    }, duration);

    return () => clearTimeout(timer);
  }, [toast.duration]);

  const handleClose = () => {
    setIsExiting(true);
    setTimeout(() => {
      onClose(toast.id);
    }, 300); // Match animation duration
  };

  const getIcon = (type: ToastType): string => {
    switch (type) {
      case 'success':
        return '✓';
      case 'error':
        return '✕';
      case 'warning':
        return '⚠';
      case 'info':
        return 'ℹ';
      default:
        return 'ℹ';
    }
  };

  return (
    <div
      className={`toast toast--${toast.type} ${isExiting ? 'toast--exiting' : ''}`}
      role="alert"
    >
      <div className="toast__icon">{getIcon(toast.type)}</div>
      <div className="toast__message">{toast.message}</div>
      <button
        className="toast__close"
        onClick={handleClose}
        aria-label="Close notification"
      >
        <CloseIcon />
      </button>
    </div>
  );
};

interface ToastContainerProps {
  toasts: Toast[];
  onClose: (id: string) => void;
}

export const ToastContainer: React.FC<ToastContainerProps> = ({ toasts, onClose }) => {
  if (toasts.length === 0) return null;

  return (
    <div className="toast-container">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onClose={onClose} />
      ))}
    </div>
  );
};

const CloseIcon: React.FC = () => (
  <svg
    width="16"
    height="16"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M18 6L6 18M6 6l12 12"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
  </svg>
);

// Toast Manager Hook
export const useToast = () => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = (
    type: ToastType,
    message: string,
    duration?: number
  ) => {
    const id = Math.random().toString(36).substr(2, 9);
    const newToast: Toast = { id, type, message, duration };
    setToasts((prev) => [...prev, newToast]);
    return id;
  };

  const removeToast = (id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  };

  const success = (message: string, duration?: number) =>
    addToast('success', message, duration);

  const error = (message: string, duration?: number) =>
    addToast('error', message, duration);

  const warning = (message: string, duration?: number) =>
    addToast('warning', message, duration);

  const info = (message: string, duration?: number) =>
    addToast('info', message, duration);

  return {
    toasts,
    addToast,
    removeToast,
    success,
    error,
    warning,
    info,
  };
};
