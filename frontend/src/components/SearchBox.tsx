import { FaSearch } from "react-icons/fa";
import { MdClear } from "react-icons/md";
import { useState } from "react";

function SearchBox() {
    const [searchValue, setSearchValue] = useState("");

    const clearSearch = () => {
        setSearchValue("");
    };

    return (
        <div className="flex items-center justify-center h-screen bg-gray-100">
            <div className="w-96 p-4 bg-white border border-gray-300 rounded-lg shadow-sm">
                <div className="relative flex items-center">
                    {/* Search Icon */}
                    <FaSearch className="absolute left-3 text-gray-400" />

                    {/* Search Input */}
                    <input
                        type="text"
                        value={searchValue}
                        onChange={(e) => setSearchValue(e.target.value)}
                        placeholder="Search..."
                        className="w-full pl-10 pr-10 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />

                    {/* Clear Icon */}
                    {searchValue && (
                        <MdClear
                            className="absolute right-3 text-gray-400 cursor-pointer hover:text-gray-600"
                            onClick={clearSearch}
                        />
                    )}
                </div>

                {/* Dropdown List */}
                <div className="mt-2 bg-white border border-gray-300 rounded-lg shadow-sm">
                    <ul className="text-sm text-gray-700">
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">macbook pro</li>
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">
                            macbook <span className="text-xs text-gray-500">(in Computers & Tablets)</span>
                        </li>
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">
                            macbook <span className="text-xs text-gray-500">(in Laptop Accessories)</span>
                        </li>
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">macbook air</li>
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">macbook pro 13</li>
                        <li className="p-2 hover:bg-gray-100 cursor-pointer">macbook port</li>
                    </ul>
                </div>
            </div>
        </div>
    );
}

export default SearchBox;
