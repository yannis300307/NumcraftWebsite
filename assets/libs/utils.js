//import { raise_error } from './pkg';
//const wasm = import("/src");

var calculator = undefined;



export function is_usb_supported() {
  return navigator.usb !== undefined;
}

export function detect() {
  if (calculator === undefined) {
    calculator = new window.Upsilon();
  }
  calculator.detect();
}