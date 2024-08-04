import { BiBookmarkAlt, BiPencil, BiExport } from "react-icons/bi";
const SideBar = ({ onIconClick }) => {
  return (
    <div className="fixed top-0 left-0 h-screen w-16 m-0 flex flex-col 
    bg-gray-900 text-white shadow-lg">
      <SideBarIcon icon={<BiPencil size="28" />} text="Create new" onClick={() => onIconClick('CreateForm')} />
      <SideBarIcon icon={<BiBookmarkAlt size="28" />} text="View items" onClick={() => onIconClick('ListView')} />
      <SideBarIcon icon={<BiExport size="28" />} text="Export items" />
    </div>
  )
};

const SideBarIcon = ({ icon, text = 'tooltip', onClick }) => (
  <div className="sidebar-icon group" onClick={onClick}>
    {icon}
    <span className="sidebar-tooltip group-hover:scale-100">
      {text}
    </span>
  </div>
);

export default SideBar
