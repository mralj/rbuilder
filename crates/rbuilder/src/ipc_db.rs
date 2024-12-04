use alloy_eips::BlockNumberOrTag;
use reth_chainspec::ChainInfo;
use reth_db::common::{IterPairResult, PairResult, ValueOnlyResult};
use reth_db::cursor::{
    DbCursorRO, DbCursorRW, DbDupCursorRO, DbDupCursorRW, DupWalker, RangeWalker, ReverseWalker,
    Walker,
};
use reth_db::table::{DupSort, Table, TableImporter};
use reth_db::transaction::{DbTx, DbTxMut};
use reth_db::{Database, DatabaseError};
use reth_errors::{ProviderError, ProviderResult};
use reth_primitives::{BlockHash, BlockNumber, Header, SealedHeader};
use reth_provider::{
    BlockHashReader, BlockIdReader, BlockNumReader, DatabaseProviderFactory, DatabaseProviderRO,
    HeaderProvider, StateProviderBox, StateProviderFactory,
};
use revm_primitives::{B256, U256};
use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};

#[derive(Debug, Clone)]
pub struct IpcProvider();

impl<DB: Database> DatabaseProviderFactory<DB> for IpcProvider {
    fn database_provider_ro(&self) -> ProviderResult<DatabaseProviderRO<DB>> {
        ProviderResult::Err(ProviderError::UnsupportedProvider)
    }
}

impl HeaderProvider for IpcProvider {
    fn header(&self, block_hash: &BlockHash) -> ProviderResult<Option<Header>> {
        todo!()
    }

    fn header_by_number(&self, num: u64) -> ProviderResult<Option<Header>> {
        todo!()
    }

    fn header_td(&self, hash: &BlockHash) -> ProviderResult<Option<U256>> {
        todo!()
    }

    fn header_td_by_number(&self, number: BlockNumber) -> ProviderResult<Option<U256>> {
        todo!()
    }

    fn headers_range(&self, range: impl RangeBounds<BlockNumber>) -> ProviderResult<Vec<Header>> {
        todo!()
    }

    fn sealed_header(&self, number: BlockNumber) -> ProviderResult<Option<SealedHeader>> {
        todo!()
    }

    fn sealed_headers_while(
        &self,
        range: impl RangeBounds<BlockNumber>,
        predicate: impl FnMut(&SealedHeader) -> bool,
    ) -> ProviderResult<Vec<SealedHeader>> {
        todo!()
    }
}

impl StateProviderFactory for IpcProvider {
    fn latest(&self) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn state_by_block_number_or_tag(
        &self,
        number_or_tag: BlockNumberOrTag,
    ) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn history_by_block_number(&self, block: BlockNumber) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn history_by_block_hash(&self, block: BlockHash) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn state_by_block_hash(&self, block: BlockHash) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn pending(&self) -> ProviderResult<StateProviderBox> {
        todo!()
    }

    fn pending_state_by_hash(&self, block_hash: B256) -> ProviderResult<Option<StateProviderBox>> {
        todo!()
    }
}

impl BlockIdReader for IpcProvider {
    fn pending_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        todo!()
    }

    fn safe_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        todo!()
    }

    fn finalized_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        todo!()
    }
}

impl BlockNumReader for IpcProvider {
    fn chain_info(&self) -> ProviderResult<ChainInfo> {
        todo!()
    }

    fn best_block_number(&self) -> ProviderResult<BlockNumber> {
        todo!()
    }

    #[doc = " Returns the last block number associated with the last canonical header in the database."]
    fn last_block_number(&self) -> ProviderResult<BlockNumber> {
        todo!()
    }

    fn block_number(&self, hash: B256) -> ProviderResult<Option<BlockNumber>> {
        todo!()
    }
}

impl BlockHashReader for IpcProvider {
    fn block_hash(&self, number: BlockNumber) -> ProviderResult<Option<B256>> {
        todo!()
    }

    fn canonical_hashes_range(
        &self,
        start: BlockNumber,
        end: BlockNumber,
    ) -> ProviderResult<Vec<B256>> {
        todo!()
    }
}

/// Mock database used for testing with inner `BTreeMap` structure
// TODO
#[derive(Clone, Debug, Default)]
pub struct DatabaseMock {
    /// Main data. TODO (Make it table aware)
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Database for DatabaseMock {
    type TX = TxMock;
    type TXMut = TxMock;
    fn tx(&self) -> Result<Self::TX, DatabaseError> {
        Ok(TxMock::default())
    }

    fn tx_mut(&self) -> Result<Self::TXMut, DatabaseError> {
        Ok(TxMock::default())
    }
}

/// Mock read only tx
#[derive(Debug, Clone, Default)]
pub struct TxMock {
    /// Table representation
    _table: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl DbTx for TxMock {
    type Cursor<T: Table> = CursorMock;
    type DupCursor<T: DupSort> = CursorMock;

    fn get<T: Table>(&self, _key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        Ok(None)
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        Ok(true)
    }

    fn abort(self) {}

    fn cursor_read<T: Table>(&self) -> Result<Self::Cursor<T>, DatabaseError> {
        Ok(CursorMock { _cursor: 0 })
    }

    fn cursor_dup_read<T: DupSort>(&self) -> Result<Self::DupCursor<T>, DatabaseError> {
        Ok(CursorMock { _cursor: 0 })
    }

    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        Ok(self._table.len())
    }

    fn disable_long_read_transaction_safety(&mut self) {}
}

impl DbTxMut for TxMock {
    type CursorMut<T: Table> = CursorMock;
    type DupCursorMut<T: DupSort> = CursorMock;

    fn put<T: Table>(&self, _key: T::Key, _value: T::Value) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete<T: Table>(
        &self,
        _key: T::Key,
        _value: Option<T::Value>,
    ) -> Result<bool, DatabaseError> {
        Ok(true)
    }

    fn clear<T: Table>(&self) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn cursor_write<T: Table>(&self) -> Result<Self::CursorMut<T>, DatabaseError> {
        Ok(CursorMock { _cursor: 0 })
    }

    fn cursor_dup_write<T: DupSort>(&self) -> Result<Self::DupCursorMut<T>, DatabaseError> {
        Ok(CursorMock { _cursor: 0 })
    }
}

impl TableImporter for TxMock {}

/// Cursor that iterates over table
#[derive(Debug)]
pub struct CursorMock {
    _cursor: u32,
}

impl<T: Table> DbCursorRO<T> for CursorMock {
    fn first(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn seek_exact(&mut self, _key: T::Key) -> PairResult<T> {
        Ok(None)
    }

    fn seek(&mut self, _key: T::Key) -> PairResult<T> {
        Ok(None)
    }

    fn next(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn prev(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn last(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn current(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn walk(&mut self, start_key: Option<T::Key>) -> Result<Walker<'_, T, Self>, DatabaseError> {
        let start: IterPairResult<T> = match start_key {
            Some(key) => <Self as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Self as DbCursorRO<T>>::first(self).transpose(),
        };

        Ok(Walker::new(self, start))
    }

    fn walk_range(
        &mut self,
        range: impl RangeBounds<T::Key>,
    ) -> Result<RangeWalker<'_, T, Self>, DatabaseError> {
        let start_key = match range.start_bound() {
            Bound::Included(key) | Bound::Excluded(key) => Some((*key).clone()),
            Bound::Unbounded => None,
        };

        let end_key = match range.end_bound() {
            Bound::Included(key) | Bound::Excluded(key) => Bound::Included((*key).clone()),
            Bound::Unbounded => Bound::Unbounded,
        };

        let start: IterPairResult<T> = match start_key {
            Some(key) => <Self as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Self as DbCursorRO<T>>::first(self).transpose(),
        };

        Ok(RangeWalker::new(self, start, end_key))
    }

    fn walk_back(
        &mut self,
        start_key: Option<T::Key>,
    ) -> Result<ReverseWalker<'_, T, Self>, DatabaseError> {
        let start: IterPairResult<T> = match start_key {
            Some(key) => <Self as DbCursorRO<T>>::seek(self, key).transpose(),
            None => <Self as DbCursorRO<T>>::last(self).transpose(),
        };
        Ok(ReverseWalker::new(self, start))
    }
}

impl<T: DupSort> DbDupCursorRO<T> for CursorMock {
    fn next_dup(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn next_no_dup(&mut self) -> PairResult<T> {
        Ok(None)
    }

    fn next_dup_val(&mut self) -> ValueOnlyResult<T> {
        Ok(None)
    }

    fn seek_by_key_subkey(
        &mut self,
        _key: <T as Table>::Key,
        _subkey: <T as DupSort>::SubKey,
    ) -> ValueOnlyResult<T> {
        Ok(None)
    }

    fn walk_dup(
        &mut self,
        _key: Option<<T>::Key>,
        _subkey: Option<<T as DupSort>::SubKey>,
    ) -> Result<DupWalker<'_, T, Self>, DatabaseError> {
        Ok(DupWalker {
            cursor: self,
            start: None,
        })
    }
}

impl<T: Table> DbCursorRW<T> for CursorMock {
    fn upsert(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn insert(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn append(
        &mut self,
        _key: <T as Table>::Key,
        _value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete_current(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }
}

impl<T: DupSort> DbDupCursorRW<T> for CursorMock {
    fn delete_current_duplicates(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn append_dup(&mut self, _key: <T>::Key, _value: <T>::Value) -> Result<(), DatabaseError> {
        Ok(())
    }
}
