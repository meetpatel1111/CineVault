import React, { useState } from 'react';
import { FilterCriteria } from '../services/mediaService';

interface FilterPanelProps {
  onFilter: (criteria: FilterCriteria) => void;
  onClear: () => void;
  className?: string;
}

const FilterPanel: React.FC<FilterPanelProps> = ({ onFilter, onClear, className }) => {
  const [minYear, setMinYear] = useState<string>('');
  const [maxYear, setMaxYear] = useState<string>('');
  const [minDuration, setMinDuration] = useState<string>('');
  const [maxDuration, setMaxDuration] = useState<string>('');
  const [resolutions, setResolutions] = useState<string[]>([]);
  const [mediaTypes, setMediaTypes] = useState<string[]>([]);

  const resolutionOptions = [
    { value: '4k', label: '4K / UHD' },
    { value: '1080p', label: '1080p / Full HD' },
    { value: '720p', label: '720p / HD' },
    { value: 'sd', label: 'SD / 480p' },
  ];

  const mediaTypeOptions = [
    { value: 'Movie', label: 'Movies' },
    { value: 'TvEpisode', label: 'TV Episodes' },
    { value: 'Music', label: 'Music' },
  ];

  const handleApply = () => {
    const criteria: FilterCriteria = {};

    if (minYear) criteria.min_year = parseInt(minYear, 10);
    if (maxYear) criteria.max_year = parseInt(maxYear, 10);
    if (minDuration) criteria.min_duration = parseInt(minDuration, 10) * 60; // Convert minutes to seconds
    if (maxDuration) criteria.max_duration = parseInt(maxDuration, 10) * 60;

    if (resolutions.length > 0) criteria.resolutions = resolutions;
    if (mediaTypes.length > 0) criteria.media_types = mediaTypes;

    onFilter(criteria);
  };

  const handleReset = () => {
    setMinYear('');
    setMaxYear('');
    setMinDuration('');
    setMaxDuration('');
    setResolutions([]);
    setMediaTypes([]);
    onClear();
  };

  const toggleResolution = (res: string) => {
    setResolutions(prev =>
      prev.includes(res) ? prev.filter(r => r !== res) : [...prev, res]
    );
  };

  const toggleMediaType = (type: string) => {
    setMediaTypes(prev =>
      prev.includes(type) ? prev.filter(t => t !== type) : [...prev, type]
    );
  };

  return (
    <div className={`bg-gray-800 p-4 rounded-lg shadow-lg ${className || ''}`}>
      <h3 className="text-lg font-semibold mb-4 text-white">Filter Media</h3>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {/* Year Range */}
        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-300">Year</label>
          <div className="flex space-x-2 items-center">
            <input
              type="number"
              placeholder="Min"
              value={minYear}
              onChange={(e) => setMinYear(e.target.value)}
              className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <span className="text-gray-400">-</span>
            <input
              type="number"
              placeholder="Max"
              value={maxYear}
              onChange={(e) => setMaxYear(e.target.value)}
              className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        {/* Duration Range */}
        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-300">Duration (min)</label>
          <div className="flex space-x-2 items-center">
            <input
              type="number"
              placeholder="Min"
              value={minDuration}
              onChange={(e) => setMinDuration(e.target.value)}
              className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <span className="text-gray-400">-</span>
            <input
              type="number"
              placeholder="Max"
              value={maxDuration}
              onChange={(e) => setMaxDuration(e.target.value)}
              className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        {/* Resolution */}
        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-300">Resolution</label>
          <div className="flex flex-col space-y-1">
            {resolutionOptions.map((opt) => (
              <label key={opt.value} className="flex items-center space-x-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={resolutions.includes(opt.value)}
                  onChange={() => toggleResolution(opt.value)}
                  className="form-checkbox bg-gray-700 text-blue-500 border-gray-600 rounded focus:ring-blue-500"
                />
                <span className="text-sm text-gray-300">{opt.label}</span>
              </label>
            ))}
          </div>
        </div>

        {/* Media Type */}
        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-300">Media Type</label>
          <div className="flex flex-col space-y-1">
            {mediaTypeOptions.map((opt) => (
              <label key={opt.value} className="flex items-center space-x-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={mediaTypes.includes(opt.value)}
                  onChange={() => toggleMediaType(opt.value)}
                  className="form-checkbox bg-gray-700 text-blue-500 border-gray-600 rounded focus:ring-blue-500"
                />
                <span className="text-sm text-gray-300">{opt.label}</span>
              </label>
            ))}
          </div>
        </div>
      </div>

      <div className="mt-6 flex justify-end space-x-3">
        <button
          onClick={handleReset}
          className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded transition-colors"
        >
          Reset
        </button>
        <button
          onClick={handleApply}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm rounded font-medium transition-colors"
        >
          Apply Filters
        </button>
      </div>
    </div>
  );
};

export default FilterPanel;
