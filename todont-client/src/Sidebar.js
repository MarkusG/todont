import NavLink from './NavLink.js';
import { solid } from '@fortawesome/fontawesome-svg-core/import.macro'

export default function Sidebar() {
    return (
        <div className="w-full md:w-[80px] md:h-full h-[60px] py-4 bg-[#1879c9] text-white text-center">
          <NavLink name="Todos" icon={solid('table-list')}/>
        </div>
    );
}
