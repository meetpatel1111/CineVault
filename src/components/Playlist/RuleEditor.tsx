import React, { useState, useEffect } from 'react';
import { Button } from '../Button';
import { Input } from '../Input';
import { Dropdown } from '../Dropdown';
import type { PlaylistRule } from '../../services/playlistService';
import './RuleEditor.css';

// We use a partial rule for editing before it's saved to DB
export interface EditableRule {
  id?: number;
  rule_type: string;
  operator: string;
  value: string;
}

interface RuleEditorProps {
  rules: EditableRule[];
  onChange: (rules: EditableRule[]) => void;
  readOnly?: boolean;
}

const FIELD_OPTIONS = [
  { id: 'media_type', label: 'Media Type' },
  { id: 'title', label: 'Title' },
  { id: 'year', label: 'Year' },
  { id: 'duration', label: 'Duration (sec)' },
];

const OPERATOR_OPTIONS = [
  { id: 'equals', label: 'Equals (=)' },
  { id: 'contains', label: 'Contains' },
  { id: 'starts_with', label: 'Starts With' },
  { id: 'ends_with', label: 'Ends With' },
  { id: 'gt', label: 'Greater Than (>)' },
  { id: 'lt', label: 'Less Than (<)' },
];

export const RuleEditor: React.FC<RuleEditorProps> = ({ rules, onChange, readOnly = false }) => {
  const handleAddRule = () => {
    onChange([...rules, { rule_type: 'title', operator: 'contains', value: '' }]);
  };

  const handleRemoveRule = (index: number) => {
    const newRules = [...rules];
    newRules.splice(index, 1);
    onChange(newRules);
  };

  const handleUpdateRule = (index: number, field: keyof EditableRule, value: string) => {
    const newRules = [...rules];
    newRules[index] = { ...newRules[index], [field]: value };
    onChange(newRules);
  };

  if (rules.length === 0 && readOnly) {
    return <div className="rule-editor-empty">No rules defined</div>;
  }

  return (
    <div className="rule-editor">
      {rules.map((rule, index) => (
        <div key={index} className="rule-row">
          {readOnly ? (
            <div className="rule-display">
              <span className="rule-field">{FIELD_OPTIONS.find(f => f.id === rule.rule_type)?.label || rule.rule_type}</span>
              <span className="rule-operator">{OPERATOR_OPTIONS.find(o => o.id === rule.operator)?.label || rule.operator}</span>
              <span className="rule-value">"{rule.value}"</span>
            </div>
          ) : (
            <>
              <div className="rule-input-group">
                <select
                  className="rule-select"
                  value={rule.rule_type}
                  onChange={(e) => handleUpdateRule(index, 'rule_type', e.target.value)}
                >
                  {FIELD_OPTIONS.map(opt => (
                    <option key={opt.id} value={opt.id}>{opt.label}</option>
                  ))}
                </select>

                <select
                  className="rule-select"
                  value={rule.operator}
                  onChange={(e) => handleUpdateRule(index, 'operator', e.target.value)}
                >
                  {OPERATOR_OPTIONS.map(opt => (
                    <option key={opt.id} value={opt.id}>{opt.label}</option>
                  ))}
                </select>

                <input
                  className="rule-input"
                  type="text"
                  value={rule.value}
                  onChange={(e) => handleUpdateRule(index, 'value', e.target.value)}
                  placeholder="Value"
                />
              </div>
              <Button
                variant="danger"
                size="sm"
                onClick={() => handleRemoveRule(index)}
                icon={<span>🗑️</span>}
              />
            </>
          )}
        </div>
      ))}

      {!readOnly && (
        <Button variant="secondary" size="sm" onClick={handleAddRule} icon={<span>➕</span>}>
          Add Rule
        </Button>
      )}
    </div>
  );
};
