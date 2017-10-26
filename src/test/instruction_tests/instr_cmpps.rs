use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 243, 48], OperandSize::Dword)
}

#[test]
fn cmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ESI, 1405311637, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 190, 149, 90, 195, 83, 48], OperandSize::Dword)
}

#[test]
fn cmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 198, 105], OperandSize::Qword)
}

#[test]
fn cmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 44, 70, 16], OperandSize::Qword)
}

