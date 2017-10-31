use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 169, 240], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 169, 44, 216], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 169, 195], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 169, 43], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 155, 169, 246], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 169, 19], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 205, 211, 169, 237], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1049103660, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 229, 143, 169, 140, 138, 44, 13, 136, 62], OperandSize::Qword)
}

