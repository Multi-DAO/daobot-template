import React from 'react';
import PropTypes from 'prop-types';

export default function Form({ onSubmit, daoId, setDaoId, nftId, setNftId, currentUser }) {
    return (
        <form onSubmit={onSubmit}>
            <fieldset id="fieldset">
                <p></p>
                <p className="highlight">
                    <div>
                        <label htmlFor="accountId">DAO account id:</label>
                    <input
                        autoFocus
                            id="accountId"
                            name="accountId"
                        required
                        value={daoId}
                        onChange={(e) => { setDaoId(e.target.value); }}
                        />
                    </div>
                    <div>
                    <label htmlFor="nft">NFT id:</label>
                    <input
                            id="nft"
                            name="nft"
                        required
                        value={nftId}
                        onChange={(e) => { setNftId(e.target.value); }}
                        />
                    </div>
                </p>
                <button type="submit">
                    Connect
                </button>
            </fieldset>
        </form>
    );
}

Form.propTypes = {
    onSubmit: PropTypes.func.isRequired,
    currentUser: PropTypes.shape({
        accountId: PropTypes.string.isRequired
    })
};
