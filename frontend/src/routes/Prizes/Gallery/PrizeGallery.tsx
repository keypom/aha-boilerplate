import React, { useState } from 'react';
import PrizeCard from '../Cards/PrizeCard';
import { Prize } from '../PrizeHome';
import './PrizeGallery.css';

interface PrizeGalleryProps {
  prizes: Prize[];
  addToCart: (prizeId: number) => void;
}

const PrizeGallery: React.FC<PrizeGalleryProps> = ({ prizes, addToCart }) => {
  const [sortOption, setSortOption] = useState('none');
  const [searchInput, setSearchInput] = useState('');

  const handleSort = (option: string) => {
    const sortedPrizes = [...prizes]; // Create a copy to avoid mutating the original array

    if (option === 'price-low-high') {
      sortedPrizes.sort((a, b) => a.pointCost - b.pointCost);
    } else if (option === 'price-high-low') {
      sortedPrizes.sort((a, b) => b.pointCost - a.pointCost);
    } else if (option === 'newest') {
      sortedPrizes.sort((a, b) => b.id - a.id);
    } else if (option === 'oldest') {
      sortedPrizes.sort((a, b) => a.id - b.id);
    }
    
    setSortOption(option);
  };

  const filteredPrizes = prizes.filter((prize) =>
    prize.name.toLowerCase().includes(searchInput.toLowerCase())
  );

  return (
    <div className="gallery-container">
      <div className="filters">
        <span
          className={`filter ${sortOption === 'price-low-high' ? 'active' : ''}`}
          onClick={() => handleSort('price-low-high')}
        >
          Price Low to High
        </span>
        <span
          className={`filter ${sortOption === 'price-high-low' ? 'active' : ''}`}
          onClick={() => handleSort('price-high-low')}
        >
          Price High to Low
        </span>
        <span
          className={`filter ${sortOption === 'newest' ? 'active' : ''}`}
          onClick={() => handleSort('newest')}
        >
          Newest
        </span>
        <span
          className={`filter ${sortOption === 'oldest' ? 'active' : ''}`}
          onClick={() => handleSort('oldest')}
        >
          Oldest
        </span>
      </div>
      <div className="search-container">
        <input
          type="text"
          placeholder="Search prizes"
          value={searchInput}
          onChange={(e) => setSearchInput(e.target.value)}
        />
      </div>
      <div className="gallery">
        {filteredPrizes.map((prize) => (
          <PrizeCard key={prize.id} prize={prize} addToCart={addToCart} />
        ))}
      </div>
    </div>
  );
};

export default PrizeGallery;
