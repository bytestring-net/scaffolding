use std::{fmt::Display, ops::{Deref, DerefMut}, str::FromStr, time::SystemTime};

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{NaiveDate as ChronoDate, NaiveTime as ChronoTime, DateTime as ChronoDateTime, TimeZone};

pub use chrono::{Datelike, Timelike, Utc as UtcTime, Local as LocalTime};



#[derive(Error, Debug)]
pub enum TryFromTimeError {
    ///Tried to create struct from number out of bounds
    #[error("Tried to create struct from number out of bounds")]
    OutOfBounds,
}


#[test]
fn test() {
    println!("Utc: {}", DateTime::utc_now());

    println!("Time: {}", Time::local_now());
    println!("Date: {}", Date::local_now());

    println!("{}", "2024-10-19".parse::<Date>().unwrap());
    println!("{}", "16:45:35".parse::<Time>().unwrap());
}


// #==================#
// #=== DATE TYPES ===#

/// Type representing a year. Can be used in for serializing dates.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Year(i32);
impl Year {
    /// Returns the current UTC year
    pub fn utc_now() -> Self {
        Self(UtcTime::now().year())
    }
    /// Returns the current local year
    pub fn local_now() -> Self {
        Self(LocalTime::now().year())
    }
    /// Check if the year is a leap year
    pub fn is_leap(&self) -> bool {
        self.0%4 == 0 && self.0%100 != 0
    }
}
impl From<Year> for i32 {
    fn from(val: Year) -> Self {
        val.0
    }
}
impl From<i32> for Year {
    fn from(value: i32) -> Self {
        Year(value)
    }
}
impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<i32> for Year {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}
impl AsMut<i32> for Year {
    fn as_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Type representing a month. Can be used in for serializing dates.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Month(u32);
impl Month {
    /// Returns the current UTC month
    pub fn utc_now() -> Self {
        Self(UtcTime::now().month())
    }
    /// Returns the current local month
    pub fn local_now() -> Self {
        Self(LocalTime::now().month())
    }
    /// This function will normalize all values into the 1..=12 range using wrapping overflow
    pub fn normalize(&self) -> Self {
        Self(((self.0 - 1) % 12 + 12) % 12 + 1)
    }
    /// This function will return how many days are in a month (normalized)
    pub fn count_days(&self, leap_year: bool) -> u32 {
        match self.normalize().0 {
            1 => 31,
            2 => if leap_year {29} else {28},
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => unreachable!(),
        }
    }
    /// This function will return english name of the month (normalized)
    pub fn get_name(&self) -> String {
        match self.normalize().0 {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => unreachable!(),
        }
    }
    /// This function will check if the month falls with the range 1..=12
    pub fn is_valid(&self) -> bool {
        matches!(self.0, 1..=12)
    }
}
impl From<Month> for u32 {
    fn from(val: Month) -> Self {
        val.0
    }
}
impl From<u32> for Month {
    fn from(value: u32) -> Self {
        Month(value)
    }
}
impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<u32> for Month {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsMut<u32> for Month {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


/// Type representing a day. Can be used in for serializing dates.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Day(u32);
impl Day {
    /// Returns the current UTC day
    pub fn utc_now() -> Self {
        Self(UtcTime::now().day())
    }
    /// Returns the current local day
    pub fn local_now() -> Self {
        Self(LocalTime::now().day())
    }
    /// This function will check if the day falls with the range of the month (month argument is normalized)
    pub fn is_valid(&self, month: impl Into<Month>, leap_year: bool) -> bool {
        let mm: Month = month.into();
        0 < self.0 && self.0 <= mm.count_days(leap_year)
    }
}
impl From<Day> for u32 {
    fn from(val: Day) -> Self {
        val.0
    }
}
impl From<u32> for Day {
    fn from(value: u32) -> Self {
        Day(value)
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<u32> for Day {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsMut<u32> for Day {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


/// Type representing a date. Can be used in for serializing dates.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Date(ChronoDate);
impl Date {
    /// Returns the current UTC date
    pub fn utc_now() -> Self {
        Date(UtcTime::now().date_naive())
    }
    /// Returns the current local date
    pub fn local_now() -> Self {
        Date(LocalTime::now().date_naive())
    }
}
impl FromStr for Date {
    type Err = TryFromTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date(s.parse::<ChronoDate>().map_err(|_| TryFromTimeError::OutOfBounds)?))
    }
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl <Y, M, D> TryFrom<(Y, M, D)> for Date where Y: Into<Year>, M: Into<Month>, D: Into<Day> {
    type Error = TryFromTimeError;
    fn try_from(value: (Y, M, D)) -> Result<Self, Self::Error> {
        Ok(Date(ChronoDate::from_ymd_opt(value.0.into().0, value.1.into().0, value.2.into().0).ok_or(TryFromTimeError::OutOfBounds)?))
    }
}

impl From<Date> for ChronoDate {
    fn from(value: Date) -> Self {
        value.0
    }
}
impl Deref for Date {
    type Target = ChronoDate;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Date {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// #==================#
// #=== TIME TYPES ===#

/// Type representing an hour. Can be used in for serializing time.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Hour(u32);
impl Hour {
    /// Returns the current UTC hour
    pub fn utc_now() -> Self {
        Hour(UtcTime::now().hour())
    }
    /// Returns the current local hour
    pub fn local_now() -> Self {
        Hour(LocalTime::now().hour())
    }
}
impl From<Hour> for u32 {
    fn from(val: Hour) -> Self {
        val.0
    }
}
impl From<u32> for Hour {
    fn from(value: u32) -> Self {
        Hour(value)
    }
}
impl Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
impl AsRef<u32> for Hour {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsMut<u32> for Hour {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


/// Type representing a minute. Can be used in for serializing time.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Minute(u32);
impl Minute {
    /// Returns the current UTC minute
    pub fn utc_now() -> Self {
        Minute(UtcTime::now().minute())
    }
    /// Returns the current local minute
    pub fn local_now() -> Self {
        Minute(LocalTime::now().minute())
    }
}
impl From<Minute> for u32 {
    fn from(val: Minute) -> Self {
        val.0
    }
}
impl From<u32> for Minute {
    fn from(value: u32) -> Self {
        Minute(value)
    }
}
impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
impl AsRef<u32> for Minute {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsMut<u32> for Minute {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


/// Type representing a second. Can be used in for serializing time.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize, Decode, Encode)]
pub struct Second(u32);
impl Second {
    /// Returns the current UTC second
    pub fn utc_now() -> Self {
        Second(UtcTime::now().second())
    }
    /// Returns the current local second
    pub fn local_now() -> Self {
        Second(LocalTime::now().second())
    }
}
impl From<Second> for u32 {
    fn from(val: Second) -> Self {
        val.0
    }
}
impl From<u32> for Second {
    fn from(value: u32) -> Self {
        Second(value)
    }
}
impl Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
impl AsRef<u32> for Second {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl AsMut<u32> for Second {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}


/// Type representing a time. Can be used in for serializing dates.
/// This type is guaranteed to be valid, otherwise cannot be initialized.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Time(ChronoTime);
impl Time {
    /// Returns the current UTC time
    pub fn utc_now() -> Self {
        Time(UtcTime::now().time())
    }
    /// Returns the current local time
    pub fn local_now() -> Self {
        Time(LocalTime::now().time())
    }
}
impl FromStr for Time {
    type Err = TryFromTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time(s.parse::<ChronoTime>().map_err(|_| TryFromTimeError::OutOfBounds)?))
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl <H, M, S> TryFrom<(H, M, S)> for Time where H: Into<Hour>, M: Into<Minute>, S: Into<Second> {
    type Error = TryFromTimeError;
    fn try_from(value: (H, M, S)) -> Result<Self, Self::Error> {
        Ok(Time(ChronoTime::from_hms_opt(value.0.into().0, value.1.into().0, value.2.into().0).ok_or(TryFromTimeError::OutOfBounds)?))
    }
}

impl From<Time> for ChronoTime {
    fn from(value: Time) -> Self {
        value.0
    }
}
impl Deref for Time {
    type Target = ChronoTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


// #=========================#
// #=== DATE & TIME TYPES ===#


/// Type representing a datetime. Can be used in for serializing dates.
/// This type is guaranteed to be valid, otherwise cannot be initialized.
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
//#[cfg_attr(feature = "skytable", derive(Query, Response))]
pub struct DateTime(ChronoDateTime<UtcTime>);
impl DateTime {
    /// Returns the current UTC time
    pub fn utc_now() -> Self {
        DateTime(UtcTime::now())
    }
}
impl FromStr for DateTime {
    type Err = TryFromTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime(s.parse::<ChronoDateTime<UtcTime>>().map_err(|_| TryFromTimeError::OutOfBounds)?))
    }
}
impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl <D, T> From<(D, T)> for DateTime where D: Into<Date>, T: Into<Time> {
    fn from(value: (D, T)) -> Self {
        let date: Date = value.0.into();
        let time: Time = value.1.into();
        DateTime(date.and_time(*time).and_utc())
    }
}
impl <Y, M, D, H, MM, S> TryFrom<(Y, M, D, H, MM, S)> for DateTime where Y: Into<Year>, M: Into<Month>, D: Into<Day>, H: Into<Hour>, MM: Into<Minute>, S: Into<Second> {
    type Error = TryFromTimeError;
    fn try_from(value: (Y, M, D, H, MM, S)) -> Result<Self, Self::Error> {
        Ok(DateTime(UtcTime.with_ymd_and_hms(value.0.into().0, value.1.into().0, value.2.into().0, value.3.into().0, value.4.into().0, value.5.into().0).single().ok_or(TryFromTimeError::OutOfBounds)?))
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        Self(ChronoDateTime::<UtcTime>::from(value))
    }
}

impl From<DateTime> for ChronoDateTime<UtcTime> {
    fn from(value: DateTime) -> Self {
        value.0
    }
}
impl Deref for DateTime {
    type Target = ChronoDateTime<UtcTime>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
