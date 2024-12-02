export interface MessageProps {
  message?: string;
  duration?: number;
  showClose?: boolean;
  type?: 'success' | 'warning' | 'info' | 'error' 
}