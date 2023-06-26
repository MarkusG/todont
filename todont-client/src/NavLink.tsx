export interface NavLinkProps {
    name: string;
    icon: string;
}

export default function NavLink({ name, icon }: NavLinkProps) {
    return (
        <div className="p-2 pt-3 flex flex-col gap-1 cursor-pointer bg-gray-100 text-primary-1000">
          <i className={`p-3 fa-xl ${icon}`}></i>
          <span>{name}</span>
        </div>
    )
}
