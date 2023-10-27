import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faTwitter, faTelegram, faGithub } from '@fortawesome/free-brands-svg-icons';
import logo from '../../ext-assets/powered_by_keypom4x.png';
import './Footer.css';

function Footer() {
  return (
    <footer className="footer bg-gray-900 text-white py-8">
      <div className="footer-container container mx-auto flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
        <div className="flex flex-col items-center md:items-start space-y-4 md:space-y-4">
          <a href="https://keypom.xyz" target="_blank" rel="noopener noreferrer">
            <img
              src={logo}
              alt="keypom-logo"
              className="w-full md:w-64 h-auto cursor-pointer"
            />
          </a>
          <div className="text-xs md:text-s mt-2">
            &copy; {new Date().getFullYear()} Your Company
          </div>
        </div>
        <div className="footer-links flex space-x-6 items-center">
          <a href="https://twitter.com/keypomxyz" target="_blank" rel="noopener noreferrer">
            <FontAwesomeIcon icon={faTwitter} size="lg" />
          </a>
          <a href="https://t.me/yourtelegram" target="_blank" rel="noopener noreferrer">
            <FontAwesomeIcon icon={faTelegram} size="lg" />
          </a>
          <a href="https://github.com/keypom" target="_blank" rel="noopener noreferrer">
            <FontAwesomeIcon icon={faGithub} size="lg" />
          </a>
          <a className="hover:underline" href="mailto:contact@yourcompany.com">
            Contact Us
          </a>
        </div>
      </div>
    </footer>
  );
}


export default Footer;