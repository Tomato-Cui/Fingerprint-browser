<script setup lang="ts">
import type {
  ColumnFiltersState,
  ExpandedState,
  SortingState,
  VisibilityState,
} from "@tanstack/vue-table";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { cn, valueUpdater } from "@/util/lib";
import {
  createColumnHelper,
  FlexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useVueTable,
} from "@tanstack/vue-table";
import { ArrowUpDown, ChromeIcon, InboxIcon } from "lucide-vue-next";
import { h, ref, defineProps, withDefaults, onMounted } from "vue";
import { PrimaryButton } from "@/components/button/index";
import { MoreBtn } from "./more-btn";
import { browser_start, browser_stops } from "@/commands/browser";
import { toast } from "vue-sonner";
import { useBrowserStatusStore } from "@/stores/browser";

export interface Payment {
  id: number;
  name: string;
  uuid: string;
  user_uuid: string;
  description: string;
  country: string;
  group: string;
  proxy: string;
  open_urls: string;
  os: string;
}

interface TableProps {
  data: Payment[];
  pagination: any;
}

const props = withDefaults(defineProps<TableProps>(), {
  data: () => [] as Payment[],
  pagination: {
    pageIndex: 0,
    pageSize: 16,
    total: 0,
  },
});

const browserStatusStore = useBrowserStatusStore();
const emits = defineEmits(["onSelect", "onSyncColumns", "removeEnv", 'setCommonBtn', 'transferEnvBtn', 'authMemberBtn', 'editProxyBtn', 'editAccountBtn', 'editEnvBtn']);
const columnHelper = createColumnHelper<Payment>();
const columns = [
  columnHelper.display({
    id: "select",
    header: ({ table }) =>
      h(Checkbox, {
        checked:
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && "indeterminate"),
        "onUpdate:checked": (value) => {
          table.toggleAllPageRowsSelected(!!value);
          emits(
            "onSelect",
            table.getSelectedRowModel().rows.map((item) => item.getValue("id"))
          );
        },
        ariaLabel: "Select all",
        class: "mx-2",
      }),
    cell: ({ row }) => {
      return h(Checkbox, {
        checked: row.getIsSelected(),
        "onUpdate:checked": (value) => {
          row.toggleSelected(!!value);
          emits(
            "onSelect",
            table.getSelectedRowModel().rows.map((item) => item.getValue("uuid"))
          );
        },
        ariaLabel: "Select row",
        class: "mx-2",
      });
    },
    enableSorting: false,
    enableHiding: false,
    enablePinning: true,
  }),
  columnHelper.accessor("uuid", {
    header: () => h("div", { class: "hidden" }),
    cell: () => h("div", { class: "hidden" }),
  }),
  columnHelper.accessor("user_uuid", {
    header: () => h("div", { class: "hidden" }),
    cell: () => h("div", { class: "hidden" }),
  }),
  columnHelper.accessor("id", {
    header: () => h("div", { class: "hidden" }),
    cell: () => h("div", { class: "hidden" }),
  }),
  columnHelper.accessor("name", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["名称", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) =>
      h("div", { class: "lowercase whitespace-nowrap" }, row.getValue("name")),
  }),
  columnHelper.accessor("description", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["描述", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) =>
      h("div", { class: "lowercase" }, row.getValue("description") || "/"),
  }),
  columnHelper.accessor("os", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["平台", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) =>
      h("div", { class: "lowercase whitespace-nowrap" }, row.getValue("os") || "/"),
  }),
  columnHelper.accessor("proxy", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["代理", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) => h("div", { class: "lowercase" }, row.getValue("proxy") || "/"),
  }),
  columnHelper.accessor("group", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["分组", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) => h("div", { class: "lowercase" }, row.getValue("group") || "/"),
  }),
  columnHelper.accessor("open_urls", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["默认打开网页", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) =>
      h(
        "div",
        { class: "lowercase whitespace-nowrap" },
        row.getValue("open_urls") || "/"
      ),
  }),
  columnHelper.accessor("country", {
    header: ({ column }) => {
      return h(
        Button,
        {
          variant: "ghost",
          onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
          class: "px-0",
        },
        () => ["位置", h(ArrowUpDown, { class: "ml-2 h-4 w-4" })]
      );
    },
    cell: ({ row }) =>
      h("div", { class: "lowercase whitespace-nowrap" }, row.getValue("country") || "/"),
  }),
  columnHelper.display({
    id: "actions",
    enableHiding: false,
    enablePinning: true,
    header: () =>
      h("div", { class: "flex gap-x-4" }, [
        h("div", { class: "whitespace-nowrap w-32" }, "操作"),
        h("div", { class: "whitespace-nowrap px-2" }, "更多"),
      ]),
    cell: ({ row }) => {
      let uuid = row.getValue("uuid") as string;
      let id = row.getValue("id") as number;
      let user_uuid = row.getValue("user_uuid") as string;

      return h("div", { class: "flex gap-x-4" }, [
        h(
          "div",
          { class: "whitespace-nowrap flex justify-start" },
          h(
            PrimaryButton,
            {
              class: "px-2 flex gap-x-2 items-center text-md w-32",
              onClick: () => {
                if (!browserStatusStore.getStatus(uuid)) {
                  if (uuid) {
                    browser_start(uuid)
                      .then((res) => {
                        let data = res.data;
                        browserStatusStore.updateStatus(data.environment_uuid, data.status);
                      })
                      .catch((_) => toast.warning("启动失败"));
                  }
                } else {
                  browser_stops([uuid]).then((res: any) => {
                    if (res.message && (res.message as string).includes("关闭成功")) {
                      browserStatusStore.updateStatus(uuid, false);
                    }
                  });
                }
              },
            },
            [
              h(ChromeIcon, { class: "w-4 h-4" }),
              h(
                "span",
                { class: "text-sm" },
                !browserStatusStore.getStatus(uuid) ? "打开浏览器" : "关闭浏览器"
              ),
            ]
          )
        ),
        h("div", { class: "parent-container" }, [
          MoreBtn({
            'editEnvBtn': () => emits('editEnvBtn', uuid, id),
            'editAccountBtn': () => emits('editAccountBtn', uuid, id, user_uuid),
            'editProxyBtn': () => emits('editProxyBtn', uuid, id),
            'transferEnvBtn': () => emits('transferEnvBtn', uuid, id),
            'removeEnv': () => emits('removeEnv', uuid, id)
          }),
        ]), //传入点击事件参数
      ]);
    },
  }),
];

const sorting = ref<SortingState>([]);
const columnFilters = ref<ColumnFiltersState>([]);
const columnVisibility = ref<VisibilityState>({});
const rowSelection = ref({});
const expanded = ref<ExpandedState>({});

onMounted(() => {
  emits(
    "onSyncColumns",
    table.getAllColumns().filter((column) => column.getCanHide())
  );
});

const table = useVueTable({
  get data() {
    return props.data;
  },
  columns,
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),
  onSortingChange: (updaterOrValue) => valueUpdater(updaterOrValue, sorting),
  onColumnFiltersChange: (updaterOrValue) => valueUpdater(updaterOrValue, columnFilters),
  onColumnVisibilityChange: (updaterOrValue) =>
    valueUpdater(updaterOrValue, columnVisibility),
  onRowSelectionChange: (updaterOrValue) => valueUpdater(updaterOrValue, rowSelection),
  onExpandedChange: (updaterOrValue) => valueUpdater(updaterOrValue, expanded),
  manualPagination: true,
  state: {
    // pagination: props.pagination,
    get sorting() {
      return sorting.value;
    },
    get columnFilters() {
      return columnFilters.value;
    },
    get columnVisibility() {
      return columnVisibility.value;
    },
    get rowSelection() {
      return rowSelection.value;
    },
    get expanded() {
      return expanded.value;
    },
    columnPinning: {
      left: ["select"],
      right: ["actions"],
    },
  },
});
</script>

<template>
  <Table>
    <TableHeader class="z-10 sticky top-0">
      <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
        <TableHead v-for="header in headerGroup.headers" :key="header.id" :data-pinned="header.column.getIsPinned()"
          :class="cn(
            {
              'sticky bg-muted': header.column.getIsPinned(),
            },
            header.column.getIsPinned() === 'left' ? 'left-0' : 'right-0'
          )
            ">
          <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
            :props="header.getContext()" />
        </TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <template v-if="table.getRowModel().rows?.length">
        <template v-for="row in table.getRowModel().rows" :key="row.id">
          <!-- {{ row }} -->
          <TableRow :data-state="row.getIsSelected() && 'selected'" class="group">
            <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id" :data-pinned="cell.column.getIsPinned()"
              :class="cn(
                {
                  'sticky bg-background/95': cell.column.getIsPinned(),
                  'bg-muted': row.getIsSelected(),
                },
                cell.column.getIsPinned() === 'left' ? 'left-0' : 'right-0',
                'group-hover:bg-muted transition-colors'
              )
                ">
              <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
            </TableCell>
          </TableRow>
          <TableRow v-if="row.getIsExpanded()">
            <TableCell :colspan="row.getAllCells().length">
              {{ row.original }}
            </TableCell>
          </TableRow>
        </template>
      </template>

      <TableRow v-else>
        <TableCell :colspan="columns.length">
          <div class="flex justify-center items-center text-gray-400 gap-x-2">
            <InboxIcon class="h-4 w-4" />
            No results.
          </div>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
