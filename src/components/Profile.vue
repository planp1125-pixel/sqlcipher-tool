<template>
  <div class="content-panel">
    <div class="content-header">
      <h2>Profile</h2>
      <p>Manage your account information and preferences</p>
    </div>
    
    <div class="profile-content">
      <!-- Account Information -->
      <div class="profile-section">
        <h3>Account Information</h3>
        <form @submit.prevent="saveProfile">
          <div class="form-row">
            <div class="form-group">
              <label>First Name</label>
              <input 
                type="text" 
                class="profile-input" 
                v-model="profile.firstName"
                required
              >
            </div>
            <div class="form-group">
              <label>Last Name</label>
              <input 
                type="text" 
                class="profile-input" 
                v-model="profile.lastName"
                required
              >
            </div>
          </div>
          
          <div class="form-group">
            <label>Email Address</label>
            <input 
              type="email" 
              class="profile-input" 
              v-model="profile.email"
              required
            >
            <small class="help-text">
              Used for login and important notifications
            </small>
          </div>
          
          <div class="form-group">
            <label>Company (Optional)</label>
            <input 
              type="text" 
              class="profile-input" 
              v-model="profile.company"
            >
          </div>
          
          <div class="form-group">
            <label>Job Title (Optional)</label>
            <input 
              type="text" 
              class="profile-input" 
              v-model="profile.jobTitle"
              placeholder="e.g., Database Administrator, Software Developer"
            >
          </div>
          
          <div class="form-actions">
            <button 
              type="submit" 
              class="btn btn-primary"
              :disabled="saving"
            >
              {{ saving ? 'Saving...' : 'Save Changes' }}
            </button>
          </div>
        </form>
      </div>

      <!-- Account Stats -->
      <div class="profile-section">
        <h3>Account Statistics</h3>
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-number">{{ accountStats.totalComparisons }}</div>
            <div class="stat-label">Total Comparisons</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ accountStats.scriptsGenerated }}</div>
            <div class="stat-label">Migration Scripts</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ accountStats.daysActive }}</div>
            <div class="stat-label">Days Active</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ accountStats.databasesConnected }}</div>
            <div class="stat-label">Databases Connected</div>
          </div>
        </div>
      </div>

      <!-- Security Settings -->
      <div class="profile-section">
        <h3>Security</h3>
        <div class="security-items">
          <div class="security-item">
            <div class="security-info">
              <h4>Password</h4>
              <p>Last changed {{ lastPasswordChange }}</p>
            </div>
            <button @click="changePassword" class="btn btn-outline">
              Change Password
            </button>
          </div>
          
          <div class="security-item">
            <div class="security-info">
              <h4>Two-Factor Authentication</h4>
              <p>{{ profile.twoFactorEnabled ? 'Enabled' : 'Not enabled' }}</p>
            </div>
            <button 
              @click="toggleTwoFactor" 
              class="btn"
              :class="profile.twoFactorEnabled ? 'btn-danger' : 'btn-primary'"
            >
              {{ profile.twoFactorEnabled ? 'Disable' : 'Enable' }} 2FA
            </button>
          </div>
          
          <div class="security-item">
            <div class="security-info">
              <h4>Login Sessions</h4>
              <p>{{ activeSessions }} active session(s)</p>
            </div>
            <button @click="viewSessions" class="btn btn-outline">
              Manage Sessions
            </button>
          </div>
        </div>
      </div>

      <!-- Data Management -->
      <div class="profile-section">
        <h3>Data Management</h3>
        <div class="data-actions">
          <div class="action-item">
            <div class="action-info">
              <h4>Export Account Data</h4>
              <p>Download all your comparison data and settings</p>
            </div>
            <button @click="exportData" class="btn btn-outline">
              Export Data
            </button>
          </div>
          
          <div class="action-item">
            <div class="action-info">
              <h4>Import Settings</h4>
              <p>Restore settings from a previous export</p>
            </div>
            <div class="import-section">
              <input 
                type="file" 
                ref="fileInput"
                @change="importData"
                accept=".json"
                style="display: none"
              >
              <button @click="fileInput?.click()" class="btn btn-outline">
                Import Settings
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Danger Zone -->
      <div class="profile-section danger-zone">
        <h3>Danger Zone</h3>
        <div class="danger-actions">
          <div class="action-item">
            <div class="action-info">
              <h4>Clear All Data</h4>
              <p>Remove all comparison history and reset settings</p>
            </div>
            <button @click="clearAllData" class="btn btn-danger">
              Clear All Data
            </button>
          </div>
          
          <div class="action-item">
            <div class="action-info">
              <h4>Delete Account</h4>
              <p>Permanently delete your account and all associated data</p>
            </div>
            <button @click="deleteAccount" class="btn btn-danger">
              Delete Account
            </button>
          </div>
        </div>
      </div>

      <!-- Save Status -->
      <div v-if="saveMessage" class="save-status" :class="saveStatus">
        {{ saveMessage }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';

const fileInput = ref<HTMLInputElement | null>(null);

interface Profile {
  firstName: string;
  lastName: string;
  email: string;
  company: string;
  jobTitle: string;
  twoFactorEnabled: boolean;
  memberSince: Date;
}

interface AccountStats {
  totalComparisons: number;
  scriptsGenerated: number;
  daysActive: number;
  databasesConnected: number;
}

const profile = reactive<Profile>({
  firstName: 'John',
  lastName: 'Doe',
  email: 'john.doe@example.com',
  company: 'Tech Corp',
  jobTitle: 'Database Administrator',
  twoFactorEnabled: false,
  memberSince: new Date('2023-06-15')
});

const accountStats = reactive<AccountStats>({
  totalComparisons: 147,
  scriptsGenerated: 23,
  daysActive: 45,
  databasesConnected: 8
});

const saving = ref(false);
const saveMessage = ref('');
const saveStatus = ref<'success' | 'error'>('success');
const activeSessions = ref(2);

const lastPasswordChange = computed(() => {
  // Simulate last password change
  const date = new Date();
  date.setDate(date.getDate() - 30);
  return date.toLocaleDateString();
});

const saveProfile = async () => {
  saving.value = true;
  saveMessage.value = '';

  try {
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Save to localStorage (in a real app, this would be an API call)
    localStorage.setItem('user-profile', JSON.stringify(profile));
    
    saveMessage.value = 'Profile updated successfully!';
    saveStatus.value = 'success';
    
    setTimeout(() => {
      saveMessage.value = '';
    }, 3000);
    
  } catch (error) {
    saveMessage.value = 'Failed to update profile. Please try again.';
    saveStatus.value = 'error';
    console.error('Failed to save profile:', error);
  } finally {
    saving.value = false;
  }
};

const changePassword = () => {
  const currentPassword = prompt('Enter your current password:');
  if (!currentPassword) return;
  
  const newPassword = prompt('Enter your new password:');
  if (!newPassword) return;
  
  const confirmPassword = prompt('Confirm your new password:');
  if (newPassword !== confirmPassword) {
    alert('Passwords do not match!');
    return;
  }
  
  if (newPassword.length < 8) {
    alert('Password must be at least 8 characters long!');
    return;
  }
  
  // Simulate password change
  alert('Password changed successfully!');
};

const toggleTwoFactor = () => {
  if (profile.twoFactorEnabled) {
    if (confirm('Are you sure you want to disable two-factor authentication?')) {
      profile.twoFactorEnabled = false;
      alert('Two-factor authentication disabled.');
    }
  } else {
    alert('Setting up 2FA...\n\nIn a real app, this would:\n1. Show QR code\n2. Ask for verification code\n3. Provide backup codes');
    profile.twoFactorEnabled = true;
  }
};

const viewSessions = () => {
  alert(`Active Sessions:\n\n1. Current session (This device)\n   - Chrome on Windows\n   - Last active: Now\n\n2. Mobile session\n   - Safari on iPhone\n   - Last active: 2 hours ago\n\nIn a real app, you could revoke sessions here.`);
};

const exportData = () => {
  const data = {
    profile: profile,
    stats: accountStats,
    settings: JSON.parse(localStorage.getItem('app-settings') || '{}'),
    exportDate: new Date().toISOString()
  };
  
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `sqlcipher-tool-data-${new Date().toISOString().slice(0, 10)}.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
  
  alert('Account data exported successfully!');
};

const importData = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  
  const reader = new FileReader();
  reader.onload = (e) => {
    try {
      const data = JSON.parse(e.target?.result as string);
      
      if (confirm('This will overwrite your current settings. Continue?')) {
        // Import profile data
        if (data.profile) {
          Object.assign(profile, data.profile);
        }
        
        // Import settings
        if (data.settings) {
          localStorage.setItem('app-settings', JSON.stringify(data.settings));
        }
        
        alert('Data imported successfully! Please refresh the app to see changes.');
      }
    } catch (error) {
      alert('Invalid file format. Please select a valid export file.');
    }
  };
  
  reader.readAsText(file);
};

const clearAllData = () => {
  if (confirm('This will permanently delete all your comparison data and reset all settings. This cannot be undone.\n\nType "DELETE" to confirm:') === false) {
    return;
  }
  
  const confirmation = prompt('Type "DELETE" to confirm:');
  if (confirmation === 'DELETE') {
    // Clear all localStorage
    const keysToKeep = ['user-profile']; // Keep profile but clear everything else
    Object.keys(localStorage).forEach(key => {
      if (!keysToKeep.includes(key)) {
        localStorage.removeItem(key);
      }
    });
    
    // Reset stats
    Object.assign(accountStats, {
      totalComparisons: 0,
      scriptsGenerated: 0,
      daysActive: 0,
      databasesConnected: 0
    });
    
    alert('All data cleared successfully!');
  }
};

const deleteAccount = () => {
  if (confirm('This will permanently delete your account and ALL associated data. This action cannot be undone.\n\nAre you absolutely sure?') === false) {
    return;
  }
  
  const email = prompt(`Type your email address (${profile.email}) to confirm account deletion:`);
  if (email === profile.email) {
    const finalConfirm = prompt('Type "DELETE MY ACCOUNT" to confirm:');
    if (finalConfirm === 'DELETE MY ACCOUNT') {
      alert('Account deletion request submitted.\n\nIn a real app, this would:\n1. Cancel subscriptions\n2. Queue account for deletion\n3. Send confirmation email\n4. Delete data after grace period');
    }
  } else {
    alert('Email address does not match. Account deletion cancelled.');
  }
};

onMounted(() => {
  // Load profile from localStorage
  try {
    const saved = localStorage.getItem('user-profile');
    if (saved) {
      const parsed = JSON.parse(saved);
      Object.assign(profile, parsed);
    }
  } catch (error) {
    console.warn('Failed to load profile:', error);
  }
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

.profile-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.profile-section {
  background: var(--white);
  padding: 1.5rem;
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid var(--gray-200);
}

.profile-section h3 {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  font-weight: 500;
  color: var(--gray-700);
  margin-bottom: 0.5rem;
}

.profile-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--gray-300);
  border-radius: 0.375rem;
  font-size: 0.875rem;
  transition: border-color 0.2s;
}

.profile-input:focus {
  outline: none;
  border-color: var(--primary-500);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}

.help-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.75rem;
  color: var(--gray-500);
}

.form-actions {
  margin-top: 1.5rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
}

.stat-card {
  text-align: center;
  padding: 1.5rem;
  background: var(--gray-50);
  border-radius: 0.5rem;
  border: 1px solid var(--gray-200);
}

.stat-number {
  font-size: 2rem;
  font-weight: 700;
  color: var(--primary-600);
  margin-bottom: 0.5rem;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--gray-600);
  font-weight: 500;
}

.security-items,
.data-actions,
.danger-actions {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.security-item,
.action-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--gray-50);
  border-radius: 0.5rem;
  border: 1px solid var(--gray-200);
}

.security-info h4,
.action-info h4 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 0.25rem;
}

.security-info p,
.action-info p {
  font-size: 0.875rem;
  color: var(--gray-600);
  margin: 0;
}

.danger-zone {
  border-left: 4px solid var(--error);
}

.danger-zone h3 {
  color: var(--error);
}

.import-section {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
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

.btn-outline {
  background: transparent;
  color: var(--gray-700);
  border: 1px solid var(--gray-300);
}

.btn-outline:hover {
  background: var(--gray-50);
  border-color: var(--gray-400);
}

.btn-danger {
  background: var(--error);
  color: var(--white);
}

.btn-danger:hover {
  background: #dc2626;
  transform: translateY(-1px);
}

.save-status {
  padding: 0.75rem;
  border-radius: 0.5rem;
  font-weight: 500;
  text-align: center;
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
  
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .security-item,
  .action-item {
    flex-direction: column;
    text-align: center;
    gap: 1rem;
  }
  
  .btn {
    width: 100%;
  }
}
</style>