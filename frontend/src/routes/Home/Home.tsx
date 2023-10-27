import React, { useEffect } from 'react';
import create from 'zustand';
import { createBrowserRouter, RouterProvider, Route } from 'react-router-dom';
import { useAppStore } from '../../state/appState';
import SquaresBG from '../../components/SquaresBG';
import DazeLogoSrc from './images/DazeLogo.png';
import logo from '../../ext-assets/AHA_LOGO-RGB_rk_LG.jpg'; // Replace 'logo.png' with the actual filename of your logo

function Home() {
  const resetFormState = useAppStore((state) => state.reset);

  useEffect(() => {
    // Clean store on load
    resetFormState();
  }, []);

  return (
    <>
      <div className="bg-cream min-h-screen flex items-center justify-center">
        <div className="md:flex max-w-6xl mx-auto">
          <div className="w-1/2 p-8">
            <div className="text-4xl font-lub-dub font-bold mb-6">
              Welcome to the American Heart Association
            </div>

            <p className="text-lg">
              The American Heart Association is dedicated to improving heart health and preventing heart diseases. Join us in our mission to create a world of healthier hearts.
            </p>

            <div className="mt-12">
              <button className="bg-red-500 text-white py-3 px-6 rounded-full font-semibold hover:bg-red-600">
                Donate Now
              </button>
            </div>
          </div>

          <div className="w-1/2">
            <img src={logo} alt="American Heart Association Logo" className="w-full" />
          </div>
        </div>
      </div>
    </>
  );
}

export default Home;
