use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 94, 238], OperandSize::Dword)
}

#[test]
fn vdivss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1177598310, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 94, 140, 128, 102, 185, 48, 70], OperandSize::Dword)
}

#[test]
fn vdivss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 94, 194], OperandSize::Qword)
}

#[test]
fn vdivss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 1413221805, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 94, 132, 249, 173, 13, 60, 84], OperandSize::Qword)
}

#[test]
fn vdivss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 110, 159, 94, 247], OperandSize::Dword)
}

#[test]
fn vdivss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 868340362, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 70, 140, 94, 150, 138, 210, 193, 51], OperandSize::Dword)
}

#[test]
fn vdivss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 14, 212, 94, 219], OperandSize::Qword)
}

#[test]
fn vdivss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 110, 139, 94, 48], OperandSize::Qword)
}

