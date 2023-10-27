import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Layout from '../Layout';
import Home from './Home';
import Stats from './Stats';
import Prizes from './Prizes';
import Error from './Error';

const AppRoutes = () => {
    return (
        <Routes>
            <Route path="/" element={<Layout><Home /></Layout>} />
            <Route path="/stats" element={<Layout><Stats /></Layout>} />
            <Route path="/prizes" element={<Layout><Prizes /></Layout>} />
            <Route path="/error" element={<Layout><Error /></Layout>} />
        </Routes>
    );
};

export default AppRoutes;