use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aam_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AAM, operand1: Some(Literal8(35)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[212, 35], OperandSize::Word)
}

fn aam_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AAM, operand1: Some(Literal8(17)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[212, 17], OperandSize::Dword)
}

