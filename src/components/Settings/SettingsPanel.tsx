import React, { useState, useEffect } from 'react';
import { Modal } from '../Modal';
import { Button } from '../Button';
import { Input } from '../Input';
import { systemService, DependencyStatus } from '../../services/systemService';
import { useToast } from '../Toast';
import './SettingsPanel.css';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SettingsPanel: React.FC<SettingsPanelProps> = ({ isOpen, onClose }) => {
  const [activeTab, setActiveTab] = useState<'general' | 'library' | 'playback' | 'tmdb'>('general');

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Settings"
      size="lg"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={onClose}>Save Changes</Button>
        </>
      }
    >
      <div className="settings-panel">
        <div className="settings-panel__tabs">
          <button
            className={`settings-panel__tab ${activeTab === 'general' ? 'settings-panel__tab--active' : ''}`}
            onClick={() => setActiveTab('general')}
          >
            <span className="settings-panel__tab-icon">⚙️</span>
            General
          </button>
          <button
            className={`settings-panel__tab ${activeTab === 'library' ? 'settings-panel__tab--active' : ''}`}
            onClick={() => setActiveTab('library')}
          >
            <span className="settings-panel__tab-icon">📚</span>
            Library
          </button>
          <button
            className={`settings-panel__tab ${activeTab === 'playback' ? 'settings-panel__tab--active' : ''}`}
            onClick={() => setActiveTab('playback')}
          >
            <span className="settings-panel__tab-icon">▶️</span>
            Playback
          </button>
          <button
            className={`settings-panel__tab ${activeTab === 'tmdb' ? 'settings-panel__tab--active' : ''}`}
            onClick={() => setActiveTab('tmdb')}
          >
            <span className="settings-panel__tab-icon">🎬</span>
            TMDB
          </button>
        </div>

        <div className="settings-panel__content">
          {activeTab === 'general' && <GeneralSettings />}
          {activeTab === 'library' && <LibrarySettings />}
          {activeTab === 'playback' && <PlaybackSettings />}
          {activeTab === 'tmdb' && <TMDBSettings />}
        </div>
      </div>
    </Modal>
  );
};

const GeneralSettings: React.FC = () => {
  const [status, setStatus] = useState<DependencyStatus>({ ffmpeg: false, vlc: false });

  useEffect(() => {
    systemService.checkDependencies().then(setStatus).catch(console.error);
  }, []);

  return (
    <div className="settings-section">
      <h3 className="settings-section__title">General Settings</h3>

      <div className="settings-group">
        <label className="settings-label">Theme</label>
        <select className="settings-select">
          <option value="dark">Dark</option>
          <option value="light">Light</option>
          <option value="auto">Auto</option>
        </select>
        <p className="settings-help">Choose your preferred color theme</p>
      </div>

      <div className="settings-group">
        <label className="settings-label">System Health</label>
        <div className="settings-status-list">
          <div className="settings-status-item">
            <span className="settings-status-label">FFmpeg</span>
            <span className={`settings-status-value ${status.ffmpeg ? 'success' : 'error'}`}>
              {status.ffmpeg ? 'Detected' : 'Not Detected'}
            </span>
          </div>
          <div className="settings-status-item">
            <span className="settings-status-label">LibVLC</span>
            <span className={`settings-status-value ${status.vlc ? 'success' : 'error'}`}>
              {status.vlc ? 'Detected' : 'Not Detected'}
            </span>
          </div>
        </div>
        <p className="settings-help">FFmpeg is required for thumbnails. LibVLC is required for advanced playback.</p>
      </div>

      <div className="settings-group">
        <label className="settings-label">
          <input type="checkbox" defaultChecked />
          <span>Start on system startup</span>
        </label>
      </div>

      <div className="settings-group">
        <label className="settings-label">
          <input type="checkbox" defaultChecked />
          <span>Minimize to system tray</span>
        </label>
      </div>
    </div>
  );
};

const LibrarySettings: React.FC = () => {
  const { success, error, info } = useToast();
  const [extracting, setExtracting] = useState(false);

  const handleRefreshMetadata = async () => {
    if (confirm('This will rescan all files for metadata. It may take a while. Continue?')) {
      setExtracting(true);
      info('Starting metadata refresh...');
      try {
        const result = await systemService.extractAllMetadata();
        success(`Metadata updated. Processed: ${result.processed}, Updated: ${result.updated}, Errors: ${result.errors}`);
      } catch (err) {
        console.error(err);
        error('Failed to refresh metadata');
      } finally {
        setExtracting(false);
      }
    }
  };

  return (
    <div className="settings-section">
      <h3 className="settings-section__title">Library Settings</h3>

      <div className="settings-group">
        <label className="settings-label">Library Paths</label>
        <div className="settings-path-list">
          <div className="settings-path-item">
            <span>/Users/username/Movies</span>
            <button className="settings-path-remove">Remove</button>
          </div>
          <div className="settings-path-item">
            <span>/Users/username/TV Shows</span>
            <button className="settings-path-remove">Remove</button>
          </div>
        </div>
        <Button variant="secondary" size="sm">
          Add Folder
        </Button>
      </div>

      <div className="settings-group">
        <label className="settings-label">Metadata</label>
        <Button
          variant="secondary"
          onClick={handleRefreshMetadata}
          disabled={extracting}
          loading={extracting}
        >
          {extracting ? 'Refreshing...' : 'Refresh Metadata'}
        </Button>
        <p className="settings-help">Force re-extraction of metadata (thumbnails, duration, tracks) for all files.</p>
      </div>

      <div className="settings-group">
        <label className="settings-label">Auto-scan interval</label>
        <select className="settings-select">
          <option value="300">5 minutes</option>
          <option value="900">15 minutes</option>
          <option value="3600">1 hour</option>
          <option value="0">Disabled</option>
        </select>
      </div>

      <div className="settings-group">
        <label className="settings-label">
          <input type="checkbox" defaultChecked />
          <span>Watch for file changes</span>
        </label>
        <p className="settings-help">Automatically update library when files are added or removed</p>
      </div>
    </div>
  );
};

const PlaybackSettings: React.FC = () => (
  <div className="settings-section">
    <h3 className="settings-section__title">Playback Settings</h3>
    
    <div className="settings-group">
      <label className="settings-label">Player Engine</label>
      <select className="settings-select">
        <option value="html5">HTML5 (Default)</option>
        <option value="vlc">LibVLC (Advanced)</option>
      </select>
      <p className="settings-help">LibVLC supports more formats (MKV, AVI) but requires VLC installation.</p>
    </div>

    <div className="settings-group">
      <label className="settings-label">Default playback speed</label>
      <select className="settings-select">
        <option value="0.5">0.5x</option>
        <option value="0.75">0.75x</option>
        <option value="1.0" selected>1.0x</option>
        <option value="1.25">1.25x</option>
        <option value="1.5">1.5x</option>
        <option value="2.0">2.0x</option>
      </select>
    </div>

    <div className="settings-group">
      <label className="settings-label">
        <input type="checkbox" defaultChecked />
        <span>Auto-resume playback</span>
      </label>
      <p className="settings-help">Continue from where you left off</p>
    </div>

    <div className="settings-group">
      <label className="settings-label">
        <input type="checkbox" defaultChecked />
        <span>Mark as watched at 95%</span>
      </label>
      <p className="settings-help">Automatically mark items as watched when reaching 95% completion</p>
    </div>

    <div className="settings-group">
      <label className="settings-label">
        <input type="checkbox" defaultChecked />
        <span>Enable subtitles by default</span>
      </label>
    </div>

    <div className="settings-group">
      <label className="settings-label">Subtitle size</label>
      <select className="settings-select">
        <option value="small">Small</option>
        <option value="medium" selected>Medium</option>
        <option value="large">Large</option>
      </select>
    </div>
  </div>
);

const TMDBSettings: React.FC = () => (
  <div className="settings-section">
    <h3 className="settings-section__title">TMDB Integration</h3>
    
    <div className="settings-group">
      <label className="settings-label">
        <input type="checkbox" />
        <span>Enable TMDB metadata</span>
      </label>
      <p className="settings-help">Fetch movie and TV show information from The Movie Database</p>
    </div>

    <div className="settings-group">
      <Input
        label="TMDB API Key"
        type="password"
        placeholder="Enter your TMDB API key"
        helperText="Get your free API key from themoviedb.org"
        fullWidth
      />
    </div>

    <div className="settings-group">
      <label className="settings-label">Language</label>
      <select className="settings-select">
        <option value="en-US">English (US)</option>
        <option value="es-ES">Español</option>
        <option value="fr-FR">Français</option>
        <option value="de-DE">Deutsch</option>
        <option value="ja-JP">日本語</option>
      </select>
    </div>

    <div className="settings-group">
      <label className="settings-label">Image quality</label>
      <select className="settings-select">
        <option value="original">Original</option>
        <option value="w780">High (780px)</option>
        <option value="w500">Medium (500px)</option>
        <option value="w342">Low (342px)</option>
      </select>
    </div>

    <div className="settings-group">
      <label className="settings-label">
        <input type="checkbox" />
        <span>Auto-match new files</span>
      </label>
      <p className="settings-help">Automatically match new files with TMDB when scanning</p>
    </div>
  </div>
);
