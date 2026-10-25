import Hero from "../components/Hero";
import Navbar from "../components/Navbar";
import Projects from "../components/Projects";
import Services from "../components/Services";
import Contact from "../components/Contact";

/*
import Skills from "../components/Skills";
import Creative from "../components/Creative";
*/

const Home = () => {
  return (
    <div className="bg-black text-white">
      <Navbar /> 
      <Hero />
      <Projects />
      <Services />
      {/*<Skills />
      <Creative />*/}
      <Contact />
    </div>
  );
};

export default Home;