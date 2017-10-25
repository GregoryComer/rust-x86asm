use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 169, 218], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1351933670, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 169, 164, 182, 230, 222, 148, 80], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 169, 251], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 169, 9], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 169, 198], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 191646245, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 140, 169, 188, 208, 37, 74, 108, 11], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 189, 191, 169, 232], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 173, 129, 169, 52, 143], OperandSize::Qword)
}

