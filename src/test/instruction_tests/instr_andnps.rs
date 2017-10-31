use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 212], OperandSize::Dword)
}

#[test]
fn andnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 132872842, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 36, 245, 138, 122, 235, 7], OperandSize::Dword)
}

#[test]
fn andnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 195], OperandSize::Qword)
}

#[test]
fn andnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 12, 90], OperandSize::Qword)
}

