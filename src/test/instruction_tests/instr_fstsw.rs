use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 59], OperandSize::Word)
}

#[test]
fn fstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 859364155, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 60, 197, 59, 219, 56, 51], OperandSize::Dword)
}

#[test]
fn fstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 63], OperandSize::Qword)
}

