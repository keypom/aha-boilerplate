import React from 'react';
import logo from '../../ext-assets/AHA_LOGO-RGB_rk_LG.jpg'; // Replace 'logo.png' with the actual filename of your logo

const WelcomePage: React.FC = () => {
  return (
    <div className="bg-white text-black p-8">
      <div className="text-4xl font-lub-dub font-bold mb-6">
        Welcome to the American Heart Association
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div>
          <img src={logo} alt="American Heart Association Logo" className="w-full" />
        </div>

        <div className="text-lg">
          The American Heart Association is dedicated to improving heart health and preventing heart diseases. Join us in our mission to create a world of healthier hearts.
        </div>
      </div>

      <div className="mt-12">
        <button className="bg-red-500 text-white py-3 px-6 rounded-full font-semibold hover:bg-red-600">
          Donate Now
        </button>
      </div>
    </div>
  );
};

export default WelcomePage;
