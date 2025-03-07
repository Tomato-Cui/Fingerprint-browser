<script setup lang="ts">
import { SidebarContent, SidebarFooter } from "@/components/ui/sidebar";
import { Card, CardContent } from "@/components/ui/card";
import { IconCardShopping } from "@/assets/icons/sidebar/index";
import { useUserStore } from "@/stores/user";

const userStore = useUserStore();
</script>

<template>
  <SidebarFooter class="pr-0">
    <SidebarContent
      className="rounded-lg light:shadow-xl bg-gray-200 dark:bg-gray-800 shadow-gray-300 overflow-hidden mx-1 relative mb-2 dark:text-black"
    >
      <div class="absolute inset-0 z-0">
        <div
          class="bg-red-200 w-20 h-20 absolute right-0 blur-xl animate-move-right"
        ></div>
        <div
          class="bg-green-200 w-20 h-20 absolute right-0 bottom-0 blur-xl animate-move-up"
        ></div>
        <div
          class="bg-blue-200 w-20 h-20 absolute right-1/4 bottom-1/4 blur-xl animate-move-diagonal"
        ></div>
        <div
          class="bg-purple-200 w-20 h-20 absolute right-1/3 top-0 blur-xl animate-move-left"
        ></div>
      </div>
      <Card className="backdrop-blur-[20px] bg-white/30 bg-opacity-55">
        <CardContent className="px-2 pb-2 space-y-[8px]">
          <div
            className="flex items-center gap-x-3 h-12 group-data-[state=collapsed]:flex-col"
          >
            <!-- <img
              src="https://img.freepik.com/free-vector/beautiful-woman-has-long-hair-pink-dress_1308-125946.jpg?t=st=1734769015~exp=1734772615~hmac=dfbb76ec0391f86c02c79773423791880b882012d3c3cb636daf171ecd61087d&w=2000"
              class="h-10 w-auto rounded-full"
            /> -->
            <div
              class="bg-gradient-to-l to-[#7D41FF] from-[#5E51FF] w-9 h-9 rounded-lg flex items-center justify-center text-white"
            >
              {{
                userStore.userInfo.account
                  ? userStore.userInfo.account.substring(0, 2)
                  : ""
              }}
            </div>
            <div className="flex-1 min-w-0 group-data-[state=collapsed]:hidden">
              <h3 className="font-semibold truncate">
                {{ userStore.userInfo.account }}
              </h3>
              <p className="text-xs truncate">
                {{ userStore.userInfo.email }}
              </p>
            </div>
            <div
              className="text-xs py-[.10rem] text-muted-foreground bg-white/80 text-black shadow-md rounded-md px-[0.35rem] whitespace-nowrap dark:text-black"
            >
              免费
            </div>
          </div>

          <button
            className="w-full text-[14px] bg-white/20 rounded-md shadow-md flex items-center justify-center gap-x-2 py-2 group-data-[state=collapsed]:flex-col group-data-[state=collapsed]:px-4 dark:text-black"
            variant="outline"
          >
            <IconCardShopping class="size-5" />
            升级套餐
          </button>
          <ul
            class="space-y-4 py-4 bg-white/20 group-data-[state=collapsed]:hidden shadow-md rounded-md dark:text-black"
          >
            <li class="flex justify-between px-4 text-xs">
              <p>环境数量</p>
              <p>1/2</p>
            </li>
            <li class="flex justify-between px-4 text-xs">
              <p>成员数量</p>
              <p>1/2</p>
            </li>
            <li class="flex justify-between px-4 text-xs">
              <p>启动数量</p>
              <p>1/2</p>
            </li>
          </ul>
        </CardContent>
      </Card>
    </SidebarContent>
  </SidebarFooter>
</template>

<style lang="css" scoped>
@keyframes moveRight {
  0% {
    transform: translateX(0);
  }
  50% {
    transform: translateX(-50px);
  }
  100% {
    transform: translateX(0);
  }
}

@keyframes moveUp {
  0% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-50px);
  }
  100% {
    transform: translateY(0);
  }
}

@keyframes moveDiagonal {
  0% {
    transform: translate(0, 0);
  }
  50% {
    transform: translate(-50px, 50px);
  }
  100% {
    transform: translate(0, 0);
  }
}

@keyframes moveLeft {
  0% {
    transform: translateX(0);
  }
  50% {
    transform: translateX(50px);
  }
  100% {
    transform: translateX(0);
  }
}

.animate-move-right {
  animation: moveRight 3s infinite;
}

.animate-move-up {
  animation: moveUp 3s infinite;
}

.animate-move-diagonal {
  animation: moveDiagonal 3s infinite;
}

.animate-move-left {
  animation: moveLeft 3s infinite;
}
</style>
