import React from 'react';
import {
    Outlet,
    useActionData,
    useNavigate,
    useSearchParams,
} from 'react-router-dom';
import { useAppStore } from '../../state/appState';

const Root: React.FC = (props) => {
    const navigate = useNavigate();

    const [searchParams, setSearchParams] = useSearchParams();

    const network = searchParams.get('network');
    const privateKey = searchParams.get('privatekey');

    // React to changes in the main App state and push routes accordingly
    const subscribeSuccess = useAppStore.subscribe(
        (state) => state.completed,
        (completed, prevCompleted) => {
            console.log('Got new completed state', completed, prevCompleted);
            if (completed && !prevCompleted) {
                console.log('Rerouting');
                navigate('/success');
            }
        }
    );

    const subscribeError = useAppStore.subscribe(
        (state) => state.error,
        (error, prevError) => {
            console.log('got new error', error);
            if (error) {
                navigate('/error');
            }
        }
    );

    return (
        <div>
            {/* <div className="bg-blue-100 py-8 text-center text-blue-600">
                <h1 className="mb-4 text-2xl underline">Debug area</h1>
                <div>
                    network:{' '}
                    {network ? (
                        network
                    ) : (
                        <span className="text-red-600">NONE PROVIDED</span>
                    )}
                </div>
                <div>
                    privatekey:{' '}
                    {privateKey ? (
                        privateKey
                    ) : (
                        <span className="text-red-600">NONE PROVIDED</span>
                    )}
                </div>
            </div> */}
            <Outlet />
        </div>
    );
};

export default Root;
