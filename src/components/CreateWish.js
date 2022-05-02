import React, { useState } from 'react'
function CreateWish({ toggleModal }) {
    const [title, setTitle] = useState('')
    const [description, setDescription] = useState('')
    const [target, setTarget] = useState(0)
    const [showNotification, setShowNotification] = useState(false)
    const handleSubmit = (event) => {
        event.preventDefault()
        window.contract.add_wish({ title: title, donate: target * 1, description: description })
        setShowNotification(!showNotification)
        alert(`Wish info: ${title} ${target} ${description}`)
    }
    console.log(`its ${toggleModal}`);
    return (
        <div>
            {toggleModal == true && (
                <div className='addcrowdfund'>
                    <form onSubmit={handleSubmit}>
                        <label>
                            Enter Wish title:
                            <input
                                type="text"
                                value={title}
                                onChange={(e) => setTitle(e.target.value)}
                            />
                        </label><br></br>
                        <label>
                            Wish target:
                            <input
                                type="number"
                                value={target}
                                onChange={(e) => setTarget(e.target.value)}
                            />
                        </label><br></br>
                        <label>
                            Your wish description:
                            <input
                                type="text"
                                value={description}
                                onChange={(e) => setDescription(e.target.value)}
                            />
                        </label><br></br>
                        <input type="submit" className='submit' />
                    </form>
                </div>
            )}

            {showNotification && <Notification />}
        </div>

    )
}
function Notification() {
    return (
        <aside>
            <footer>
                <div>✔ Succeeded </div>
                <div>Added new Wish Just now</div>
            </footer>
        </aside>
    )
}
export default CreateWish