use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xabort_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XABORT, operand1: Some(Literal8(48)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[198, 248, 48], OperandSize::Dword)
}

fn xabort_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XABORT, operand1: Some(Literal8(1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[198, 248, 1], OperandSize::Qword)
}

