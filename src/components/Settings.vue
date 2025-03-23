<template>
  <div class="settings">
    <button class="add-btn" @click="$router.push('/add')">
      <i class="bi bi-plus-circle"></i>
      <span>Add new term</span>
    </button>

    <button class="open-install-btn" @click="openAppLocalAppData">
      <i class="bi bi-folder"></i>
      <span>Open app resources</span>
    </button>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from '@tauri-apps/plugin-opener';

const openAppLocalAppData = async () => {
  const path = await invoke('get_app_localappdata');
  console.log(path);
  await revealItemInDir(`${path}/`);
};
</script>

<style scoped>
.settings {
  padding: 20px;
  color: white;
}

.add-btn,
.open-install-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  border: 1px solid #FF3214;
  background: transparent;
  color: #fff;
  padding: 8px 14px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
}

.add-btn:hover,
.open-install-btn:hover {
  background: #FF3214;
  color: #fff;
}

.add-btn:active,
.open-install-btn:active {
  background: #d72b10;
}
</style>
