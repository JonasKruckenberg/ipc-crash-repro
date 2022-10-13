interface Window {
  new_window: () => Promise<void>;
  message_windows: () => Promise<void>;
}
