export default function Modal(props) {

    if (!props.isOpen)
        return (<></>)
    return (
        <div className="fixed top-0 left-0 w-screen h-screen" style={{backgroundColor: 'rgba(0, 0, 0, 0.8)'}}>
            <div className="relative w-fit p-2 top-1/2 left-1/2 translate-x-[-50%] translate-y-[-50%] bg-gray-50 rounded">
                <div className="flex justify-end">
                    <button onClick={() => { props.onRequestClose() }}>
                        <span className="x text-2xl"></span>
                    </button>
                </div>
                {props.children}
            </div>
        </div>
    )

}
