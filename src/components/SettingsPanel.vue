<script setup lang="ts">
import type { ConversionSettings } from '../types';

const model = defineModel<ConversionSettings>({ required: true });
</script>

<template>
  <div class="settings-panel">
    <div class="panel-header">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
      <h2>Conversion Settings</h2>
    </div>
    
    <div class="settings-grid">
      <div class="setting-item">
        <label for="fps">
          <span class="label-text">FPS</span>
          <span class="label-hint">Frames per second</span>
        </label>
        <div class="input-group">
          <input 
            id="fps"
            type="number" 
            v-model.number="model.fps"
            min="1" 
            max="60"
          />
          <span class="input-suffix">fps</span>
        </div>
      </div>
      
      <div class="setting-item">
        <label for="max_dimension">
          <span class="label-text">Max Dimension</span>
          <span class="label-hint">Longest side of output</span>
        </label>
        <div class="input-group">
          <input 
            id="max_dimension"
            type="number" 
            v-model.number="model.max_dimension"
            min="100" 
            max="3840"
            step="10"
          />
          <span class="input-suffix">px</span>
        </div>
      </div>
      
      <div class="setting-item">
        <label for="max_frames">
          <span class="label-text">Max Frames</span>
          <span class="label-hint">Maximum frames to extract</span>
        </label>
        <div class="input-group">
          <input 
            id="max_frames"
            type="number" 
            v-model.number="model.max_frames"
            min="1" 
            max="500"
          />
          <span class="input-suffix">frames</span>
        </div>
      </div>
      
      <div class="setting-item">
        <label for="quality">
          <span class="label-text">Quality</span>
          <span class="label-hint">WebP quality (0-100)</span>
        </label>
        <div class="input-group range-group">
          <input 
            id="quality"
            type="range" 
            v-model.number="model.quality"
            min="0" 
            max="100"
          />
          <span class="range-value">{{ model.quality }}</span>
        </div>
      </div>
      
      <div class="setting-item full-width">
        <label for="compression">
          <span class="label-text">Compression Level</span>
          <span class="label-hint">Higher = smaller file, slower encoding</span>
        </label>
        <div class="input-group range-group">
          <input 
            id="compression"
            type="range" 
            v-model.number="model.compression_level"
            min="0" 
            max="6"
          />
          <span class="range-value">{{ model.compression_level }}</span>
        </div>
      </div>


    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 20px;
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.panel-header svg {
  width: 20px;
  height: 20px;
  color: var(--accent-primary);
}

.panel-header h2 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.settings-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  overflow-y: auto;
  flex: 1;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-item.full-width {
  grid-column: span 2;
}

.setting-item label {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.label-text {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.label-hint {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.input-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.input-group input[type="number"] {
  flex: 1;
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
  font-size: 0.875rem;
  color: var(--text-primary);
  font-family: inherit;
  outline: none;
  transition: var(--transition);
}

.input-group input[type="number"]:focus {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(124, 92, 255, 0.2);
}

.input-suffix {
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 40px;
}

.range-group {
  gap: 12px;
}

.range-group input[type="range"] {
  flex: 1;
  -webkit-appearance: none;
  appearance: none;
  height: 6px;
  background: var(--bg-elevated);
  border-radius: 3px;
  outline: none;
}

.range-group input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: var(--accent-gradient);
  border-radius: 50%;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(124, 92, 255, 0.4);
  transition: transform 0.15s ease;
}

.range-group input[type="range"]::-webkit-slider-thumb:hover {
  transform: scale(1.1);
}

.range-value {
  min-width: 32px;
  text-align: right;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--accent-primary);
}

/* Hide number input spinners */
input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type="number"] {
  -moz-appearance: textfield;
}

.checkbox-item {
  margin-top: 4px;
}

.checkbox-label {
  flex-direction: row !important;
  align-items: center;
  gap: 12px !important;
  cursor: pointer;
}

.checkbox-wrapper {
  position: relative;
  width: 20px;
  height: 20px;
}

.checkbox-wrapper input {
  position: absolute;
  opacity: 0;
  width: 100%;
  height: 100%;
  cursor: pointer;
  z-index: 2;
}

.checkbox-custom {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: var(--transition);
}

.checkbox-wrapper input:checked + .checkbox-custom {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
}

.checkbox-custom svg {
  width: 14px;
  height: 14px;
  color: white;
  opacity: 0;
  transition: var(--transition);
  transform: scale(0.5);
}

.checkbox-wrapper input:checked + .checkbox-custom svg {
  opacity: 1;
  transform: scale(1);
}

.label-content {
  display: flex;
  flex-direction: column;
}
</style>
