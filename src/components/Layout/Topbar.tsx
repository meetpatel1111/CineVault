import React, { useState } from 'react';
import { Input } from '../Input';
import './Topbar.css';

interface TopbarProps {
  title?: string;
  onSearch?: (query: string) => void;
}

export const Topbar: React.FC<TopbarProps> = ({ 
  title = 'Home',
  onSearch 
}) => {
  const [searchQuery, setSearchQuery] = useState('');

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    const query = e.target.value;
    setSearchQuery(query);
    onSearch?.(query);
  };

  return (
    <header className="topbar">
      <div className="topbar__left">
        <h1 className="topbar__title">{title}</h1>
      </div>

      <div className="topbar__center">
        <div className="topbar__search">
          <Input
            type="search"
            placeholder="Search movies, TV shows, music..."
            value={searchQuery}
            onChange={handleSearch}
            icon={<SearchIcon />}
            iconPosition="left"
          />
        </div>
      </div>

      <div className="topbar__right">
        <button className="topbar__action" title="Scan Library">
          <span>🔄</span>
        </button>
        <button className="topbar__action" title="View Options">
          <span>⚙️</span>
        </button>
        <button className="topbar__action topbar__action--profile" title="Profile">
          <span>👤</span>
        </button>
      </div>
    </header>
  );
};

const SearchIcon: React.FC = () => (
  <svg
    width="20"
    height="20"
    viewBox="0 0 20 20"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M9 17A8 8 0 1 0 9 1a8 8 0 0 0 0 16zM19 19l-4.35-4.35"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    />
  </svg>
);
