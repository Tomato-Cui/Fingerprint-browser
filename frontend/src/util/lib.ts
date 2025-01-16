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

export function generateRandomUserAgent() {
  const uaTemplates = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chromeVersion} Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{firefoxVersion}) Gecko/20100101 Firefox/{firefoxVersion}",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{safariVersion} Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chromeVersion} Safari/537.36 Edg/{edgeVersion}",

    "Mozilla/5.0 (Linux; Android {androidVersion}; {deviceModel}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chromeVersion} Mobile Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS {iosVersion} like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{safariVersion} Mobile/15E148 Safari/604.1",

    "Mozilla/5.0 (iPad; CPU OS {iosVersion} like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{safariVersion} Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android {androidVersion}; {deviceModel}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chromeVersion} Safari/537.36",
  ];

  const template = uaTemplates[Math.floor(Math.random() * uaTemplates.length)];

  const randomVersion = (min = 90, max = 120) => {
    const major = Math.floor(Math.random() * (max - min + 1)) + min;
    const minor = Math.floor(Math.random() * 10);
    const patch = Math.floor(Math.random() * 10);
    return `${major}.${minor}.${patch}`;
  };

  const randomAndroidVersion = () => {
    const versions = ["10", "11", "12", "13"];
    return versions[Math.floor(Math.random() * versions.length)];
  };

  const randomIOSVersion = () => {
    const versions = ["14_0", "15_0", "16_0", "17_0"];
    return versions[Math.floor(Math.random() * versions.length)];
  };

  const randomDeviceModel = () => {
    const models = [
      "SM-G991U", // Samsung Galaxy S21
      "Pixel 6", // Google Pixel 6
      "iPhone14,3", // iPhone 13 Pro Max
      "iPad7,11", // iPad 7th Gen
      "SM-T870", // Samsung Galaxy Tab S7
    ];
    return models[Math.floor(Math.random() * models.length)];
  };

  const ua = template
    .replace(/{chromeVersion}/g, randomVersion())
    .replace(/{firefoxVersion}/g, randomVersion(80, 120))
    .replace(/{safariVersion}/g, randomVersion(10, 17))
    .replace(/{edgeVersion}/g, randomVersion())
    .replace(/{androidVersion}/g, randomAndroidVersion())
    .replace(/{iosVersion}/g, randomIOSVersion())
    .replace(/{deviceModel}/g, randomDeviceModel());

  return ua;
}