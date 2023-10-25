import React from 'react';

// Not using this for now, opting instead for an inline spinner in the button.
const LoadingModal: React.FC = (props) => {
    return (
        <div className="absolute top-0 -left-0 right-0 bottom-0">
            <div className="absolute top-0 left-0 right-0 bottom-0 z-10 bg-black opacity-50 backdrop-blur-md"></div>
            <div className="">Spinner</div>
        </div>
    );
};

export default LoadingModal;
