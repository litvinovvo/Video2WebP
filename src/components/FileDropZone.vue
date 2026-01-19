<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'files-added', paths: string[]): void;
  (e: 'browse-click'): void;
}>();

const isDragOver = ref(false);

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragOver.value = true;
}

function handleDragLeave() {
  isDragOver.value = false;
}

function handleDrop(event: DragEvent) {
  event.preventDefault();
  isDragOver.value = false;
  
  const items = event.dataTransfer?.files;
  if (!items) return;
  
  const paths: string[] = [];
  for (let i = 0; i < items.length; i++) {
    const file = items[i];
    if (file.type.startsWith('video/') || /\.(mp4|mov|avi|mkv|webm)$/i.test(file.name)) {
      // In Tauri, we need to use the file path
      // @ts-ignore - path exists in Tauri file objects
      if (file.path) {
        // @ts-ignore
        paths.push(file.path);
      }
    }
  }
  
  if (paths.length > 0) {
    emit('files-added', paths);
  }
}

function handleClick() {
  emit('browse-click');
}
</script>

<template>
  <div 
    class="drop-zone"
    :class="{ 'drag-over': isDragOver }"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
    @click="handleClick"
  >
    <div class="drop-content">
      <div class="drop-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="17 8 12 3 7 8"/>
          <line x1="12" y1="3" x2="12" y2="15"/>
        </svg>
      </div>
      <div class="drop-text">
        <span class="drop-title">Drop video files here</span>
        <span class="drop-subtitle">or click to browse</span>
      </div>
      <div class="drop-formats">
        Supports MP4, MOV, AVI, MKV, WebM
      </div>
    </div>
    
    <div class="drop-overlay">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M12 5v14M5 12h14"/>
      </svg>
      <span>Drop files to add</span>
    </div>
  </div>
</template>

<style scoped>
.drop-zone {
  position: relative;
  background: var(--bg-secondary);
  border: 2px dashed var(--border-color);
  border-radius: var(--radius-lg);
  padding: 32px;
  cursor: pointer;
  transition: var(--transition);
  overflow: hidden;
  flex-shrink: 0;
}

.drop-zone:hover {
  border-color: var(--accent-primary);
  background: var(--bg-tertiary);
}

.drop-zone.drag-over {
  border-color: var(--accent-primary);
  background: rgba(124, 92, 255, 0.1);
  border-style: solid;
}

.drop-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  transition: var(--transition);
}

.drag-over .drop-content {
  opacity: 0;
  transform: scale(0.95);
}

.drop-icon {
  width: 56px;
  height: 56px;
  background: var(--accent-gradient);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 20px rgba(124, 92, 255, 0.3);
}

.drop-icon svg {
  width: 28px;
  height: 28px;
  color: white;
}

.drop-text {
  text-align: center;
}

.drop-title {
  display: block;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.drop-subtitle {
  display: block;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.drop-formats {
  font-size: 0.75rem;
  color: var(--text-muted);
  background: var(--bg-elevated);
  padding: 6px 12px;
  border-radius: var(--radius-sm);
}

.drop-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  background: rgba(124, 92, 255, 0.15);
  opacity: 0;
  pointer-events: none;
  transition: var(--transition);
}

.drag-over .drop-overlay {
  opacity: 1;
}

.drop-overlay svg {
  width: 48px;
  height: 48px;
  color: var(--accent-primary);
}

.drop-overlay span {
  font-size: 1rem;
  font-weight: 600;
  color: var(--accent-primary);
}
</style>
