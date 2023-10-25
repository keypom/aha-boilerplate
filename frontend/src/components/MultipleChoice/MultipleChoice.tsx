import React from 'react';

interface MultipleChoiceProps {
  question: string;
  options: string[];
  selectedOption: string | null;
  onChange: (selectedOption: string) => void;
}

const MultipleChoice: React.FC<MultipleChoiceProps> = ({
  question,
  options,
  selectedOption,
  onChange,
}) => {
  return (
    <div className="mb-4">
      <p className="mb-2 text-lg font-semibold">{question}</p>
      <div className="space-y-2">
        {options.map((option, index) => (
          <div
            key={index}
            className={`rounded-md overflow-hidden cursor-pointer ${
              selectedOption === option
                ? 'bg-blue-500 text-white'
                : 'bg-white border border-gray-300 hover:bg-blue-50'
            }`}
            onClick={() => onChange(option)}
          >
            <div className="p-3">{option}</div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default MultipleChoice;
