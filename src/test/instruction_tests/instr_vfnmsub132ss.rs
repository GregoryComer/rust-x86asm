use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 159, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 2009664806, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 159, 168, 38, 13, 201, 119], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 159, 232], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 763479235, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 159, 140, 126, 195, 196, 129, 45], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 222, 159, 239], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1417333692, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 159, 172, 150, 188, 203, 122, 84], OperandSize::Dword)
}

#[test]
fn vfnmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 21, 222, 159, 249], OperandSize::Qword)
}

#[test]
fn vfnmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 681224414, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 129, 159, 44, 149, 222, 168, 154, 40], OperandSize::Qword)
}

