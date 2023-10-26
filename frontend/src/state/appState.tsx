import React from 'react';
import create, { StateCreator } from 'zustand';
import { subscribeWithSelector } from 'zustand/middleware';
import { succeed, fail } from '../mock-api/mock';

interface FTRet {
    error: string | null;
    account: string | null;
}

interface AppState {
    completed: boolean;
    pending: boolean;
    error?: string | boolean;
    setError: (error: string | boolean) => void;
    fetch: (
        pin: number,
        accountName: string,
        privateKey: string,
        network: 'mainnet' | 'testnet'
    ) => void;
    reset: () => void;
    url: string | null;
    setUrl: (url: string) => void;
    accountTakenFlag: boolean;
    setAccountTakenFlag: (accountTakenFlag: boolean) => void;
}

const createAppSlice: StateCreator<
    AppState,
    [['zustand/subscribeWithSelector', never]],
    [],
    AppState
> = (set) => ({
    url: null,
    setUrl: (url) => set({ url }),
    completed: false,
    pending: false,
    error: false,
    setError: (error: string | boolean) => set({ error }),
    reset: () =>
        set({
            completed: false,
            pending: false,
            error: false,
        }),
    accountTakenFlag: false,
    setAccountTakenFlag: (accountTakenFlag) => set({ accountTakenFlag }),
    fetch: async (pin, accountName, privateKey, network) => {
        set({ error: false });
        set({ pending: true });
        set({ accountTakenFlag: false });
        try {
            // const res = await succeed();
            await setTimeout(() => succeed(), 1000);
            let res = {
                error: "none",
                account: "TEST_ACCOUNT"
            }

            console.log('got res', res);
            set({ pending: false });
            if (res.error === 'Account exists') {
                // offramp to handle this specific error in the form itself
                set({ accountTakenFlag: true });
            } else if (res.error) {
                // generic, other errors
                set({
                    error: res.error,
                });
            } else if (res.account) {
                set({ completed: true, url: res.account });
                // Receiving "false" from the API means a username has likely been taken
            } else {
                // This should not be possible, but should have a case here anyway
                set({
                    error: 'Something went wrong. Please try again.',
                });
            }
            // return res.account;
        } catch (e: any) {
            set({ pending: false });
            // Something has gone wrong; throw an error
            set({ error: e.toString() });
            return false;
        }
    },
});

const useAppStore = create<AppState>()(subscribeWithSelector(createAppSlice));

export { useAppStore };
