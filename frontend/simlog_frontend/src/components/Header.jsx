import React from 'react';
import { useNavigate } from 'react-router-dom';

function Header() {
  const navigate = useNavigate();
  
  return (
    <div className='flex gap-4 p-4 bg-black'>
      <button onClick={() => navigate('/')} className="bg-white text-black px-2 py-2 rounded">
        Home
      </button>
      <button onClick={() => navigate('/add_flight')} className="bg-white text-black px-2 py-2 rounded">
        Post a new flight
      </button>
      <button onClick={() => navigate('/flights')} className="bg-white text-black px-2 py-2 rounded">
        View All Flights
      </button>
    </div>
  );
}

export default Header;