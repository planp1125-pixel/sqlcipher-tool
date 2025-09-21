<template>
  <div class="content-panel">
    <div class="content-header">
      <h2>Settings</h2>
      <p>Configure your application preferences</p>
    </div>
    
    <div class="settings-content">
      <!-- Database Settings -->
      <div class="setting-group">
        <h3>Database Settings</h3>
        <div class="setting-item">
          <label>Default timeout (seconds)</label>
          <input 
            type="number" 
            class="setting-input" 
            v-model="settings.timeout"
            min="5"
            max="300"
          >
        </div>
        <div class="setting-item">
          <label>Auto-save comparisons</label>
          <input 
            type="checkbox" 
            v-model="settings.autoSave"
          >
        </div>
        <div class="setting-item">
          <label>Max concurrent connections</label>
          <input 
            type="number" 
            class="setting-input" 
            v-model="settings.maxConnections"
            min="1"
            max="10"
          >
        </div>
      </div>
      
      <!-- Display Settings -->
      <div class="setting-group">
        <h3>Display Settings</h3>
        <div class="setting-item">
          <label>Theme</label>
          <select class="setting-input" v-model="settings.theme">
            <option value="light">Light</option>
            <option value="dark">Dark</option>
            <option value="system">System</option>
          </select>
        </div>
        <div class="setting-item">
          <label>Font size</label>
          <select class="setting-input" v-model="settings.fontSize">
            <option value="small">Small</option>
            <option value="medium">Medium</option>
            <option value="large">Large</option>
          </select>
        </div>
        <div class="setting-item">
          <label>Show row numbers in tables</label>
          <input 
            type="checkbox" 
            v-model="settings.showRowNumbers"
          >
        </div>
      </div>

      <!-- Export Settings -->
      <div class="setting-group">
        <h3>Export Settings</h3>
        <div class="setting-item">
          <label>Default export format</label>
          <select class="setting-input" v-model="settings.exportFormat">
            <option value="json">JSON</option>
            <option value="csv">CSV</option>
            <option value="sql">SQL</option>
            <option value="txt">Text</option>
          </select>
        </div>
        <div class="setting-item">
          <label>Include metadata in exports</label>
          <input 
            type="checkbox" 
            v-model="settings.includeMetadata"
          >
        </div>
      </div>

      <!-- Notification Settings -->
      <div class="setting-group">
        <h3>Notifications</h3>
        <div class="setting-item">
          <label>Show completion notifications</label>
          <input 
            type="checkbox" 
            v-model="settings.showNotifications"
          >
        </div>
        <div class="setting-item">
          <label>Sound alerts</label>
          <input 
            type="checkbox" 
            v-model="settings.soundAlerts"
            :disabled="!settings.showNotifications"
          >
        </div>
      </div>

      <!-- Save Button -->
      <div class="setting-actions">
        <button 
          @click="saveSettings" 
          class="btn btn-primary"
          :disabled="saving"
        >
          {{ saving ? 'Saving...' : 'Save Settings' }}
        </button>
        <button 
          @click="resetToDefaults" 
          class="btn btn-secondary"
        >
          Reset to Defaults
        </button>
      </div>

      <!-- Save Status -->
      <div v-if="saveMessage" class="save-status" :class="saveStatus">
        {{ saveMessage }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

interface SettingsData {
  timeout: number;
  autoSave: boolean;
  maxConnections: number;
  theme: 'light' | 'dark' | 'system';
  fontSize: 'small' | 'medium' | 'large';
  showRowNumbers: boolean;
  exportFormat: 'json' | 'csv' | 'sql' | 'txt';
  includeMetadata: boolean;
  showNotifications: boolean;
  soundAlerts: boolean;
}

const defaultSettings: SettingsData = {
  timeout: 30,
  autoSave: true,
  maxConnections: 5,
  theme: 'system',
  fontSize: 'medium',
  showRowNumbers: true,
  exportFormat: 'json',
  includeMetadata: true,
  showNotifications: true,
  soundAlerts: false
};

const settings = reactive<SettingsData>({ ...defaultSettings });
const saving = ref(false);
const saveMessage = ref('');
const saveStatus = ref<'success' | 'error'>('success');

const loadSettings = () => {
  try {
    const saved = localStorage.getItem('app-settings');
    if (saved) {
      const parsed = JSON.parse(saved);
      Object.assign(settings, { ...defaultSettings, ...parsed });
    }
  } catch (error) {
    console.warn('Failed to load settings:', error);
  }
};

const saveSettings = async () => {
  saving.value = true;
  saveMessage.value = '';

  try {
    // Simulate async operation
    await new Promise(resolve => setTimeout(resolve, 500));
    
    localStorage.setItem('app-settings', JSON.stringify(settings));
    
    saveMessage.value = 'Settings saved successfully!';
    saveStatus.value = 'success';
    
    // Clear message after 3 seconds
    setTimeout(() => {
      saveMessage.value = '';
    }, 3000);
    
  } catch (error) {
    saveMessage.value = 'Failed to save settings. Please try again.';
    saveStatus.value = 'error';
    console.error('Failed to save settings:', error);
  } finally {
    saving.value = false;
  }
};

const resetToDefaults = () => {
  if (confirm('Are you sure you want to reset all settings to defaults?')) {
    Object.assign(settings, defaultSettings);
    saveMessage.value = 'Settings reset to defaults';
    saveStatus.value = 'success';
    
    setTimeout(() => {
      saveMessage.value = '';
    }, 3000);
  }
};

onMounted(() => {
  loadSettings();
});
</script>

<style scoped>
.content-panel {
  height: 100vh; /* Use viewport height instead */
  overflow-y: auto;
  padding: 2rem;
  background: var(--gray-50);
  box-sizing: border-box;
}

.content-header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--gray-200);
}

.content-header h2 {
  font-size: 1.875rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 0.5rem;
}

.content-header p {
  color: var(--gray-600);
  font-size: 1rem;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.setting-group {
  background: var(--white);
  padding: 1.5rem;
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid var(--gray-200);
}

.setting-group h3 {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 1rem;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
  padding: 0.5rem 0;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-item label {
  font-weight: 500;
  color: var(--gray-700);
  flex: 1;
}

.setting-input {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--gray-300);
  border-radius: 0.375rem;
  font-size: 0.875rem;
  min-width: 150px;
  transition: border-color 0.2s;
}

.setting-input:focus {
  outline: none;
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.setting-input:disabled {
  background: var(--gray-100);
  color: var(--gray-500);
  cursor: not-allowed;
}

input[type="checkbox"] {
  width: 1.25rem;
  height: 1.25rem;
  cursor: pointer;
}

.setting-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-primary {
  background: var(--primary-500);
  color: var(--white);
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-600);
  transform: translateY(-1px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.btn-primary:disabled {
  background: var(--gray-400);
  cursor: not-allowed;
  transform: none;
}

.btn-secondary {
  background: var(--gray-200);
  color: var(--gray-800);
}

.btn-secondary:hover {
  background: var(--gray-300);
  transform: translateY(-1px);
}

.save-status {
  padding: 0.75rem;
  border-radius: 0.5rem;
  font-weight: 500;
  text-align: center;
  margin-top: 1rem;
}

.save-status.success {
  background: #f0fdf4;
  color: #166534;
  border: 1px solid #bbf7d0;
}

.save-status.error {
  background: #fef2f2;
  color: #991b1b;
  border: 1px solid #fecaca;
}

@media (max-width: 768px) {
  .content-panel {
    padding: 1rem;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .setting-input {
    width: 100%;
    min-width: auto;
  }
  
  .setting-actions {
    flex-direction: column;
  }
}
</style>