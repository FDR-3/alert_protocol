use anchor_lang::prelude::*;
use core::mem::size_of;
use solana_security_txt::security_txt;

declare_id!("8a9Uup7MfuERSx9iLr99uS2WQQJ9QidTvc7QaVH9nwH3");

#[cfg(not(feature = "no-entrypoint"))] //Ensure it's not included when compiled as a library
security_txt!
{
    name: "Alert Protocol",
    project_url: "https://m4a.io",
    contacts: "email fdr3@m4a.io",
    preferred_languages: "en",
    source_code: "https://github.com/FDR-3/alert_protocol",
    policy: "If you find a bug, email me and say something please D:"
}

#[cfg(feature = "dev")] 
const INITIAL_CEO_ADDRESS: Pubkey = pubkey!("Fdqu1muWocA5ms8VmTrUxRxxmSattrmpNraQ7RpPvzZg");

#[cfg(feature = "local")] 
const INITIAL_CEO_ADDRESS: Pubkey = pubkey!("DSLn1ofuSWLbakQWhPUenSBHegwkBBTUwx8ZY4Wfoxm");

//PSA Alert needs atleast 424 extra bytes of space to pass with full load
const SITE_PSA_ALERT_EXTRA_SIZE: usize = 430;

const MAX_PSA_LENGTH: usize = 444;

//Error Codes
#[error_code]
pub enum AuthorizationError 
{
    #[msg("Only the CEO can call this function")]
    NotCEO
}

#[error_code]
pub enum InvalidOperationError 
{
    #[msg("Can't set flag to the same state")]
    FlagSameState
}

#[error_code]
pub enum InvalidLengthError 
{
    #[msg("Message can't be longer than 444 characters")]
    MSGTooLong
} 

#[program]
pub mod alert_protocol
{
    use super::*;

    pub fn initialize_alert_protocol(ctx: Context<InitializeAlertProtocol>) -> Result<()>
    {
        //Only the initial CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), INITIAL_CEO_ADDRESS, AuthorizationError::NotCEO);

        let ceo = &mut ctx.accounts.ceo;
        ceo.address = INITIAL_CEO_ADDRESS;

        msg!("Alert Protocol Initialized");
        msg!("New CEO Address: {}", ceo.address.key());

        Ok(())
    }

    pub fn pass_on_alert_protocol_ceo(ctx: Context<PassOnAlertProtocolCEO>, new_ceo_address: Pubkey) -> Result<()> 
    {
        let ceo = &mut ctx.accounts.ceo;
        //Only the CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), ceo.address.key(), AuthorizationError::NotCEO);

        msg!("The Alert Protocol CEO has passed on the title to a new CEO");
        msg!("New CEO: {}", new_ceo_address.key());

        ceo.address = new_ceo_address.key();

        Ok(())
    }

    pub fn clock_in_dead_mans_break(ctx: Context<ClockInDeadMansBreak>) -> Result<()> 
    {
        let ceo = & ctx.accounts.ceo;
        //Only the CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), ceo.address.key(), AuthorizationError::NotCEO);

        let dead_mans_break_alert = &mut ctx.accounts.dead_mans_break_alert;
        dead_mans_break_alert.unix_clock_in_time_stamp = Clock::get()?.unix_timestamp as u64;

        msg!("Dead Mans Break Refreshed");

        Ok(())
    }

    pub fn trigger_new_ui_available_alert(ctx: Context<TriggerNewUIAvailableAlert>) -> Result<()> 
    {
        let ceo = & ctx.accounts.ceo;
        //Only the CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), ceo.address.key(), AuthorizationError::NotCEO);

        let site_update_alert = &mut ctx.accounts.site_update_alert;
        site_update_alert.site_update_count += 1;

        msg!("Site Update Alert Triggered: #{}", site_update_alert.site_update_count);

        Ok(())
    }

    pub fn trigger_new_psa_alert(ctx: Context<TriggerNewPSAAlert>, psa_msg: String) -> Result<()> 
    {
        let ceo = & ctx.accounts.ceo;
        //Only the CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), ceo.address.key(), AuthorizationError::NotCEO);

        //Message string must not be longer than 444 characters
        require!(psa_msg.len() <= MAX_PSA_LENGTH, InvalidLengthError::MSGTooLong);

        let site_psa_alert = &mut ctx.accounts.site_psa_alert;
        site_psa_alert.site_psa_msg = psa_msg.clone();

        msg!("Site PSA Message Updated To :{}", psa_msg);

        Ok(())
    }

    pub fn toggle_sos_alert(ctx: Context<ToggleSOSAlert>, is_enabled: bool) -> Result<()> 
    {
        let ceo = & ctx.accounts.ceo;
        //Only the CEO can call this function
        require_keys_eq!(ctx.accounts.signer.key(), ceo.address.key(), AuthorizationError::NotCEO);

        let site_sos_alert = &mut ctx.accounts.site_sos_alert;
        //Can't set flag to the same state
        require!(site_sos_alert.sos_flag != is_enabled, InvalidOperationError::FlagSameState);

        site_sos_alert.sos_flag = is_enabled;

        msg!("SOS Flag Updated To: {}", is_enabled);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct InitializeAlertProtocol<'info> 
{
    #[account(
        init, 
        payer = signer,
        seeds = [b"alertProtocolCEO".as_ref()],
        bump,
        space = size_of::<AlertProtocolCEO>() + 8)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"deadMansBreakAlert".as_ref()],
        bump,
        space = size_of::<DeadMansBreakAlert>() + 8)]
    pub dead_mans_break_alert: Account<'info, DeadMansBreakAlert>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"siteUpdateAlert".as_ref()],
        bump,
        space = size_of::<SiteUpdateAlert>() + 8)]
    pub site_update_alert: Account<'info, SiteUpdateAlert>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"sitePSAAlert".as_ref()],
        bump,
        space = size_of::<SitePSAAlert>() + SITE_PSA_ALERT_EXTRA_SIZE + 8)]
    pub site_psa_alert: Account<'info, SitePSAAlert>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"siteSOSAlert".as_ref()],
        bump,
        space = size_of::<SiteSOSAlert>() + 8)]
    pub site_sos_alert: Account<'info, SiteSOSAlert>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct PassOnAlertProtocolCEO<'info> 
{
    #[account(
        mut,
        seeds = [b"alertProtocolCEO".as_ref()],
        bump)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ClockInDeadMansBreak<'info> 
{
    #[account(
        seeds = [b"alertProtocolCEO".as_ref()],
        bump)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(
        mut,
        seeds = [b"deadMansBreakAlert".as_ref()],
        bump)]
    pub dead_mans_break_alert: Account<'info, DeadMansBreakAlert>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct TriggerNewUIAvailableAlert<'info> 
{
    #[account(
        seeds = [b"alertProtocolCEO".as_ref()],
        bump)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(
        mut,
        seeds = [b"siteUpdateAlert".as_ref()],
        bump)]
    pub site_update_alert: Account<'info, SiteUpdateAlert>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct TriggerNewPSAAlert<'info> 
{
    #[account(
        seeds = [b"alertProtocolCEO".as_ref()],
        bump)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(
        mut,
        seeds = [b"sitePSAAlert".as_ref()],
        bump)]
    pub site_psa_alert: Account<'info, SitePSAAlert>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ToggleSOSAlert<'info> 
{
    #[account(
        seeds = [b"alertProtocolCEO".as_ref()],
        bump)]
    pub ceo: Account<'info, AlertProtocolCEO>,

    #[account(
        mut,
        seeds = [b"siteSOSAlert".as_ref()],
        bump)]
    pub site_sos_alert: Account<'info, SiteSOSAlert>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

//Accounts
#[account]
pub struct AlertProtocolCEO
{
    pub address: Pubkey
}

#[account]
pub struct DeadMansBreakAlert
{
    pub unix_clock_in_time_stamp: u64
}

#[account]
pub struct SiteUpdateAlert
{
    pub site_update_count: u64
}

#[account]
pub struct SitePSAAlert
{
    pub site_psa_msg: String
}

#[account]
pub struct SiteSOSAlert
{
    pub sos_flag: bool
}