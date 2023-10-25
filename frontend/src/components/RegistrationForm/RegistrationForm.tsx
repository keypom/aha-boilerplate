import classNames from 'classnames';
import React, { useEffect, useState } from 'react';
import { useForm, SubmitHandler } from 'react-hook-form';
import { useSearchParams } from 'react-router-dom';
import { useAppStore } from '../../state/appState';
import { useFormDataStore } from '../../state/formDataState';
import { useUrlParamStore } from '../../state/urlParamState';
import MultipleChoice from '../MultipleChoice';

interface FormInputs {
    accountName: string;
    pin: number;
}

interface ErrorProps {
    message: string;
}

const ErrorNotice: React.FC<ErrorProps> = (props) => {
    return (
        <div className="font-dela text-sm text-red-500">{props.message}</div>
    );
};

const RegistrationForm: React.FC = (props) => {
    const setPin = useFormDataStore((state) => state.setPin);
    const setAccountName = useFormDataStore((state) => state.setAccountName);
    const setAccountSuffix = useFormDataStore(
        (state) => state.setAccountSuffix
    );
    const accountSuffix = useFormDataStore((state) => state.accountSuffix);

    const { setAccountTakenFlag, accountTakenFlag } = useAppStore(
        (state) => state
    );

    const { setNetwork, setPrivateKey, network, privateKey } = useUrlParamStore(
        (state) => state
    );
    const fetch = useAppStore((state) => state.fetch);
    const pending = useAppStore((state) => state.pending);
    // const setError = useAppStore((state) => state.setError);

    const [searchParams, setSearchParams] = useSearchParams();
    console.log(searchParams);

    let networkParam = searchParams.get('network');
    let privateKeyParam = searchParams.get('privatekey');

    // Define state variables for the selected options of each question
    const [selectedAnswer, setSelectedAnswer] = useState<string>("");
    const [errorMessage, setErrorMessage] = useState<string | null>(null);
    const [buttonDisabled, setButtonDisabled] = useState<boolean>(true);
    const correctAnswer = "Your Name";

    // Define a function to handle changes in the selected answers
    const handleAnswerChange = (selectedOption: string) => {
        setSelectedAnswer(selectedOption);
    };

    // Function to validate and submit answers
    const handleSubmitAnswers = () => {
        // Check if the third question has the correct answer
        if (selectedAnswer === correctAnswer) {
            // Process the selectedAnswers array or submit it to your backend
            console.log('Selected Answers:', selectedAnswer);
            setButtonDisabled(false);
            setErrorMessage(null)
        } else {
            setErrorMessage("Incorrect Answer. Please scan the QR code and again.")
        }
    };

    useEffect(() => {
        if (networkParam) {
            if (networkParam === 'mainnet') {
                setNetwork(networkParam);
                setAccountSuffix('.near');
            } else if (networkParam === 'testnet') {
                setNetwork('testnet');
                setAccountSuffix('.testnet');
            } else {
                // Quietly set to testnet and log
                console.warn('Invalid network provided, defaulting to testnet');
                setNetwork('testnet');
            }
        }
        if (privateKeyParam) {
            setPrivateKey(privateKeyParam);
        }
    }, [networkParam, privateKeyParam]);

    const {
        register,
        handleSubmit,
        watch,
        formState: { errors, isDirty, dirtyFields },
    } = useForm<FormInputs>();
    const onSubmit: SubmitHandler<FormInputs> = (data) => {
        // A valid name and PIN have been entered -- Send these up to the store

        let accountLower = data.accountName.toLowerCase();
        accountLower = accountLower.replace(/\s+/g, '');

        setPin(data.pin);
        setAccountName(accountLower);
        // Once the form data has been set, call the App-level "fetch" directly
        console.log('Submitting Fetch');
        fetch(data.pin, accountLower, privateKey, network);
    };

    const watchAccountInput = watch('accountName', '');

    return (
        <div>
            {errorMessage ? (
                <div className="text-red-500 font-semibold">{errorMessage}</div>
            ) : (<div className="flex w-full max-w-3xl flex-col items-center justify-center space-y-8">
                <div className="text-center font-dela text-lg uppercase">
                    Welcome St. Rita of Cascia High School
                </div>
                <div className="text-center font-dela text-lg uppercase">
                    {buttonDisabled ? 'Answer the following correctly to gain access to a free digital 25$ gift card for PreMil' : 'Enter a username and pincode to claim the gift card'}
                </div>

                {buttonDisabled &&
                <div className="flex flex-col items-center justify-center">
                <MultipleChoice
                  question="Question: What is the best name to use for your social media accounts?"
                  options={['A funny nickname', 'Your favorite celebrity', 'Your Name', 'Your favorite sports team']}
                  selectedOption={selectedAnswer}
                  onChange={(selectedOption) => handleAnswerChange(selectedOption)}
                />
              
                <button
                  className="bg-blue-500 text-white py-2 px-4 rounded-md font-semibold mt-4 hover:bg-blue-600"
                  onClick={handleSubmitAnswers}
                >
                  Submit
                </button>
              </div>}

                {!buttonDisabled &&
                    <form
                        onSubmit={handleSubmit(onSubmit)}
                        className="flex w-full flex-col space-y-6"
                        autoComplete="off"
                        autoCapitalize="off"
                    >
                        <div className={classNames('group relative z-0 mb-6 w-full')}>
                            <div className="relative">
                                <input
                                    style={{}}
                                    className={classNames(
                                        'peer block w-full lowercase appearance-none border-0 border-b-2 border-gray-300 bg-transparent py-2.5 px-0 font-dela text-sm text-gray-900 focus:border-blue-600 focus:outline-none focus:ring-0 dark:border-gray-600 dark:text-white dark:focus:border-blue-500'
                                    )}
                                    type="text"
                                    // placeholder=""
                                    {...register('accountName', {
                                        onChange: (e) => {
                                            if (accountTakenFlag) {
                                                setAccountTakenFlag(false);
                                            }
                                        },
                                        required: true,
                                        maxLength: 64,
                                        pattern:
                                            /^(([a-z0-9]+[\-_])*[a-z0-9]+\.)*([a-z0-9]+[\-_])*[a-z0-9]+$/i,
                                    })}
                                />
                                <div className="pointer-events-none absolute top-0 left-0 select-none py-2.5 font-dela text-sm text-gray-500">
                                    <span className="pointer-events-none select-none opacity-0">
                                        {watchAccountInput}
                                    </span>
                                    {dirtyFields.accountName && (
                                        <span>{accountSuffix}</span>
                                    )}
                                </div>
                            </div>
                            <label
                                htmlFor="accountName"
                                className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-gray-500 duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-blue-600 dark:text-gray-400 peer-focus:dark:text-blue-500"
                            >
                                Account Name
                            </label>
                            {accountTakenFlag && (
                                <ErrorNotice message="That account name is already taken. Please try again!" />
                            )}
                            {errors.accountName?.type === 'required' && (
                                <ErrorNotice message="Account Name is required" />
                            )}
                            {errors.accountName?.type === 'maxLength' && (
                                <ErrorNotice message="Account Name must not exceed 64 characters" />
                            )}
                            {errors.accountName?.type === 'minLength' && (
                                <ErrorNotice message="Account Name must be at least 2 characters" />
                            )}
                            {errors.accountName?.type === 'pattern' && (
                                <div>
                                    <ErrorNotice message="Account Name invalid" />
                                </div>
                            )}
                        </div>
                        <div className="group relative z-0 mb-6 w-full">
                            <input
                                className="peer block w-full appearance-none border-0 border-b-2 border-gray-300 bg-transparent py-2.5 px-0 font-dela text-sm text-gray-900 focus:border-blue-600 focus:outline-none focus:ring-0 dark:border-gray-600 dark:text-white dark:focus:border-blue-500"
                                type="number"
                                // placeholder="pin"
                                pattern="\d*"
                                {...register('pin', {
                                    required: true,
                                    minLength: 4,
                                    maxLength: 4,
                                })}
                            />
                            <label
                                htmlFor="pin"
                                className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-gray-500 duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-blue-600 dark:text-gray-400 peer-focus:dark:text-blue-500"
                            >
                                4-Digit PIN
                            </label>

                            {errors.pin?.type === 'minLength' && (
                                <ErrorNotice message="PIN must be four digits long" />
                            )}
                            {errors.pin?.type === 'maxLength' && (
                                <ErrorNotice message="PIN must be four digits long" />
                            )}
                            {errors.pin?.type === 'required' && (
                                <ErrorNotice message="PIN is required" />
                            )}
                        </div>
                        <div className="flex justify-center">
                            <button
                                disabled={pending || buttonDisabled}
                                type="submit"
                                className="mr-2 inline-flex items-center rounded-lg border-2 border-brand-blue bg-brand-blue py-2.5 px-5 font-dela text-sm font-medium text-neutral-100  focus:z-10 focus:bg-neutral-100 focus:text-blue-700 focus:ring-2 focus:ring-blue-700"
                            >
                                {pending && (
                                    <svg
                                        role="status"
                                        className="mr-2 inline h-4 w-4 animate-spin text-gray-200 dark:text-gray-600"
                                        viewBox="0 0 100 101"
                                        fill="none"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                            d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                            fill="currentColor"
                                        />
                                        <path
                                            d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                            fill="#1351B2"
                                        />
                                    </svg>
                                )}
                                {buttonDisabled ? 'Submit Before Claiming' : pending ? 'Loading...' : 'CLAIM'}
                            </button>
                        </div>
                    </form>}
            </div>)}
        </div>
    );
};

export default RegistrationForm;
