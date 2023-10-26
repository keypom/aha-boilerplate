import React from 'react';
import { Link } from 'react-router-dom';
import logo from '../../ext-assets/AHA_LOGO-RGB_rk_LG.jpg';
import './Navbar.css';

function Navbar() {
  return (
    <nav className="bg-white p-4 flex items-center justify-between shadow-md">
      {/* AHA Logo on the left, wrapped in a Link */}
      <Link to="/" className="flex items-center">
        <img
          src={logo}
          alt="American Heart Association Logo"
          className="h-10 w-auto cursor-pointer"
        />
      </Link>

      {/* Navigation buttons on the right */}
      <div className="flex space-x-4">
        <Link to="/stats" className="text-black nav-link">
          Stats
        </Link>
        <Link to="/raffle" className="text-black nav-link">
          Raffle
        </Link>
        <Link to="/login" className="text-black nav-link">
          Login
        </Link>
      </div>
    </nav>
  );
}

export default Navbar;