import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

interface NavLinkProps {
    name: string;
    icon: any;
}

export default function NavLink({ name, icon }: NavLinkProps) {
    console.log(typeof icon);
    return (
        <div className="p-2 pt-3 flex flex-col gap-1 cursor-pointer bg-gray-100 text-primary-1000">
          <FontAwesomeIcon icon={icon} size="xl"/>
          <span>{name}</span>
        </div>
    )
}
