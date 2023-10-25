import React, { useEffect } from 'react';
import create from 'zustand';
import { createBrowserRouter, RouterProvider, Route } from 'react-router-dom';
import RegistrationForm from '../../components/RegistrationForm';
import { useAppStore } from '../../state/appState';
import SquaresBG from '../../components/SquaresBG';
import DazeLogoSrc from './images/DazeLogo.png';

function Home() {
    const resetFormState = useAppStore((state) => state.reset);

    useEffect(() => {
        // Clean store on load
        resetFormState();
    }, []);

    return (
<>
  <div className="mx-20 flex min-h-screen flex-col items-center justify-start md:mx-0">
    <div className="mt-8"> {/* Add vertical space here */}
      <RegistrationForm />
    </div>
    <div
      // This flexbox trick will act as a footer unless content comes down too far,
      // and then it will get pushed
      className="mt-auto"
    ></div>
  </div>
  <SquaresBG />
</>


    );
}

export default Home;
