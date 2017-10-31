use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 208], OperandSize::Dword)
}

#[test]
fn andnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDI, 1323963327, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 143, 191, 19, 234, 78], OperandSize::Dword)
}

#[test]
fn andnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 221], OperandSize::Qword)
}

#[test]
fn andnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 12, 199], OperandSize::Qword)
}

