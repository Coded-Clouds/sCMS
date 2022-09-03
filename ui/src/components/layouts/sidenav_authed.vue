<template>
  <div class="relative">
    <nav class="absolute top-10 left-5 flex justify-center items-center h-[75vh]">
      <div class="space-y-6">
        <div class="w-64 p-4 border border-zinc-200 rounded-md shadow-lg">
          <div class="flex justify-between items-center border-b border-zinc-100">
            <h3 class="text-lg text-zinc-800 font-bold">Entities</h3>
            <span class="inline-flex items-center rounded-full bg-gray-100 px-2.5 py-0.5 text-xs font-bold text-gray-800">5</span>
          </div>
          <div class="mt-5 space-y-1" aria-label="Sidebar">
            <!-- Current: "bg-gray-100 text-gray-900", Default: "text-gray-600 hover:bg-gray-50 hover:text-gray-900" -->
            <a href="#" class="bg-gray-100 text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md" aria-current="page">
              <span class="truncate">Blog Posts</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Layouts</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Authors</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Pages</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Schemes</span>
            </a>
          </div>
        </div>

        <div class="w-64 p-4 border border-zinc-200 rounded-md shadow-lg">
          <div class="flex justify-between items-center border-b border-zinc-100">
            <h3 class="text-lg text-zinc-800 font-bold">Admin</h3>
          </div>
          <div class="mt-5 space-y-1" aria-label="Sidebar">
            <!-- Current: "bg-gray-100 text-gray-900", Default: "text-gray-600 hover:bg-gray-50 hover:text-gray-900" -->
            <a href="#" class="bg-gray-100 text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md" aria-current="page">
              <span class="truncate">Entity Manager</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Users</span>
            </a>

            <a href="#" class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 flex items-center px-3 py-2 text-sm font-medium rounded-md">
              <span class="truncate">Settings</span>
            </a>
          </div>
        </div>
      </div>
    </nav> 
  </div>
</template>
