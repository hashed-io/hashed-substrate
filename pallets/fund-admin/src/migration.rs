//! Various pieces of common functionality.
use super::*;

const LOG_TARGET: &str = "\nFund Admin pallet migration ";
use crate::types::*;
use frame_support::{log, pallet_prelude::*, storage_alias, traits::OnRuntimeUpgrade, Identity};
use sp_runtime::Saturating;
use sp_std::vec::Vec;

mod v0 {
  use super::*;

  #[derive(Decode, Encode)]
  pub struct OldDrawdownData<T: Config> {
    pub project_id: ProjectId,
    pub drawdown_number: DrawdownNumber,
    pub drawdown_type: DrawdownType,
    pub total_amount: TotalAmount,
    pub status: DrawdownStatus,
    pub bulkupload_documents: Option<Documents<T>>,
    pub bank_documents: Option<Documents<T>>,
    pub description: Option<FieldDescription>,
    pub feedback: Option<FieldDescription>,
    pub status_changes: DrawdownStatusChanges<T>,
    pub created_date: CreatedDate,
    pub closed_date: CloseDate,
  }

  // #[cfg(feature = "try-runtime")]
  #[storage_alias]
  pub(super) type DrawdownsInfo<T: Config> =
    StorageMap<Pallet<T>, Identity, DrawdownId, OldDrawdownData<T>>;

  #[derive(Decode, Encode)]
  pub struct OldRevenueData<T: Config> {
    pub project_id: ProjectId,
    pub revenue_number: RevenueNumber,
    pub total_amount: RevenueAmount,
    pub status: RevenueStatus,
    pub status_changes: RevenueStatusChanges<T>,
    pub created_date: CreatedDate,
    pub closed_date: CloseDate,
  }

  // #[cfg(feature = "try-runtime")]
  #[storage_alias]
  pub(super) type RevenuesInfo<T: Config> =
    StorageMap<Pallet<T>, Identity, RevenueId, OldRevenueData<T>>;
}

pub mod v1 {
  pub use super::v0::OldDrawdownData;
  pub use super::v0::OldRevenueData;
  use super::*;

  impl<T: Config> OldDrawdownData<T> {
    fn migrate_to_v1_drawdown(self) -> DrawdownData<T> {
      DrawdownData {
        project_id: self.project_id,
        drawdown_number: self.drawdown_number,
        drawdown_type: self.drawdown_type,
        total_amount: self.total_amount,
        status: self.status,
        bulkupload_documents: self.bulkupload_documents,
        bank_documents: self.bank_documents,
        description: self.description,
        feedback: self.feedback,
        status_changes: self.status_changes,
        recovery_record: RecoveryRecord::<T>::default(),
        created_date: self.created_date,
        closed_date: self.closed_date,
      }
    }
  }

  impl<T: Config> OldRevenueData<T> {
    pub fn migrate_to_v1_revenue(self) -> RevenueData<T> {
      RevenueData {
        project_id: self.project_id,
        revenue_number: self.revenue_number,
        total_amount: self.total_amount,
        status: self.status,
        status_changes: self.status_changes,
        recovery_record: RecoveryRecord::<T>::default(),
        created_date: self.created_date,
        closed_date: self.closed_date,
      }
    }
  }

  pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
  impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
    #[allow(deprecated)]
    fn on_runtime_upgrade() -> Weight {
      let onchain_version = Pallet::<T>::on_chain_storage_version();
      let current_version = Pallet::<T>::current_storage_version();

      log::info!(
        target: LOG_TARGET,
        "Running migration with current storage version: {:?} / onchain version: {:?}",
        current_version,
        onchain_version
      );

      if onchain_version == 0 && current_version == 1 {
        // migrate to v1
        // Very inefficient, mostly here for illustration purposes.
        let count_drawdowns = v0::DrawdownsInfo::<T>::iter().count();
        let mut translated_drawdowns = 0u64;

        let count_revenues = v0::RevenuesInfo::<T>::iter().count();
        let mut translated_revenues = 0u64;

        DrawdownsInfo::<T>::translate::<OldDrawdownData<T>, _>(
          |_key: DrawdownId, value: OldDrawdownData<T>| {
            translated_drawdowns.saturating_inc();
            Some(value.migrate_to_v1_drawdown())
          },
        );

        RevenuesInfo::<T>::translate::<OldRevenueData<T>, _>(
          |_key: RevenueId, value: OldRevenueData<T>| {
            translated_revenues.saturating_inc();
            Some(value.migrate_to_v1_revenue())
          },
        );

        // Update storage version
        current_version.put::<Pallet<T>>();

        log::info!(
          target: LOG_TARGET,
          "Upgraded {} DrawdownData<T> from {} initial drawdowns, storage to version {:?}",
          count_drawdowns,
          translated_drawdowns,
          current_version
        );

        log::info!(
          target: LOG_TARGET,
          "Upgraded {} RevenueData<T> from {} initial revenues, storage to version {:?}",
          count_revenues,
          translated_revenues,
          current_version
        );

        T::DbWeight::get().reads_writes(translated_drawdowns + 1, translated_revenues + 1)
      } else {
        log::info!(
          target: LOG_TARGET,
          "Migration did not execute. This probably should be removed"
        );
        T::DbWeight::get().reads(1)
      }
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
      log::info!(
        target: LOG_TARGET,
        "pre_upgrade: current storage version {:?}",
        Pallet::<T>::current_storage_version()
      );

      ensure!(Pallet::<T>::on_chain_storage_version() == 0, "must upgrade linearly");
      ensure!(Pallet::<T>::current_storage_version() == 1, "migration from version 0 to 1");

      let prev_count_drawdowns = v0::DrawdownsInfo::<T>::iter().count();
      let keys_drawdowns = v0::DrawdownsInfo::<T>::iter_keys().count() as u32;
      let decodable_drawdowns = v0::DrawdownsInfo::<T>::iter_values().count() as u32;

      let prev_count_revenues = v0::RevenuesInfo::<T>::iter().count();
      let keys_revenues = v0::RevenuesInfo::<T>::iter_keys().count() as u32;
      let decodable_revenues = v0::RevenuesInfo::<T>::iter_values().count() as u32;

      log::info!(
        target: LOG_TARGET,
        "pre_upgrade: {:?} drawdowns, {:?} decodable drawdowns, {:?} total",
        keys_drawdowns,
        decodable_drawdowns,
        prev_count_drawdowns,
      );

      log::info!(
        target: LOG_TARGET,
        "pre_upgrade: {:?} revenues, {:?} decodable revenues, {:?} total",
        keys_revenues,
        decodable_revenues,
        prev_count_revenues,
      );

      ensure!(keys_drawdowns == decodable_drawdowns, "Not all drawdown values are decodable.");

      ensure!(keys_revenues == decodable_revenues, "Not all revenue values are decodable.");

      Ok(((prev_count_drawdowns as u32, prev_count_revenues as u32)).encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(prev_count: Vec<u8>) -> Result<(), &'static str> {
      // Split the encoded data into two u32s
      let (prev_count_drawdowns, prev_count_revenues) =
        <(u32, u32)>::decode(&mut &prev_count[..]).map_err(|_| "Unable to decode prev_count")?;

      let post_count_drawdowns = crate::DrawdownsInfo::<T>::iter().count() as u32;
      let post_count_revenues = crate::RevenuesInfo::<T>::iter().count() as u32;

      assert_eq!(
        prev_count_drawdowns, post_count_drawdowns,
        "the records count before and after the migration should be the same"
      );
      assert_eq!(
        prev_count_revenues, post_count_revenues,
        "the records count before and after the migration should be the same"
      );

      let current_version = Pallet::<T>::current_storage_version();
      let onchain_version = Pallet::<T>::on_chain_storage_version();

      ensure!(current_version == 1, "must upgrade to v1");
      assert_eq!(
        current_version, onchain_version,
        "after migration, the current_version and onchain_version should be the same"
      );

      crate::DrawdownsInfo::<T>::iter().for_each(|(_key, value)| {
        assert!(
          value.recovery_record == RecoveryRecord::<T>::default(),
          "recovery record should be default value"
        );
        assert!(value.recovery_record.len() == 0, "recovery record should be empty");
      });

      crate::RevenuesInfo::<T>::iter().for_each(|(_key, value)| {
        assert!(
          value.recovery_record == RecoveryRecord::<T>::default(),
          "recovery record should be default value"
        );
        assert!(value.recovery_record.len() == 0, "recovery record should be empty");
      });
      Ok(())
    }
  }
}
