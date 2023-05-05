export default function Todo({ title, description }) {
    return (
      <div className="p-4 flex justify-between border shadow-md">
        <div>
          <h2 className="text-xl">{title}</h2>
          <p>{description}</p>
        </div>
        <div className="ml-2 my-auto flex gap-2">
          <span className="cursor-pointer check"></span>
          <span className="cursor-pointer x"></span>
        </div>
      </div>
    )
}
