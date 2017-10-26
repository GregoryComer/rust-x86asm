use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 228], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 485616212, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 191, 4, 77, 84, 234, 241, 28], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 191, 198], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 692263392, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 191, 156, 192, 224, 25, 67, 41], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 186, 191, 236], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 913692588, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 191, 60, 221, 172, 215, 117, 54], OperandSize::Dword)
}

#[test]
fn vfnmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 61, 223, 191, 246], OperandSize::Qword)
}

#[test]
fn vfnmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 109, 139, 191, 28, 191], OperandSize::Qword)
}

