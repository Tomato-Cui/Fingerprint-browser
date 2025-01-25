export function formatDate(inputDate: string): string {
    // 将输入字符串转换为 Date 对象
    const date = new Date(inputDate);
    // 提取年月日时分秒
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // 月份从 0 开始，需要加 1
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    // 拼接成目标格式
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}
