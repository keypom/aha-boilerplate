import React, { useEffect } from 'react';
import create from 'zustand';
import { useAppStore } from '../../state/appState';
import './Home.css'; // Import your CSS

function Home() {
  const resetFormState = useAppStore((state) => state.reset);

  useEffect(() => {
    // Clean store on load
    resetFormState();
  }, []);

  return (
    <div className="relative">
      <video autoPlay loop muted className="object-cover w-full h-[70vh]">
        <source src="/ext-assets/hero_video.mp4" type="video/mp4" />
        Your browser does not support the video tag.
      </video>

      <div className="absolute bottom-20 left-20 max-w-4xl h-full w-1/3 flex flex-col justify-center">
        <h1 className="text-4xl md:text-6xl font-lub-dub font-bold text-gray-800 mb-4">
          Heartbeat Heroes Challenge
        </h1>
        <p className="text-lg text-gray-800 mb-8">
          Fuel the Beat, Ignite a Legacy
        </p>
        <button className="bg-deep-red text-white py-3 px-6 font-semibold hover:bg-red-600 w-full">
          Get Started
        </button>
      </div>
    </div>
  );
}

export default Home;
