use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 94, 224], OperandSize::Dword)
}

#[test]
fn vdivss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 998195713, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 94, 44, 149, 1, 66, 127, 59], OperandSize::Dword)
}

#[test]
fn vdivss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 94, 194], OperandSize::Qword)
}

#[test]
fn vdivss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 924353334, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 94, 20, 141, 54, 131, 24, 55], OperandSize::Qword)
}

#[test]
fn vdivss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 110, 255, 94, 232], OperandSize::Dword)
}

#[test]
fn vdivss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 430256023, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 102, 138, 94, 172, 207, 151, 47, 165, 25], OperandSize::Dword)
}

#[test]
fn vdivss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 86, 251, 94, 220], OperandSize::Qword)
}

#[test]
fn vdivss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 980187695, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 78, 139, 94, 172, 136, 47, 122, 108, 58], OperandSize::Qword)
}

