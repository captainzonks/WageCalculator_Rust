#[derive(Clone, Copy)]
pub struct Data {
    pub(crate) wage: f32,
    pub(crate) days: f32,
    pub(crate) total_hours: f32,
    pub(crate) total_tips: f32,
    pub(crate) today_hours: f32,
    pub(crate) today_tips: f32,
}

impl Data {
    /// Setters ///

    pub(crate) fn set_wage(&mut self, in_wage: f32) {
        self.wage = in_wage;
    }

    pub(crate) fn set_days(&mut self, in_days: f32) {
        self.days = in_days;
    }

    pub(crate) fn set_today_hours(&mut self, in_hours: f32) {
        self.today_hours = in_hours;
    }

    pub(crate) fn set_today_tips(&mut self, in_tips: f32) {
        self.today_tips = in_tips;
    }

    pub(crate) fn set_total_tips_earned(&mut self, in_tips: f32) {
        self.total_tips = in_tips;
    }

    pub(crate) fn set_total_hours_worked(&mut self, in_hours: f32) {
        self.total_hours = in_hours;
    }

    pub(crate) fn add_hours(&mut self, in_hours: f32) {
        self.total_hours += in_hours;
        self.days += 1.0;
    }

    pub(crate) fn add_tips(&mut self, in_tips: f32) {
        self.total_tips += in_tips;
    }

    /// Getters ///

    pub(crate) fn get_wage(&self) -> f32 {
        self.wage
    }

    pub(crate) fn get_days(&self) -> f32 {
        self.days
    }

    pub(crate) fn get_total_hours(&self) -> f32 {
        self.total_hours
    }

    pub(crate) fn get_total_tips(&self) -> f32 {
        self.total_tips
    }

    pub(crate) fn get_today_hours(&self) -> f32 {
        self.today_hours
    }

    pub(crate) fn get_today_tips(&self) -> f32 {
        self.today_tips
    }

    ///////////////////////////////////

    fn get_average_hours(&self) -> f32 {
        if self.days != 0.0 {
            return self.total_hours / self.days;
        }
        0.0
    }

    pub(crate) fn get_average_tips(&self) -> f32 {
        if self.days != 0.0 {
            return self.total_tips / self.days;
        }
        0.0
    }

    pub(crate) fn get_today_average_wage(&self) -> f32 {
        if self.total_hours != 0.0 {
            return self.wage + (self.today_tips / self.today_hours);
        }
        0.0
    }

    pub(crate) fn get_overall_average_wage(&self) -> f32 {
        if self.total_hours != 0.0 {
            return self.wage + (self.total_tips / self.total_hours);
        }
        0.0
    }

    pub(crate) fn get_total_daily_earned(&self) -> f32 {
        self.wage * self.total_hours + self.total_tips
    }

    pub(crate) fn get_overall_average_wage_post_tax(&self) -> f32 {
        let federal_tax: f32 = 0.153;
        let co_state_tax: f32 = 0.0455;
        let fica_state_ins_tax: f32 = 0.0765;

        self.get_overall_average_wage() - (self.get_overall_average_wage() * federal_tax) -
            (self.get_overall_average_wage() * co_state_tax) -
            (self.get_overall_average_wage() * fica_state_ins_tax)
    }

    pub(crate) fn get_total_daily_earned_post_tax(&self) -> f32 {
        let federal_tax: f32 = 0.153;
        let co_state_tax: f32 = 0.0455;
        let fica_state_ins_tax: f32 = 0.0765;

        self.get_total_daily_earned() - (self.get_total_daily_earned() * federal_tax) -
            (self.get_total_daily_earned() * co_state_tax) -
            (self.get_total_daily_earned() * fica_state_ins_tax)
    }

    pub(crate) fn get_today_average_wage_post_tax(&self) -> f32 {
        let federal_tax: f32 = 0.153;
        let co_state_tax: f32 = 0.0455;
        let fica_state_ins_tax: f32 = 0.0765;

        self.get_today_average_wage() - (self.get_today_average_wage() * federal_tax) -
            (self.get_today_average_wage() * co_state_tax) -
            (self.get_today_average_wage() * fica_state_ins_tax)
    }
}