<template>
  <div class="settings container">
    <!-- Open Install Folder Section -->
    <div class="setting-group">
      <h5>Open install folder</h5>
      <button class="btn open-btn" @click="openAppLocalAppData">
        <i class="bi bi-folder"></i>
        <span>Open</span>
      </button>
    </div>

    <!-- Add & Import Section -->
    <div class="setting-group">
      <h5>Add & Import</h5>
      <div class="btn-group">
        <button class="btn import-btn" @click="$router.push('/import')">
          <i class="bi bi-file-earmark-arrow-up"></i>
          <span>Import</span>
        </button>

        <button class="btn add-btn" @click="$router.push('/add')">
          <i class="bi bi-plus-circle"></i>
          <span>Add manually</span>
        </button>
      </div>
    </div>
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
  color: #333;
}

.setting-group {
  margin-bottom: 24px;
}

h5 {
  font-size: 0.95rem;
  color: #adadad;
  margin-bottom: 5px;
  font-weight: 500;
}

.btn {
  background-color: transparent;
  color: #adadad;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-weight: 400;
  border-radius: 8px;
  transition: all 0.2s ease-in-out;
  color: #333;
  border: 1px solid #363636;
}

.btn span {
  font-size: 0.92rem;
  color: #adadad;
  margin-left: 2px;
}

.btn i {
  font-size: 1.1rem;
  color: #adadad;
}

.btn:hover {
  background-color: #2b2b2b;
  /* color: #d6d6d6; */
}

.btn-group {
  display: flex;
  gap: 10px;
}
</style>
