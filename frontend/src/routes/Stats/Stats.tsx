import React from 'react';
import {
    Outlet,
    useActionData,
    useNavigate,
    useSearchParams,
} from 'react-router-dom';
import { useAppStore } from '../../state/appState';

const Stats: React.FC = (props) => {
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
            STATS PAGE
        </div>
    );
};

export default Stats;
