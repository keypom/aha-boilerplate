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
          className="h-16 w-auto cursor-pointer"
        />
      </Link>

      {/* Navigation buttons on the right */}
      <div className="flex items-center space-x-8">
        <Link to="/stats" className="text-black text-lg nav-link"> {/* Added text-lg class here */}
          Stats
        </Link>

        <div className="space-x-14">
          <Link to="/raffle" className="text-black text-lg nav-link"> {/* Added text-lg class here */}
            Raffle
          </Link>
          <Link to="/login" className="login-button text-lg"> {/* Added text-lg class here */}
            Login
          </Link>
        </div>
      </div>
    </nav>
  );
}

export default Navbar;
