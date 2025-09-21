<template>
  <div class="popup-overlay" @click="$emit('close')">
    <div class="popup-content" @click.stop>
      <div class="popup-header">
        <h2>Detailed Comparison: {{ tableResult.tableName }}</h2>
        <div class="popup-actions">
          <button @click="exportTableDifferences" class="export-btn">
            Export Report
          </button>
          <button @click="$emit('close')" class="close-btn">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
      </div>

      <div class="popup-summary">
        <div class="summary-grid">
          <div class="summary-item">
            <span class="summary-label">Source Rows:</span>
            <span class="summary-value">{{ tableResult.sourceData.length }}</span>
          </div>
          <div class="summary-item">
            <span class="summary-label">Target Rows:</span>
            <span class="summary-value">{{ tableResult.targetData.length }}</span>
          </div>
          <div class="summary-item identical">
            <span class="summary-label">Identical:</span>
            <span class="summary-value">{{ tableResult.summary.identicalRows }}</span>
          </div>
          <div class="summary-item different">
            <span class="summary-label">Different:</span>
            <span class="summary-value">{{ tableResult.summary.differentRows }}</span>
          </div>
          <div class="summary-item missing">
            <span class="summary-label">Missing in Target:</span>
            <span class="summary-value">{{ tableResult.summary.missingInTarget }}</span>
          </div>
          <div class="summary-item extra">
            <span class="summary-label">Extra in Target:</span>
            <span class="summary-value">{{ tableResult.summary.extraInTarget }}</span>
          </div>
        </div>
      </div>

      <div class="popup-controls">
        <div class="view-toggle">
          <button 
            @click="currentViewMode = 'side-by-side'"
            :class="{ active: currentViewMode === 'side-by-side' }"
          >
            Side by Side
          </button>
          <button 
            @click="currentViewMode = 'unified'"
            :class="{ active: currentViewMode === 'unified' }"
          >
            Unified View
          </button>
        </div>
        
        <div class="filter-options">
          <label>
            <input type="checkbox" v-model="showIdentical"> 
            Show identical rows
          </label>
          <label>
            <input type="checkbox" v-model="showDifferent"> 
            Show different rows
          </label>
          <label>
            <input type="checkbox" v-model="showMissing"> 
            Show missing rows
          </label>
        </div>
      </div>

      <div class="popup-data">
        <!-- Side by Side View -->
        <div v-if="currentViewMode === 'side-by-side'" class="popup-side-by-side">
          <div class="popup-data-column">
            <div class="popup-column-header">
              <h4>{{ sourceDatabaseName }}</h4>
              <span class="row-count">{{ tableResult.sourceData.length }} rows</span>
            </div>
            <div class="popup-table-container">
              <table class="popup-data-table">
                <thead>
                  <tr>
                    <th v-for="(column, index) in tableResult.comparison.columns" :key="'src_' + column">
                      {{ index + 1 }}. {{ column }}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr 
                    v-for="(row, index) in filteredSourceData" 
                    :key="'popup_src_row_' + index"
                    :class="getRowClass(row, 'source')"
                  >
                    <td 
                      v-for="column in tableResult.comparison.columns" 
                      :key="'popup_src_' + column"
                      :class="getCellClass(row, column, 'source')"
                    >
                      {{ formatCellValue(row[column]) }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <div class="popup-data-column">
            <div class="popup-column-header">
              <h4>{{ targetDatabaseName }}</h4>
              <span class="row-count">{{ tableResult.targetData.length }} rows</span>
            </div>
            <div class="popup-table-container">
              <table class="popup-data-table">
                <thead>
                  <tr>
                    <th v-for="(column, index) in tableResult.comparison.columns" :key="'tgt_' + column">
                      {{ index + 1 }}. {{ column }}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr 
                    v-for="(row, index) in filteredTargetData" 
                    :key="'popup_tgt_row_' + index"
                    :class="getRowClass(row, 'target')"
                  >
                    <td 
                      v-for="column in tableResult.comparison.columns" 
                      :key="'popup_tgt_' + column"
                      :class="getCellClass(row, column, 'target')"
                    >
                      {{ formatCellValue(row[column]) }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- Unified View -->
        <div v-else class="popup-unified">
          <div class="popup-table-container">
            <table class="popup-unified-table">
              <thead>
                <tr>
                  <th class="status-col">Status</th>
                  <th v-for="(column, index) in tableResult.comparison.columns" :key="'popup_uni_' + column">
                    {{ index + 1 }}. {{ column }}
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr 
                  v-for="(item, index) in unifiedData" 
                  :key="'popup_uni_row_' + index"
                  :class="'row-' + item.status"
                >
                  <td class="status-col">
                    <span :class="'status-badge status-' + item.status">
                      {{ getStatusLabel(item.status) }}
                    </span>
                  </td>
                  <td 
                    v-for="column in tableResult.comparison.columns" 
                    :key="'popup_uni_' + column"
                    :class="{ 'cell-different': item.differentColumns?.includes(column) }"
                  >
                    <div v-if="item.status === 'different' && item.differentColumns?.includes(column)" class="cell-diff">
                      <div class="source-value">{{ formatCellValue(item.sourceRow?.[column]) }}</div>
                      <div class="arrow">→</div>
                      <div class="target-value">{{ formatCellValue(item.targetRow?.[column]) }}</div>
                    </div>
                    <div v-else>
                      {{ formatCellValue(item.sourceRow?.[column] || item.targetRow?.[column]) }}
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

interface DataRow {
  [key: string]: any;
}

interface ComparisonResult {
  columns: string[];
  totalRows: number;
  differentRows: any[];
  missingInTarget: DataRow[];
  extraInTarget: DataRow[];
  identicalRows: DataRow[];
}

interface UnifiedDataItem {
  status: 'identical' | 'different' | 'missing' | 'extra';
  sourceRow?: DataRow;
  targetRow?: DataRow;
  differentColumns?: string[];
}

interface TableComparisonResult {
  tableName: string;
  sourceData: DataRow[];
  targetData: DataRow[];
  comparison: ComparisonResult;
  summary: {
    identicalRows: number;
    differentRows: number;
    missingInTarget: number;
    extraInTarget: number;
  };
}

interface Props {
  tableResult: TableComparisonResult;
  sourceDatabaseName: string;
  targetDatabaseName: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  close: [];
}>();

// Local state
const currentViewMode = ref<'side-by-side' | 'unified'>('side-by-side');
const showIdentical = ref(true);
const showDifferent = ref(true);
const showMissing = ref(true);

// Computed data filters
const filteredSourceData = computed(() => {
  if (!showDifferent.value) return props.tableResult.sourceData;
  
  return props.tableResult.sourceData.filter(row => {
    return isRowDifferent(row, 'source');
  });
});

const filteredTargetData = computed(() => {
  if (!showDifferent.value) return props.tableResult.targetData;
  
  return props.tableResult.targetData.filter(row => {
    return isRowDifferent(row, 'target');
  });
});

const unifiedData = computed((): UnifiedDataItem[] => {
  const unified: UnifiedDataItem[] = [];
  
  // Add identical rows
  if (showIdentical.value) {
    props.tableResult.comparison.identicalRows.forEach(row => {
      unified.push({
        status: 'identical',
        sourceRow: row,
        targetRow: row
      });
    });
  }
  
  // Add different rows
  if (showDifferent.value) {
    props.tableResult.comparison.differentRows.forEach(diffItem => {
      unified.push({
        status: 'different',
        sourceRow: diffItem.sourceRow,
        targetRow: diffItem.targetRow,
        differentColumns: diffItem.differentColumns
      });
    });
  }
  
  // Add missing rows
  if (showMissing.value) {
    props.tableResult.comparison.missingInTarget.forEach(row => {
      unified.push({
        status: 'missing',
        sourceRow: row
      });
    });
    
    props.tableResult.comparison.extraInTarget.forEach(row => {
      unified.push({
        status: 'extra',
        targetRow: row
      });
    });
  }
  
  return unified;
});

// Helper functions
const createRowKey = (row: DataRow, columns: string[]): string => {
  return columns.map(col => {
    let value = row[col];
    return String(value);
  }).join('|');
};

const isRowDifferent = (row: DataRow, type: 'source' | 'target'): boolean => {
  const key = createRowKey(row, props.tableResult.comparison.columns);
  
  return props.tableResult.comparison.differentRows.some(diff => {
    const compareRow = type === 'source' ? diff.sourceRow : diff.targetRow;
    const compareKey = createRowKey(compareRow, props.tableResult.comparison.columns);
    return key === compareKey;
  });
};

const getRowClass = (row: DataRow, type: 'source' | 'target'): string => {
  const key = createRowKey(row, props.tableResult.comparison.columns);
  
  if (props.tableResult.comparison.differentRows.some(diff => {
    const compareRow = type === 'source' ? diff.sourceRow : diff.targetRow;
    return createRowKey(compareRow, props.tableResult.comparison.columns) === key;
  })) {
    return 'row-different';
  }
  
  if ((type === 'source' && props.tableResult.comparison.missingInTarget.some(missing => 
    createRowKey(missing, props.tableResult.comparison.columns) === key
  )) || (type === 'target' && props.tableResult.comparison.extraInTarget.some(extra => 
    createRowKey(extra, props.tableResult.comparison.columns) === key
  ))) {
    return 'row-missing';
  }
  
  return 'row-identical';
};

const getCellClass = (row: DataRow, column: string, type: 'source' | 'target'): string => {
  const diffRow = props.tableResult.comparison.differentRows.find(diff => {
    const compareRow = type === 'source' ? diff.sourceRow : diff.targetRow;
    return createRowKey(compareRow, props.tableResult.comparison.columns) === 
           createRowKey(row, props.tableResult.comparison.columns);
  });
  
  if (diffRow && diffRow.differentColumns.includes(column)) {
    return 'cell-different';
  }
  
  return '';
};

const formatCellValue = (value: any): string => {
  if (value === null || value === undefined) return 'NULL';
  if (typeof value === 'string' && value === '') return '(empty)';
  return String(value);
};

const getStatusLabel = (status: string): string => {
  switch (status) {
    case 'identical': return 'Same';
    case 'different': return 'Different';
    case 'missing': return 'Missing';
    case 'extra': return 'Extra';
    default: return status;
  }
};

const exportTableDifferences = () => {
  const report = generateTableReport();
  downloadReport(report, `${props.tableResult.tableName}-comparison-report.txt`);
};

const generateTableReport = (): string => {
  let report = `Table Comparison Report: ${props.tableResult.tableName}\n`;
  report += `${'='.repeat(50)}\n\n`;
  report += `Source Database: ${props.sourceDatabaseName}\n`;
  report += `Target Database: ${props.targetDatabaseName}\n`;
  report += `Generated: ${new Date().toLocaleString()}\n\n`;
  
  report += `Summary:\n`;
  report += `- Total Rows (Source): ${props.tableResult.sourceData.length}\n`;
  report += `- Total Rows (Target): ${props.tableResult.targetData.length}\n`;
  report += `- Identical Rows: ${props.tableResult.summary.identicalRows}\n`;
  report += `- Different Rows: ${props.tableResult.summary.differentRows}\n`;
  report += `- Missing in Target: ${props.tableResult.summary.missingInTarget}\n`;
  report += `- Extra in Target: ${props.tableResult.summary.extraInTarget}\n\n`;
  
  if (props.tableResult.comparison.differentRows.length > 0) {
    report += `Different Rows (showing all ${props.tableResult.comparison.differentRows.length} differences):\n`;
    report += `${'-'.repeat(50)}\n`;
    props.tableResult.comparison.differentRows.forEach((diff, index) => {
      report += `Row ${index + 1}:\n`;
      diff.differentColumns.forEach((col: string) => {
        report += `  ${col}: "${diff.sourceRow[col]}" → "${diff.targetRow[col]}"\n`;
      });
      report += `\n`;
    });
  }
  
  return report;
};

const downloadReport = (content: string, filename: string) => {
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};

const handleEscapeKey = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    emit('close');
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleEscapeKey);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleEscapeKey);
});
</script>

<style scoped>
.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 2rem;
}

.popup-content {
  background: var(--white);
  border-radius: 0.75rem;
  width: 95vw;
  height: 90vh;
  max-width: 1400px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  overflow: hidden;
}

.popup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--gray-200);
  background: var(--gray-50);
}

.popup-header h2 {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--gray-900);
  margin: 0;
}

.popup-actions {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.export-btn {
  padding: 0.75rem 1.5rem;
  background: var(--blue-500);
  color: var(--white);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.export-btn:hover {
  background: var(--blue-600);
  transform: translateY(-1px);
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  background: var(--gray-200);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.close-btn:hover {
  background: var(--gray-300);
}

.popup-summary {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--gray-200);
  background: var(--white);
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.summary-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  border-radius: 0.5rem;
  background: var(--gray-50);
}

.summary-item.identical {
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
}

.summary-item.different {
  background: #fffbeb;
  border: 1px solid #fed7aa;
}

.summary-item.missing {
  background: #fef2f2;
  border: 1px solid #fecaca;
}

.summary-item.extra {
  background: #f3f4f6;
  border: 1px solid #d1d5db;
}

.summary-label {
  font-weight: 500;
  color: var(--gray-700);
  font-size: 0.875rem;
}

.summary-value {
  font-weight: 600;
  color: var(--gray-900);
  font-size: 1rem;
}

.popup-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--gray-200);
  background: var(--white);
}

.view-toggle {
  display: flex;
  border: 1px solid var(--gray-300);
  border-radius: 0.5rem;
  overflow: hidden;
}

.view-toggle button {
  padding: 0.5rem 1rem;
  border: none;
  background: var(--white);
  color: var(--gray-700);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.view-toggle button:hover {
  background: var(--gray-50);
}

.view-toggle button.active {
  background: var(--primary-500);
  color: var(--white);
}

.filter-options {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.filter-options label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--gray-700);
  cursor: pointer;
}

.popup-data {
  flex: 1;
  overflow: hidden;
  background: var(--white);
}

.popup-side-by-side {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1px;
  height: 100%;
  background: var(--gray-200);
}

.popup-data-column {
  background: var(--white);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.popup-column-header {
  padding: 1rem;
  background: var(--gray-100);
  border-bottom: 1px solid var(--gray-200);
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.popup-column-header h4 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--gray-900);
  margin: 0;
}

.row-count {
  font-size: 0.75rem;
  color: var(--gray-600);
}

.popup-table-container {
  flex: 1;
  overflow: auto;
}

.popup-data-table,
.popup-unified-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
  table-layout: fixed;
}

.popup-data-table th,
.popup-data-table td,
.popup-unified-table th,
.popup-unified-table td {
  padding: 0.75rem;
  text-align: left;
  border-bottom: 1px solid var(--gray-100);
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.popup-data-table th,
.popup-unified-table th {
  background: var(--gray-50);
  font-weight: 600;
  color: var(--gray-700);
  position: sticky;
  top: 0;
  z-index: 10;
}

.popup-unified {
  height: 100%;
  overflow: hidden;
}

.popup-unified .popup-table-container {
  height: 100%;
}

.status-col {
  width: 80px;
}

.status-badge {
  padding: 0.25rem 0.5rem;
  border-radius: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
}

.status-identical {
  background: #dcfce7;
  color: #166534;
}

.status-different {
  background: #fef3c7;
  color: #92400e;
}

.status-missing {
  background: #fee2e2;
  color: #991b1b;
}

.status-extra {
  background: #ddd6fe;
  color: #5b21b6;
}

.row-different {
  background: #fef3c7;
}

.row-missing {
  background: #fee2e2;
}

.row-identical {
  background: var(--white);
}

.cell-different {
  background: #fed7aa;
  font-weight: 500;
}

.cell-diff {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.75rem;
}

.source-value {
  color: #dc2626;
  text-decoration: line-through;
}

.target-value {
  color: #059669;
  font-weight: 500;
}

.arrow {
  color: var(--gray-500);
}
</style>