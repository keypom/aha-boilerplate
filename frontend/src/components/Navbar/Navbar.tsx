import React from 'react';
import { Link } from 'react-router-dom';
import './Navbar.css';
const assetUrl = process.env.PUBLIC_URL + '/ext-assets/AHA_LOGO-RGB_rk_LG.jpg';

function Navbar() {
  return (
    <nav className="bg-white p-4 flex items-center justify-between shadow-md">
      {/* AHA Logo on the left, wrapped in a Link */}
      <Link to="/" className="flex items-center">
        <img
          src={assetUrl}
          alt="American Heart Association Logo"
          className="h-16 w-auto cursor-pointer"
        />
      </Link>

      {/* Navigation buttons on the right */}
      <div className="flex items-center space-x-14">
        
        <div className="space-x-8">
          <Link to="/stats" className="text-black text-lg nav-link"> 
            Stats
          </Link>
          <Link to="/prizes" className="text-black text-lg nav-link"> 
            Prizes
          </Link>
        </div>

        <Link to="/login" className="login-button text-lg">
          Login
        </Link>
      </div>
    </nav>
  );
}

export default Navbar;
