import React from 'react';
import { useEffect } from 'react';
import { useFormDataStore } from '../../state/formDataState';
import { useAppStore } from '../../state/appState';

const Success: React.FC = (props) => {
    const { accountName, accountSuffix, pin } = useFormDataStore(
        (state) => state
    );
    const setError = useAppStore((state) => state.setError);
    const url = useAppStore((state) => state.url) || '/';

    useEffect(() => {
        if (!accountName) {
            // TODO: Re-enable this
            // setError("Account name not found")
        }
    }, []);

    return (
        <div className="absolute top-0 bottom-0 left-0 right-0 bg-brand-blue pt-16 text-center font-dela text-brand-cream">
            <div className="mb-8 text-5xl uppercase">Success!</div>
            <div className="text-lg uppercase">Your PIN is: </div>
            <div className="text-3xl shadow-black drop-shadow-hard">{pin}</div>
            <div className="mt-4 text-lg uppercase">Your Account name is:</div>
            <div className="text-3xl shadow-black drop-shadow-hard">
                {accountName}
                {accountSuffix}
            </div>
            <div className="drop-shadow-m mt-8 text-sm uppercase shadow-black">
                <div>Steps to Claim:</div>
                <div className="mt-2">
                    1. Hit Button To Proceed
                </div>
                <div className="mt-2">
                    2. Enter a Password to Unlock Prize
                </div>
                <div className="mt-2">
                    3. Go To Collectibles Tab
                </div>
                <div className="mt-2">
                    4. Show This Page to Prize Booth
                </div>
            </div>
            <div className="mt-20">
                <a
                    className="rounded bg-white py-4 px-8 capitalize text-brand-blue drop-shadow-hard drop-shadow-lg hover:text-blue-700"
                    href={url}
                    rel={'noreferrer'}
                    target={'_blank'}
                >
                    Proceed to Prize
                </a>
            </div>
            <div className="mt-8 text-xs uppercase">
                make sure you show your collectible to the prize booth
            </div>
        </div>
    );
};

export default Success;
