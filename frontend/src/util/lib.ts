import type { Updater } from '@tanstack/vue-table'
import type { Ref } from 'vue'
import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function valueUpdater<T extends Updater<any>>(updaterOrValue: T, ref: Ref) {
  ref.value = typeof updaterOrValue === 'function'
    ? updaterOrValue(ref.value)
    : updaterOrValue
}

export const convertToCSV = (objArray: any[]) => {
  const array: any[] = [Object.keys(objArray[0])];

  objArray.forEach(obj => {
    array.push([...Object.values(obj)])
  });

  return array.map(row => row.join(',')).join('\n');
}

export const downloadCSV = (csv: any) => {
  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');

  link.download = 'data.csv';

  const url = URL.createObjectURL(blob);
  link.setAttribute('href', url);

  link.click();
  URL.revokeObjectURL(url);
}
