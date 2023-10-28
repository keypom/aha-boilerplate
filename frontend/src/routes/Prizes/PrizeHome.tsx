import React, { useState } from 'react';
import './PrizeHome.css';
import PrizeCard from './Cards/PrizeCard';
import PrizeGallery from './Gallery/PrizeGallery';
const assetUrl = process.env.PUBLIC_URL + '/ext-assets/stock_prize_image.jpg';

export interface Prize {
  id: number;
  name: string;
  category: string;
  description: string;
  pointCost: number;
  image: string;
  inStock: boolean;
}

const PrizeHome: React.FC = (props) => {
  const [prizes, setPrizes] = useState([
    {
      id: 1,
      name: 'Smartphone',
      category: 'Electronics',
      description: 'The latest smartphone with amazing features.',
      pointCost: 500,
      image: assetUrl,
      inStock: true,
    },
    {
      id: 2,
      name: 'Gift Card',
      category: 'Gift Cards',
      description: 'A $50 gift card for your favorite store.',
      pointCost: 200,
      image: assetUrl,
      inStock: true,
    },
    {
      id: 3,
      name: 'Jersey',
      category: 'Jerseys',
      description: 'A signed Jersey',
      pointCost: 300,
      image: assetUrl,
      inStock: false,
    },
    {
      id: 4,
      name: 'Keyboard',
      category: 'Keyboards',
      description: 'A nice keyboard',
      pointCost: 25,
      image: assetUrl,
      inStock: true,
    },
  ]);

  const userPoints = 750;
  const [cart, setCart] = useState<Prize[]>([]);

  const addToCart = (prizeId: number) => {
    const prizeToAdd = prizes.find((prize) => prize.id === prizeId);
    if (prizeToAdd) {
      setCart([...cart, prizeToAdd]);
    }
  };

  const removeFromCart = (prizeId: number) => {
    setCart(cart.filter((item) => item.id !== prizeId));
  };

  return (
    <div className="container">
      <div className="header">
        <h1 className="title">Prizes Shop</h1>
        <p className="sub-text">Choose your reward and redeem it using your points.</p>
      </div>
      <PrizeGallery prizes={prizes} addToCart={addToCart} />
      <div className="cart">
        <h2>Your Cart</h2>
        {cart.length === 0 ? (
          <p>Your cart is empty.</p>
        ) : (
          <ul>
            {cart.map((item) => (
              <li key={item.id}>
                <h3>{item.name}</h3>
                <p>{item.description}</p>
                <p>Points: {item.pointCost}</p>
                <button className="remove-button" onClick={() => removeFromCart(item.id)}>
                  Remove
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default PrizeHome;
