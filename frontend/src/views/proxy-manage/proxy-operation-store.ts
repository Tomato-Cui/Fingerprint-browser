import { ref } from "vue";

export const checkRows = ref();

export const setCheckRow = (row: any) => {
  checkRows.value = row;
};

export const getCheckRow = () => {
  return checkRows.value;
};


