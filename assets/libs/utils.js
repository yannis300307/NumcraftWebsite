export function is_usb_supported() {
  return navigator.usb !== undefined;
}