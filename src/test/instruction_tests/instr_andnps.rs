use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 195], OperandSize::Dword)
}

#[test]
fn andnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 344712330, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 44, 197, 138, 228, 139, 20], OperandSize::Dword)
}

#[test]
fn andnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 211], OperandSize::Qword)
}

#[test]
fn andnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1191948481, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 36, 141, 193, 176, 11, 71], OperandSize::Qword)
}

