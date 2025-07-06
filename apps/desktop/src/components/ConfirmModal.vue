<template>
  <div class="modal-backdrop" @click.self="cancel">
    <div class="modal-content">
      <div class="modal-message">{{ message }}</div>
      <div class="modal-actions">
        <button class="btn cancel" @click="cancel">
          <span>Cancel</span>
        </button>
        <button class="btn confirm" @click="confirm">
          <span>Delete</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  message: { type: String, default: 'Are you sure?' }
})

const emit = defineEmits(['confirm', 'cancel'])

const confirm = () => emit('confirm')
const cancel = () => emit('cancel')
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(10, 10, 10, 0.55);
  /* more dim */
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.modal-content {
  background-color: var(--card-bg, #fff);
  border-radius: 10px;
  padding: 2rem 2.5rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  min-width: 300px;
  max-width: 90%;
  text-align: center;
  color: var(--text-color);
  border: 1px solid var(--border-color, #e0e0e0);
  /* subtle border */
}

.modal-message {
  margin-bottom: 1.5rem;
  font-size: 1rem;
  font-weight: 500;
  opacity: 0.9;
}

.modal-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
}

/* Button styles */
.btn {
  background-color: transparent;
  color: var(--text-color);
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  font-weight: 400;
  font-family: Inter, sans-serif;
  border-radius: 8px;
  border: 1px solid var(--border-color, #ccc);
  transition: background-color 0.2s ease-in-out, color 0.2s ease-in-out;
  cursor: pointer;
}

.btn:hover {
  background-color: var(--btn-hover-bg, rgba(0, 0, 0, 0.05));
}

.btn i,
.btn span {
  font-size: 0.92rem;
  color: var(--text-color);
  opacity: 0.85;
}

.btn.confirm {
  color: #c0392b;
  border-color: #c0392b;
}

.btn.confirm:hover {
  background-color: rgba(192, 57, 43, 0.1);
}

.btn.cancel {
  color: var(--text-color);
}
</style>
