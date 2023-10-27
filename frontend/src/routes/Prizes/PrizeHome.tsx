import React, { useState } from 'react';
import './PrizeHome.css'; // Import your CSS
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
          image: assetUrl, // Replace with actual image source
          inStock: true,
        },
        {
          id: 2,
          name: 'Gift Card',
          category: 'Gift Cards',
          description: 'A $50 gift card for your favorite store.',
          pointCost: 200,
          image: assetUrl, // Replace with actual image source
          inStock: true,
        },
        // Add more prize objects as needed
      ]);

  const userPoints = 750; // Replace with the user's actual points
  const [cart, setCart] = useState<Prize[]>([]); // Store selected items in the cart

  const addToCart = (prizeId: number) => {
    // Add the selected prize to the cart
    const prizeToAdd = prizes.find((prize) => prize.id === prizeId);
    if (prizeToAdd) {
      setCart([...cart, prizeToAdd]);
    }
  };

  const removeFromCart = (prizeId: number) => {
    // Filter out the selected prize from the cart
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
        <h2>Cart</h2>
        <ul>
          {cart.map((item) => (
            <li key={item.id}>
              <h3>{item.name}</h3>
              <p>{item.description}</p>
              <p>Points: {item.pointCost}</p>
              <button
                className="remove-button"
                onClick={() => removeFromCart(item.id)}
              >
                Remove
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default PrizeHome;
