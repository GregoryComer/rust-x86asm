use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 218, 97], OperandSize::Dword)
}

#[test]
fn cmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 4, 247, 13], OperandSize::Dword)
}

#[test]
fn cmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 252, 114], OperandSize::Qword)
}

#[test]
fn cmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 2099927785, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 132, 86, 233, 90, 42, 125, 35], OperandSize::Qword)
}

