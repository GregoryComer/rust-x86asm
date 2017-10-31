use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 169, 222], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 648795046, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 169, 36, 133, 166, 211, 171, 38], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 169, 238], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1406042430, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 169, 135, 62, 129, 206, 83], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 250, 169, 249], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 971763278, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 169, 188, 121, 78, 238, 235, 57], OperandSize::Dword)
}

#[test]
fn vfmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 149, 209, 169, 244], OperandSize::Qword)
}

#[test]
fn vfmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 141, 169, 12, 179], OperandSize::Qword)
}

