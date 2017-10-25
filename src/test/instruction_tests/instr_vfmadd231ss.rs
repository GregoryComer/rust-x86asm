use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 185, 218], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1004755045, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 185, 12, 93, 101, 88, 227, 59], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 185, 247], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 669021974, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 185, 148, 136, 22, 119, 224, 39], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 219, 185, 248], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 837852455, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 185, 156, 120, 39, 157, 240, 49], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 37, 219, 185, 210], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1153416586, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 21, 141, 185, 172, 202, 138, 189, 191, 68], OperandSize::Qword)
}

