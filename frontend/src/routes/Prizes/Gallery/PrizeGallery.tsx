import React, { useState } from 'react';
import PrizeCard from '../Cards/PrizeCard'; // Import the PrizeCard component
import { Prize } from '../PrizeHome';
import './PrizeGallery.css'; // Import the Gallery.css file

interface PrizeGalleryProps {
  prizes: Prize[];
  addToCart: (prizeId: number) => void;
}

const PrizeGallery: React.FC<PrizeGalleryProps> = ({ prizes, addToCart }) => {
  const [sortOption, setSortOption] = useState('none');

  const handleSort = (option: string) => {
    // Sort the prizes based on the selected option
    if (option === 'price-low-high') {
      prizes.sort((a, b) => a.pointCost - b.pointCost);
    } else if (option === 'price-high-low') {
      prizes.sort((a, b) => b.pointCost - a.pointCost);
    } else if (option === 'newest') {
      prizes.sort((a, b) => b.id - a.id);
    } else if (option === 'oldest') {
      prizes.sort((a, b) => a.id - b.id);
    }
    setSortOption(option);
  };

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
      <div className="gallery">
        {prizes.map((prize) => (
          <PrizeCard key={prize.id} prize={prize} addToCart={addToCart} />
        ))}
      </div>
    </div>
  );
};

export default PrizeGallery;
