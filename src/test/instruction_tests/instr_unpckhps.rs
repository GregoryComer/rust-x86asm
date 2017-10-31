use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 203], OperandSize::Dword)
}

#[test]
fn unpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 997086756, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 182, 36, 86, 110, 59], OperandSize::Dword)
}

#[test]
fn unpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 199], OperandSize::Qword)
}

#[test]
fn unpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 1144802475, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 151, 171, 76, 60, 68], OperandSize::Qword)
}

