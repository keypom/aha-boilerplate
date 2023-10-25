import React from 'react';
import create from 'zustand';

type NetworkType = 'testnet' | 'mainnet';

interface UrlParamState {
    network: NetworkType;
    privateKey: string;
    setNetwork: (network: NetworkType) => void;
    setPrivateKey: (key: string) => void;
}

const useUrlParamStore = create<UrlParamState>((set) => ({
    network: 'testnet',
    privateKey: '',
    setNetwork: (network) => set({ network }),
    setPrivateKey: (privateKey) => set({ privateKey }),
}));

export { useUrlParamStore };
