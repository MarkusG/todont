import './App.css';
import NavLink from './NavLink.js';
import Todo from './Todo.js';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro'

function App() {
  return (
      <>
      <div className="flex flex-col md:flex-row h-screen w-screen">
      { /* sidebar */ }
        <div className="w-full md:w-[80px] md:h-full h-[60px] py-4 bg-slate-700 text-white text-center">
          <NavLink name="Todos" icon={solid('table-list')}/>
        </div>
        { /* main page container */ }
        <div className="w-full p-4">
          <div className="flex flex-col gap-4">
            {[0,1,2,3].map((i) => (
                <div key={i}>
                    <Todo title={"Todo title " + i} description="This is a rather longer description to the todo. Perhaps it contains some rambling. Really there's no telling how long this will be. Hmmmmmmmmmm."/>
                </div>
            ))}
          </div>
        </div>
      </div>
      </>
  );
}

export default App;
