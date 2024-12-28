import { FaSearch } from "react-icons/fa";
import { MdClear } from "react-icons/md";
import { useState, useEffect } from "react";
import { getSuggestions, addSuggestion } from '../api/suggestions'
import { Bounce, ToastContainer, toast } from "react-toastify";

function SearchBox() {
    const [searchValue, setSearchValue] = useState("");
    const [results, setResults] = useState<string[]>([]);
    const [debouncedSearchValue, setDebouncedSearchValue] = useState("");

    const clearSearch = () => {
        setSearchValue("");
    };

    // Debounce Logic
    useEffect(() => {
        const handler = setTimeout(() => {
            setDebouncedSearchValue(searchValue)
        }, 500)

        return () => {
            clearTimeout(handler)
        };
    }, [searchValue]);

    // API Call on debounced value change
    useEffect(() => {
        if (debouncedSearchValue) {
            fetchResults(debouncedSearchValue)
        }
    }, [debouncedSearchValue]);

    const fetchResults = async (query: string) => {
        try {
            const result = await getSuggestions(query)
            setResults(result.data.suggestions || []);

            toast.success(`Fetched ${result.data.total_results} suggestions in ${result.data.time_taken} millis.`, {
                position: "bottom-right",
                autoClose: 1500,
                hideProgressBar: false,
                closeOnClick: false,
                pauseOnHover: true,
                draggable: true,
                progress: undefined,
                theme: "light",
                transition: Bounce,
            });
        } catch (error) {
            console.error("Error fetching data:", error);
        }
    };

    const addSearchQuery = async (query: string) => {
        try {
            addSuggestion(query)
        } catch (error) {
            console.error("Error adding query:", error);
        }
    }

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
                        onKeyDown={(e) => {
                            if (e.key === "Enter") {
                                addSearchQuery(searchValue)
                            }
                        }}
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
                        {results.length > 0 ? (
                            results.map((result, index) => (
                                <li
                                    key={index}
                                    className="px-4 py-2 hover:bg-gray-100 cursor-pointer"
                                    onClick={() => {
                                        setSearchValue(result)
                                    }}
                                >
                                    {result}
                                </li>
                            ))
                        ) : (
                            <li className="px-4 py-2 text-gray-500">No results found</li>
                        )}
                    </ul>
                </div>
            </div>

            <ToastContainer
                position="bottom-right"
                autoClose={1500}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick={false}
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="light"
                transition={Bounce}
            />
        </div>
    );
}

export default SearchBox;
