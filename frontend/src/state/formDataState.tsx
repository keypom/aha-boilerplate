import React from 'react';
import create from 'zustand';

type AccountSuffix = '.near' | '.testnet';

interface FormState {
    pin: number;
    accountName: string;
    setAccountName: (accountName: string) => void;
    setPin: (pin: number) => void;
    accountSuffix: AccountSuffix;
    setAccountSuffix: (suffix: AccountSuffix) => void;
}

const useFormDataStore = create<FormState>((set) => ({
    accountName: '',
    pin: 1111,
    setAccountName: (accountName) => set({ accountName }),
    setPin: (pin) => set({ pin }),
    accountSuffix: '.testnet', // Default to testnet
    setAccountSuffix: (accountSuffix) => set({ accountSuffix }),
}));

export { useFormDataStore };
