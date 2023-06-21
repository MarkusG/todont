import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

export default function NavLink({ name, icon }) {
    return (
        <div className="p-2 pt-3 flex flex-col gap-1 cursor-pointer bg-[#eee] text-[#010015]">
          <FontAwesomeIcon icon={icon} size="xl"/>
          <span>{name}</span>
        </div>
    )
}
