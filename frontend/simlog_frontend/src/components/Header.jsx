import React from 'react';
import { useNavigate } from 'react-router-dom';

function Header() {
  const navigate = useNavigate();
  
  return (
    <div className='flex justify-between p-4 bg-blue-950'>
      <button onClick={() => navigate('/')} className=" text-white text-2xl font-bold px-2 py-2 rounded">
        Simlog
      </button>
      <div className='flex gap-4'>
        <button onClick={() => navigate('/add_flight')} className="bg-white text-black px-2 py-2 rounded">
          Post a new flight
        </button>
        <button onClick={() => navigate('/flights')} className="bg-white text-black px-2 py-2 rounded">
          View All Flights
        </button>
      </div>
    </div>
  );
}

export default Header;