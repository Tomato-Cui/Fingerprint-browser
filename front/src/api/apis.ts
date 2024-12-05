import axios from "axios"

//TODO 数据迁移
export const StartingMigrations = (data: any) => {
    // const data = await axios.post("/", data).then(())
    console.log("api数据：", data.gologinToken);
    
    return data;
}
export const fun1 = () => {
    console.log("----------------------------------------");
}

export default {
    fun1,
    StartingMigrations
}