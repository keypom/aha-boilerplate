// PrizeCard.tsx
import React from 'react';
import { Prize } from '../PrizeHome';
import '../Gallery/PrizeGallery.css';

interface PrizeCardProps {
  prize: Prize;
  addToCart: (prizeId: number) => void;
}

const PrizeCard: React.FC<PrizeCardProps> = ({ prize, addToCart }) => {
  return (
    <div className="prize-card">
      <img src={prize.image} alt={prize.name} className="prize-image" />
      <h2 className="prize-name">{prize.name}</h2>
      <p className="prize-description">{prize.description}</p>
      <p className="prize-points">Points: {prize.pointCost}</p>
      {prize.inStock ? (
        <button className="redeem-button" onClick={() => addToCart(prize.id)}>
          Add to Cart
        </button>
      ) : (
        <p className="out-of-stock">Out of Stock</p>
      )}
    </div>
  );
};

export default PrizeCard;
