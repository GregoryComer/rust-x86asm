use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 198], OperandSize::Dword)
}

#[test]
fn vsqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 569955663, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 81, 140, 218, 79, 213, 248, 33], OperandSize::Dword)
}

#[test]
fn vsqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 81, 222], OperandSize::Qword)
}

#[test]
fn vsqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 81, 44, 90], OperandSize::Qword)
}

#[test]
fn vsqrtsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 199, 156, 81, 217], OperandSize::Dword)
}

#[test]
fn vsqrtsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 239, 139, 81, 35], OperandSize::Dword)
}

#[test]
fn vsqrtsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 175, 212, 81, 213], OperandSize::Qword)
}

#[test]
fn vsqrtsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1719122460, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 151, 132, 81, 180, 131, 28, 186, 119, 102], OperandSize::Qword)
}

