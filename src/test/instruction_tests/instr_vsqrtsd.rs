use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 246], OperandSize::Dword)
}

#[test]
fn vsqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 145493077, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 81, 148, 246, 85, 12, 172, 8], OperandSize::Dword)
}

#[test]
fn vsqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 81, 252], OperandSize::Qword)
}

#[test]
fn vsqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 81, 60, 131], OperandSize::Qword)
}

#[test]
fn vsqrtsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 207, 250, 81, 221], OperandSize::Dword)
}

#[test]
fn vsqrtsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 304093029, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 140, 81, 12, 149, 101, 23, 32, 18], OperandSize::Dword)
}

#[test]
fn vsqrtsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 239, 159, 81, 252], OperandSize::Qword)
}

#[test]
fn vsqrtsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1263797306, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 135, 141, 81, 188, 147, 58, 4, 84, 75], OperandSize::Qword)
}

