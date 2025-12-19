import React from 'react';
import './Sidebar.css';

interface SidebarProps {
  activeSection?: string;
  onSectionChange?: (section: string) => void;
}

interface NavItem {
  id: string;
  label: string;
  icon: string;
}

const navItems: NavItem[] = [
  { id: 'home', label: 'Home', icon: '🏠' },
  { id: 'movies', label: 'Movies', icon: '🎬' },
  { id: 'tv', label: 'TV Shows', icon: '📺' },
  { id: 'music', label: 'Music', icon: '🎵' },
  { id: 'playlists', label: 'Playlists', icon: '📋' },
  { id: 'collections', label: 'Collections', icon: '📚' },
  { id: 'analytics', label: 'Analytics', icon: '📊' },
];

const libraryItems: NavItem[] = [
  { id: 'recent', label: 'Recently Added', icon: '🆕' },
  { id: 'watching', label: 'Continue Watching', icon: '▶️' },
  { id: 'unwatched', label: 'Unwatched', icon: '👁️' },
];

export const Sidebar: React.FC<SidebarProps> = ({ 
  activeSection = 'home', 
  onSectionChange 
}) => {
  return (
    <aside className="sidebar">
      <div className="sidebar__header">
        <h1 className="sidebar__logo">
          <span className="sidebar__logo-icon">🎬</span>
          <span className="sidebar__logo-text">CineVault</span>
        </h1>
      </div>

      <nav className="sidebar__nav">
        <div className="sidebar__section">
          <h2 className="sidebar__section-title">Library</h2>
          <ul className="sidebar__list">
            {navItems.map((item) => (
              <li key={item.id}>
                <button
                  className={`sidebar__item ${activeSection === item.id ? 'sidebar__item--active' : ''}`}
                  onClick={() => onSectionChange?.(item.id)}
                >
                  <span className="sidebar__item-icon">{item.icon}</span>
                  <span className="sidebar__item-label">{item.label}</span>
                </button>
              </li>
            ))}
          </ul>
        </div>

        <div className="sidebar__section">
          <h2 className="sidebar__section-title">Quick Access</h2>
          <ul className="sidebar__list">
            {libraryItems.map((item) => (
              <li key={item.id}>
                <button
                  className={`sidebar__item ${activeSection === item.id ? 'sidebar__item--active' : ''}`}
                  onClick={() => onSectionChange?.(item.id)}
                >
                  <span className="sidebar__item-icon">{item.icon}</span>
                  <span className="sidebar__item-label">{item.label}</span>
                </button>
              </li>
            ))}
          </ul>
        </div>
      </nav>

      <div className="sidebar__footer">
        <button 
          className="sidebar__item"
          onClick={() => onSectionChange?.('settings')}
        >
          <span className="sidebar__item-icon">⚙️</span>
          <span className="sidebar__item-label">Settings</span>
        </button>
      </div>
    </aside>
  );
};
