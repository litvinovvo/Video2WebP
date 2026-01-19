<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import FileDropZone from './components/FileDropZone.vue';
import SettingsPanel from './components/SettingsPanel.vue';
import ConversionQueue from './components/ConversionQueue.vue';
import type { ConversionSettings, ConversionResult, FileItem } from './types';
import { defaultSettings } from './types';

const files = ref<FileItem[]>([]);
const settings = ref<ConversionSettings>({ ...defaultSettings });
const isConverting = ref(false);
const ffmpegAvailable = ref(true);

onMounted(async () => {
  try {
    ffmpegAvailable.value = await invoke<boolean>('check_ffmpeg');
  } catch {
    ffmpegAvailable.value = false;
  }
});

function generateId(): string {
  return Math.random().toString(36).substring(2, 11);
}

function handleFilesAdded(paths: string[]) {
  const newFiles: FileItem[] = paths
    .filter(path => !files.value.some(f => f.path === path))
    .map(path => ({
      id: generateId(),
      name: path.split('/').pop() || path,
      path,
      status: 'pending' as const,
    }));
  files.value = [...files.value, ...newFiles];
}

async function selectFiles() {
  const selected = await open({
    multiple: true,
    filters: [{
      name: 'Video Files',
      extensions: ['mp4', 'MP4', 'mov', 'MOV', 'avi', 'AVI', 'mkv', 'MKV', 'webm', 'WEBM']
    }]
  });
  
  if (selected) {
    const paths = Array.isArray(selected) ? selected : [selected];
    handleFilesAdded(paths);
  }
}

async function startConversion(regenerate: boolean = false) {
  if (isConverting.value) return;
  
  if (regenerate) {
    // Reset all files to pending
    files.value.forEach(f => {
      f.status = 'pending';
      f.error = undefined;
      f.outputPath = undefined;
      f.outputSize = undefined;
    });
  }
  
  const pendingFiles = files.value.filter(f => f.status === 'pending');
  if (pendingFiles.length === 0) return;
  
  isConverting.value = true;
  
  for (const file of pendingFiles) {
    const index = files.value.findIndex(f => f.id === file.id);
    if (index === -1) continue;
    
    files.value[index].status = 'converting';
    
    try {
      const result = await invoke<ConversionResult>('convert_video', {
        inputPath: file.path,
        outputPath: null,
        settings: settings.value,
      });
      
      if (result.success) {
        files.value[index].status = 'done';
        files.value[index].outputPath = result.output_path;
        if (result.output_size) {
          files.value[index].outputSize = formatBytes(result.output_size);
        }
      } else {
        files.value[index].status = 'error';
        files.value[index].error = result.error || 'Unknown error';
      }
    } catch (e) {
      files.value[index].status = 'error';
      files.value[index].error = String(e);
    }
  }
  
  isConverting.value = false;
}

function removeFile(id: string) {
  files.value = files.value.filter(f => f.id !== id);
}

function clearCompleted() {
  files.value = files.value.filter(f => f.status !== 'done');
}

function clearAll() {
  files.value = [];
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function handleRerender(id: string) {
  const index = files.value.findIndex(f => f.id === id);
  if (index !== -1) {
    files.value[index].status = 'pending';
    files.value[index].error = undefined;
    files.value[index].outputPath = undefined;
    files.value[index].outputSize = undefined;
    startConversion();
  }
}
</script>

<template>
  <div class="app">
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 7l-7 5 7 5V7z"/>
            <rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
          </svg>
          <h1>Video to WebP</h1>
        </div>
        <span class="subtitle">Convert videos to animated WebP images</span>
      </div>
    </header>

    <main class="main">
      <div v-if="!ffmpegAvailable" class="warning-banner">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
          <line x1="12" y1="9" x2="12" y2="13"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
        <span>FFmpeg not found. Please install FFmpeg to use this application.</span>
      </div>

      <div class="content-grid">
        <div class="left-panel">
          <FileDropZone 
            @files-added="handleFilesAdded" 
            @browse-click="selectFiles"
          />
          
          <SettingsPanel v-model="settings" />
        </div>

        <div class="right-panel">
          <ConversionQueue 
            :files="files"
            :is-converting="isConverting"
            v-model:overwrite="settings.overwrite"
            @remove-file="removeFile"
            @rerender-file="handleRerender"
            @start-conversion="(regenerate) => startConversion(regenerate)"
            @clear-completed="clearCompleted"
            @clear-all="clearAll"
          />
        </div>
      </div>
    </main>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-primary: #0f0f14;
  --bg-secondary: #16161d;
  --bg-tertiary: #1c1c26;
  --bg-elevated: #22222e;
  --border-color: #2a2a3a;
  --text-primary: #f0f0f5;
  --text-secondary: #9090a0;
  --text-muted: #606070;
  --accent-primary: #7c5cff;
  --accent-secondary: #5c8cff;
  --accent-gradient: linear-gradient(135deg, #7c5cff 0%, #5c8cff 100%);
  --success: #4caf50;
  --error: #ff5c5c;
  --warning: #ffb74d;
  --shadow-sm: 0 2px 8px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 16px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 8px 32px rgba(0, 0, 0, 0.5);
  --radius-sm: 8px;
  --radius-md: 12px;
  --radius-lg: 16px;
  --transition: all 0.2s ease;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
  line-height: 1.5;
  overflow: hidden;
}

.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.header {
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  padding: 16px 24px;
  flex-shrink: 0;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo svg {
  width: 28px;
  height: 28px;
  color: var(--accent-primary);
}

.logo h1 {
  font-size: 1.25rem;
  font-weight: 600;
  background: var(--accent-gradient);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.main {
  flex: 1;
  padding: 24px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.warning-banner {
  display: flex;
  align-items: center;
  gap: 12px;
  background: rgba(255, 183, 77, 0.1);
  border: 1px solid rgba(255, 183, 77, 0.3);
  border-radius: var(--radius-md);
  padding: 12px 16px;
  color: var(--warning);
  font-size: 0.875rem;
  flex-shrink: 0;
}

.warning-banner svg {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.content-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  flex: 1;
  min-height: 0;
}

.left-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
}

.right-panel {
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}
</style>