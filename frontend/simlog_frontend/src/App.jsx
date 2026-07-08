import Header from './components/Header'
import Home from './pages/Home'
import AddFlight from './pages/AddFlight'
import AllFlights from './pages/AllFlights'
import {BrowserRouter, Routes, Route} from 'react-router-dom'

function App() {

  return (
    <>
      <BrowserRouter>
        <Header/>
        <Routes>
          <Route path='/' element={<Home />} />
          <Route path='/add_flight' element={<AddFlight />} />
          <Route path='/flights' element={<AllFlights />} />
        </Routes>
      </BrowserRouter>
    </>
  )
}

export default App
