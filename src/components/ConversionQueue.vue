<script setup lang="ts">
import { computed } from 'vue';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import type { FileItem } from '../types';

const props = defineProps<{
  files: FileItem[];
  isConverting: boolean;
  overwrite: boolean;
}>();

const emit = defineEmits<{
  (e: 'remove-file', id: string): void;
  (e: 'rerender-file', id: string): void;
  (e: 'start-conversion', regenerate?: boolean): void;
  (e: 'clear-completed'): void;
  (e: 'clear-all'): void;
  (e: 'update:overwrite', value: boolean): void;
}>();

const stats = computed(() => {
  const total = props.files.length;
  const pending = props.files.filter(f => f.status === 'pending').length;
  const done = props.files.filter(f => f.status === 'done').length;
  const errors = props.files.filter(f => f.status === 'error').length;
  return { total, pending, done, errors };
});

const canConvert = computed(() => 
  !props.isConverting && stats.value.pending > 0
);

async function openFileLocation(path: string) {
  try {
    await revealItemInDir(path);
  } catch (e) {
    console.error('Failed to open file location:', e);
  }
}

function getStatusIcon(status: FileItem['status']) {
  switch (status) {
    case 'pending':
      return `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/>
        <polyline points="12 6 12 12 16 14"/>
      </svg>`;
    case 'converting':
      return `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spin">
        <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
      </svg>`;
    case 'done':
      return `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>`;
    case 'error':
      return `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>`;
  }
}
</script>

<template>
  <div class="queue-panel">
    <div class="panel-header">
      <div class="header-left">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
          <line x1="16" y1="13" x2="8" y2="13"/>
          <line x1="16" y1="17" x2="8" y2="17"/>
        </svg>
        <h2>Conversion Queue</h2>
        <span class="file-count" v-if="stats.total > 0">{{ stats.total }}</span>
      </div>
      
      <div class="header-actions" v-if="files.length > 0">
        <button 
          class="action-btn text-only" 
          @click="emit('clear-completed')"
          v-if="stats.done > 0"
        >
          Clear Completed
        </button>
        <button 
          class="action-btn" 
          @click="emit('clear-all')"
          title="Clear All"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
          </svg>
          <span>Clear All</span>
        </button>
      </div>
    </div>
    
    <div class="queue-content">
      <div v-if="files.length === 0" class="empty-state">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        <span>Add files to get started</span>
      </div>
      
      <div v-else class="file-list">
        <div 
          v-for="file in files" 
          :key="file.id"
          class="file-item"
          :class="file.status"
        >
          <div class="file-status" v-html="getStatusIcon(file.status)"></div>
          
          <div class="file-info">
            <span class="file-name" :title="file.path">{{ file.name }}</span>
            <div class="file-meta">
              <span class="file-path" :title="file.path">{{ file.path }}</span>
              <span v-if="file.outputSize" class="file-size">{{ file.outputSize }}</span>
            </div>
            <span v-if="file.error" class="file-error">{{ file.error }}</span>
          </div>
          
          <div class="file-actions">
            <button 
              v-if="file.status === 'done' && file.outputPath"
              class="icon-btn success"
              @click="openFileLocation(file.outputPath)"
              title="Show in Finder"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
              </svg>
            </button>
            <button 
              v-if="file.status === 'done' || file.status === 'error'"
              class="icon-btn"
              @click="emit('rerender-file', file.id)"
              title="Rerender with current settings"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M23 4v6h-6"/>
                <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
              </svg>
            </button>
            <button 
              class="icon-btn"
              @click="emit('remove-file', file.id)"
              :disabled="file.status === 'converting'"
              title="Remove"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <div class="queue-footer" v-if="files.length > 0">
      <label class="overwrite-checkbox">
        <div class="checkbox-wrapper-sm">
          <input 
            type="checkbox" 
            :checked="overwrite"
            @change="emit('update:overwrite', ($event.target as HTMLInputElement).checked)"
          />
          <div class="checkbox-custom-sm">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          </div>
        </div>
        <span>Overwrite</span>
      </label>

      <div class="footer-right">
        <div class="stats">
          <span v-if="stats.pending > 0" class="stat pending">
            {{ stats.pending }} pending
          </span>
          <span v-if="stats.done > 0" class="stat done">
            {{ stats.done }} done
          </span>
          <span v-if="stats.errors > 0" class="stat error">
            {{ stats.errors }} failed
          </span>
        </div>
        
        <button 
          class="convert-btn"
          :disabled="!canConvert && !(stats.total > 0 && stats.pending === 0)"
          @click="emit('start-conversion', stats.pending === 0)"
        >
          <svg v-if="isConverting" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spin">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
          </svg>
          <svg v-else-if="stats.pending === 0 && stats.total > 0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 4v6h-6"/>
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="5 3 19 12 5 21 5 3"/>
          </svg>
          <span>
            {{ 
              isConverting 
                ? 'Converting...' 
                : stats.pending === 0 && stats.total > 0 
                  ? 'Regenerate All' 
                  : 'Convert' 
            }}
          </span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.queue-panel {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.header-left svg {
  width: 20px;
  height: 20px;
  color: var(--accent-primary);
}

.header-left h2 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.file-count {
  background: var(--accent-gradient);
  color: white;
  font-size: 0.75rem;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 10px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: 6px 12px;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  transition: var(--transition);
  font-family: inherit;
  display: flex;
  align-items: center;
  gap: 6px;
}

.action-btn svg {
  width: 14px;
  height: 14px;
}

.action-btn:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border-color: var(--text-muted);
}

.queue-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  min-height: 0;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--text-muted);
}

.empty-state svg {
  width: 48px;
  height: 48px;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 12px 16px;
  transition: var(--transition);
}

.file-item:hover {
  background: var(--bg-elevated);
}

.file-item.done {
  border-color: rgba(76, 175, 80, 0.3);
}

.file-item.error {
  border-color: rgba(255, 92, 92, 0.3);
}

.file-item.converting {
  border-color: rgba(124, 92, 255, 0.5);
  background: rgba(124, 92, 255, 0.05);
}

.file-status {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
}

.file-status :deep(svg) {
  width: 100%;
  height: 100%;
}

.pending .file-status {
  color: var(--text-muted);
}

.converting .file-status {
  color: var(--accent-primary);
}

.done .file-status {
  color: var(--success);
}

.error .file-status {
  color: var(--error);
}

.file-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.file-name {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-path {
  font-size: 0.75rem;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.file-size {
  font-size: 0.75rem;
  color: var(--accent-secondary);
  background: rgba(92, 140, 255, 0.1);
  padding: 1px 6px;
  border-radius: 4px;
  flex-shrink: 0;
  white-space: nowrap;
}

.file-error {
  font-size: 0.75rem;
  color: var(--error);
}

.file-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.icon-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: var(--text-muted);
  transition: var(--transition);
}

.icon-btn:hover:not(:disabled) {
  background: var(--bg-elevated);
  color: var(--text-primary);
}

.icon-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.icon-btn.success:hover {
  color: var(--success);
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

.queue-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}

.stats {
  display: flex;
  gap: 16px;
}

.stat {
  font-size: 0.75rem;
  font-weight: 500;
}

.stat.pending {
  color: var(--text-secondary);
}

.stat.done {
  color: var(--success);
}

.stat.error {
  color: var(--error);
}

.convert-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--accent-gradient);
  border: none;
  border-radius: var(--radius-md);
  padding: 10px 20px;
  font-size: 0.875rem;
  font-weight: 600;
  color: white;
  cursor: pointer;
  transition: var(--transition);
  font-family: inherit;
  box-shadow: 0 4px 16px rgba(124, 92, 255, 0.3);
}

.convert-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(124, 92, 255, 0.4);
}

.convert-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.convert-btn svg {
  width: 18px;
  height: 18px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.spin, :deep(.spin) {
  animation: spin 1s linear infinite;
}

.footer-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.overwrite-checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.overwrite-checkbox:hover {
  color: var(--text-primary);
}

.checkbox-wrapper-sm {
  position: relative;
  width: 18px;
  height: 18px;
}

.checkbox-wrapper-sm input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
  height: 0;
  width: 0;
}

.checkbox-custom-sm {
  position: absolute;
  top: 0;
  left: 0;
  width: 18px;
  height: 18px;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: var(--transition);
}

.checkbox-wrapper-sm input:checked ~ .checkbox-custom-sm {
  background-color: var(--accent-primary);
  border-color: var(--accent-primary);
}

.checkbox-custom-sm svg {
  width: 12px;
  height: 12px;
  color: white;
  display: none;
}

.checkbox-wrapper-sm input:checked ~ .checkbox-custom-sm svg {
  display: block;
}
</style>
