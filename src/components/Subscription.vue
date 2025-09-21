<template>
  <div class="content-panel">
    <div class="content-header">
      <h2>Subscription</h2>
      <p>Manage your subscription and billing</p>
    </div>
    
    <div class="subscription-content">
      <!-- Current Plan Card -->
      <div class="plan-card current-plan">
        <div class="plan-header">
          <h3>Current Plan: {{ currentPlan.name }}</h3>
          <span class="plan-badge" :class="currentPlan.type">{{ currentPlan.name }}</span>
        </div>
        
        <div class="plan-details">
          <div class="usage-stats">
            <div class="stat-item">
              <span class="stat-label">Comparisons this month</span>
              <span class="stat-value">{{ usage.comparisons }}/{{ currentPlan.maxComparisons === -1 ? '∞' : currentPlan.maxComparisons }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Migration scripts generated</span>
              <span class="stat-value">{{ usage.scripts }}/{{ currentPlan.maxScripts === -1 ? '∞' : currentPlan.maxScripts }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Next billing date</span>
              <span class="stat-value">{{ currentPlan.type === 'free' ? 'N/A' : nextBillingDate }}</span>
            </div>
          </div>
          
          <div class="progress-bars" v-if="currentPlan.type !== 'pro'">
            <div class="progress-item">
              <label>Comparisons Usage</label>
              <div class="progress-bar">
                <div 
                  class="progress-fill" 
                  :style="{ width: `${(usage.comparisons / currentPlan.maxComparisons) * 100}%` }"
                ></div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="plan-actions" v-if="currentPlan.type === 'free'">
          <button @click="upgradeToPro" class="btn btn-primary">
            Upgrade to Pro - $19/month
          </button>
        </div>
      </div>

      <!-- Pricing Plans -->
      <div class="pricing-section">
        <h4>Choose Your Plan</h4>
        <div class="pricing-grid">
          <!-- Free Plan -->
          <div class="pricing-card" :class="{ active: currentPlan.type === 'free' }">
            <div class="pricing-header">
              <h5>Free</h5>
              <div class="price">$0<span>/month</span></div>
            </div>
            
            <ul class="features-list">
              <li>5 comparisons per month</li>
              <li>Basic schema visualization</li>
              <li>Text export only</li>
              <li>Community support</li>
            </ul>
            
            <button 
              class="btn btn-outline" 
              :disabled="currentPlan.type === 'free'"
            >
              {{ currentPlan.type === 'free' ? 'Current Plan' : 'Downgrade' }}
            </button>
          </div>

          <!-- Pro Plan -->
          <div class="pricing-card pro-plan" :class="{ active: currentPlan.type === 'pro' }">
            <div class="pricing-header">
              <h5>Pro</h5>
              <div class="price">$19<span>/month</span></div>
              <div class="popular-badge">Most Popular</div>
            </div>
            
            <ul class="features-list">
              <li>✅ Unlimited comparisons</li>
              <li>✅ Migration script generation</li>
              <li>✅ Advanced export formats (SQL, JSON, CSV)</li>
              <li>✅ Batch comparisons</li>
              <li>✅ Priority email support</li>
              <li>✅ Schema versioning</li>
            </ul>
            
            <button 
              @click="currentPlan.type === 'pro' ? manageBilling() : upgradeToPro()" 
              class="btn"
              :class="currentPlan.type === 'pro' ? 'btn-secondary' : 'btn-primary'"
            >
              {{ currentPlan.type === 'pro' ? 'Manage Billing' : 'Upgrade to Pro' }}
            </button>
          </div>

          <!-- Enterprise Plan -->
          <div class="pricing-card">
            <div class="pricing-header">
              <h5>Enterprise</h5>
              <div class="price">Custom<span>/month</span></div>
            </div>
            
            <ul class="features-list">
              <li>✅ Everything in Pro</li>
              <li>✅ Team collaboration</li>
              <li>✅ API access</li>
              <li>✅ Custom integrations</li>
              <li>✅ Dedicated support</li>
              <li>✅ SLA guarantee</li>
            </ul>
            
            <button @click="contactSales" class="btn btn-outline">
              Contact Sales
            </button>
          </div>
        </div>
      </div>

      <!-- Billing History -->
      <div class="billing-section" v-if="currentPlan.type === 'pro'">
        <h4>Billing History</h4>
        <div class="billing-card">
          <div class="billing-table">
            <div class="table-header">
              <span>Date</span>
              <span>Description</span>
              <span>Amount</span>
              <span>Status</span>
              <span>Invoice</span>
            </div>
            
            <div v-for="bill in billingHistory" :key="bill.id" class="table-row">
              <span>{{ formatDate(bill.date) }}</span>
              <span>{{ bill.description }}</span>
              <span>${{ bill.amount }}</span>
              <span class="status" :class="bill.status.toLowerCase()">{{ bill.status }}</span>
              <button @click="downloadInvoice(bill.id)" class="btn-link">Download</button>
            </div>
          </div>
        </div>
      </div>

      <!-- FAQ Section -->
      <div class="faq-section">
        <h4>Frequently Asked Questions</h4>
        <div class="faq-list">
          <div class="faq-item" v-for="faq in faqs" :key="faq.id">
            <div class="faq-question" @click="toggleFaq(faq.id)">
              <span>{{ faq.question }}</span>
              <span class="faq-icon">{{ expandedFaqs.has(faq.id) ? '−' : '+' }}</span>
            </div>
            <div v-if="expandedFaqs.has(faq.id)" class="faq-answer">
              {{ faq.answer }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue';

interface Plan {
  name: string;
  type: 'free' | 'pro' | 'enterprise';
  maxComparisons: number;
  maxScripts: number;
}

interface Usage {
  comparisons: number;
  scripts: number;
}

interface BillingRecord {
  id: string;
  date: Date;
  description: string;
  amount: number;
  status: 'Paid' | 'Pending' | 'Failed';
}

interface FAQ {
  id: number;
  question: string;
  answer: string;
}

const currentPlan = reactive<Plan>({
  name: 'Free',
  type: 'free',
  maxComparisons: 5,
  maxScripts: 0
});

const usage = reactive<Usage>({
  comparisons: 2,
  scripts: 0
});

const expandedFaqs = ref<Set<number>>(new Set());

const nextBillingDate = computed(() => {
  const date = new Date();
  date.setMonth(date.getMonth() + 1);
  return date.toLocaleDateString();
});

const billingHistory: BillingRecord[] = [
  {
    id: 'inv_001',
    date: new Date('2024-01-15'),
    description: 'SQLCipher Tool Pro - Monthly',
    amount: 19.00,
    status: 'Paid'
  },
  {
    id: 'inv_002',
    date: new Date('2023-12-15'),
    description: 'SQLCipher Tool Pro - Monthly',
    amount: 19.00,
    status: 'Paid'
  }
];

const faqs: FAQ[] = [
  {
    id: 1,
    question: 'Can I cancel my subscription at any time?',
    answer: 'Yes, you can cancel your subscription at any time. You will continue to have access to Pro features until the end of your billing period.'
  },
  {
    id: 2,
    question: 'What happens to my data if I downgrade?',
    answer: 'Your data is always safe. If you downgrade, you will lose access to Pro features but your comparison history and settings will be preserved.'
  },
  {
    id: 3,
    question: 'Do you offer refunds?',
    answer: 'We offer a 30-day money-back guarantee for all new subscriptions. Contact support for refund requests.'
  },
  {
    id: 4,
    question: 'Is there a discount for annual plans?',
    answer: 'Yes! Annual plans receive a 20% discount. Contact us to switch to annual billing.'
  }
];

const upgradeToPro = () => {
  // Simulate upgrade process
  alert('Redirecting to payment processor...\n\nThis would open Stripe/PayPal in a real app.');
  
  // For demo purposes, simulate successful upgrade
  setTimeout(() => {
    currentPlan.name = 'Pro';
    currentPlan.type = 'pro';
    currentPlan.maxComparisons = -1;
    currentPlan.maxScripts = -1;
    alert('Upgrade successful! Welcome to Pro!');
  }, 2000);
};

const manageBilling = () => {
  alert('Opening billing portal...\n\nThis would open Stripe customer portal in a real app.');
};

const contactSales = () => {
  alert('Opening contact form...\n\nThis would open a contact form or redirect to sales email.');
};

const downloadInvoice = (invoiceId: string) => {
  alert(`Downloading invoice ${invoiceId}...\n\nThis would download the PDF invoice in a real app.`);
};

const formatDate = (date: Date): string => {
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  });
};

const toggleFaq = (id: number) => {
  const newSet = new Set(expandedFaqs.value);
  if (newSet.has(id)) {
    newSet.delete(id);
  } else {
    newSet.add(id);
  }
  expandedFaqs.value = newSet;
};
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

.subscription-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.plan-card {
  background: var(--white);
  padding: 2rem;
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid var(--gray-200);
}

.current-plan {
  border-left: 4px solid var(--primary-500);
}

.plan-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.plan-header h3 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--gray-900);
}

.plan-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
}

.plan-badge.free {
  background: var(--gray-100);
  color: var(--gray-700);
}

.plan-badge.pro {
  background: var(--primary-100);
  color: var(--primary-700);
}

.usage-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 0.75rem;
  background: var(--gray-50);
  border-radius: 0.5rem;
}

.stat-label {
  color: var(--gray-600);
  font-size: 0.875rem;
}

.stat-value {
  font-weight: 600;
  color: var(--gray-900);
}

.progress-bars {
  margin-bottom: 1.5rem;
}

.progress-item {
  margin-bottom: 1rem;
}

.progress-item label {
  display: block;
  font-size: 0.875rem;
  color: var(--gray-700);
  margin-bottom: 0.5rem;
}

.progress-bar {
  width: 100%;
  height: 0.5rem;
  background: var(--gray-200);
  border-radius: 0.25rem;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary-500);
  transition: width 0.3s ease;
}

.pricing-section h4,
.billing-section h4,
.faq-section h4 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 1.5rem;
}

.pricing-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
}

.pricing-card {
  background: var(--white);
  padding: 2rem;
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 2px solid var(--gray-200);
  position: relative;
  transition: all 0.2s;
}

.pricing-card:hover {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.pricing-card.active {
  border-color: var(--primary-500);
}

.pro-plan {
  border-color: var(--primary-500);
  transform: scale(1.05);
}

.pricing-header {
  text-align: center;
  margin-bottom: 2rem;
  position: relative;
}

.pricing-header h5 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--gray-900);
  margin-bottom: 0.5rem;
}

.price {
  font-size: 2rem;
  font-weight: 700;
  color: var(--primary-600);
}

.price span {
  font-size: 1rem;
  color: var(--gray-500);
}

.popular-badge {
  position: absolute;
  top: -0.5rem;
  right: -0.5rem;
  background: var(--primary-500);
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
}

.features-list {
  list-style: none;
  padding: 0;
  margin: 0 0 2rem 0;
}

.features-list li {
  padding: 0.5rem 0;
  color: var(--gray-700);
  border-bottom: 1px solid var(--gray-100);
}

.features-list li:last-child {
  border-bottom: none;
}

.billing-card {
  background: var(--white);
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid var(--gray-200);
  overflow: hidden;
}

.billing-table {
  display: grid;
  grid-template-columns: 1fr 2fr 1fr 1fr 1fr;
  gap: 1rem;
}

.table-header {
  display: contents;
  font-weight: 600;
  color: var(--gray-900);
  background: var(--gray-50);
}

.table-header > span {
  padding: 1rem;
  background: var(--gray-50);
  border-bottom: 1px solid var(--gray-200);
}

.table-row {
  display: contents;
}

.table-row > span,
.table-row > button {
  padding: 1rem;
  border-bottom: 1px solid var(--gray-100);
  display: flex;
  align-items: center;
}

.status.paid {
  color: var(--success);
}

.status.pending {
  color: var(--warning);
}

.status.failed {
  color: var(--error);
}

.faq-list {
  background: var(--white);
  border-radius: 0.75rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border: 1px solid var(--gray-200);
  overflow: hidden;
}

.faq-item {
  border-bottom: 1px solid var(--gray-100);
}

.faq-item:last-child {
  border-bottom: none;
}

.faq-question {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  cursor: pointer;
  font-weight: 500;
  color: var(--gray-900);
  transition: background-color 0.2s;
}

.faq-question:hover {
  background: var(--gray-50);
}

.faq-icon {
  font-size: 1.25rem;
  color: var(--gray-500);
  font-weight: normal;
}

.faq-answer {
  padding: 0 1.5rem 1rem 1.5rem;
  color: var(--gray-700);
  line-height: 1.6;
  background: var(--gray-50);
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  width: 100%;
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

.btn-secondary {
  background: var(--gray-200);
  color: var(--gray-800);
}

.btn-secondary:hover {
  background: var(--gray-300);
  transform: translateY(-1px);
}

.btn-outline {
  background: transparent;
  color: var(--gray-700);
  border: 1px solid var(--gray-300);
}

.btn-outline:hover:not(:disabled) {
  background: var(--gray-50);
}

.btn:disabled {
  background: var(--gray-300);
  color: var(--gray-500);
  cursor: not-allowed;
  transform: none;
}

.btn-link {
  background: none;
  border: none;
  color: var(--primary-600);
  cursor: pointer;
  font-size: 0.875rem;
  padding: 0;
}

.btn-link:hover {
  text-decoration: underline;
}

@media (max-width: 768px) {
  .content-panel {
    padding: 1rem;
  }
  
  .usage-stats {
    grid-template-columns: 1fr;
  }
  
  .pricing-grid {
    grid-template-columns: 1fr;
  }
  
  .pro-plan {
    transform: none;
  }
  
  .billing-table {
    display: block;
  }
  
  .table-header {
    display: none;
  }
  
  .table-row {
    display: block;
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--gray-200);
    border-radius: 0.5rem;
  }
  
  .table-row > span,
  .table-row > button {
    display: block;
    padding: 0.25rem 0;
    border-bottom: none;
  }
  
  .table-row > span:before {
    content: attr(data-label);
    font-weight: 600;
    display: inline-block;
    width: 100px;
  }
}
</style>